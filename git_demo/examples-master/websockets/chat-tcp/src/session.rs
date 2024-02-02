//! `ClientSession` is an actor, it manages peer tcp connection and
//! proxies commands from peer to `ChatServer`.

use std::{
    io, net,
    str::FromStr,
    time::{Duration, Instant},
};

use actix::{prelude::*, spawn};
use tokio::{
    io::{split, WriteHalf},
    net::{TcpListener, TcpStream},
};
use tokio_util::codec::FramedRead;

use crate::{
    codec::{ChatCodec, ChatRequest, ChatResponse},
    server::{self, ChatServer},
};

/// Chat server sends this messages to session
/// 聊天服务器将此消息发送到会话
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// `ChatSession` actor is responsible for tcp peer communications.
/// ChatSession的actor负责tcp对等通信。
pub struct ChatSession {
    /// unique session id
    /// 唯一会话id
    id: usize,
    /// this is address of chat server
    /// 这是聊天服务器的地址
    addr: Addr<ChatServer>,
    /// Client must send ping at least once per 10 seconds, otherwise we drop
    /// connection.
    /// 客户端必须至少每10秒发送一次ping，否则我们将断开连接。
    hb: Instant,
    /// joined room
    /// 联合会议室
    room: String,
    /// Framed wrapper
    /// 框架包装
    framed: actix::io::FramedWrite<ChatResponse, WriteHalf<TcpStream>, ChatCodec>,
}

impl Actor for ChatSession {
    /// For tcp communication we are going to use `FramedContext`.
    /// It is convenient wrapper around `Framed` object from `tokio_io`
    /// 对于tcp通信，我们将使用“FramedContext”。它是对“tokio_io”中的“Framed”对象的方便包装`
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        // 我们将在会话启动时启动心跳进程。
        self.hb(ctx);
        println!("===========");
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        //在聊天服务器中注册自己`AsyncContext:：wait`在上下文中注册future，但上下文要等到该future解析后才能处理任何其他事件。
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    // 聊天服务器出现问题
                    _ => ctx.stop(),
                }
                actix::fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        // 通知聊天服务器
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl actix::io::WriteHandler<io::Error> for ChatSession {}

/// To use `Framed` we have to define Io type and Codec
/// 要使用“Framed”，我们必须定义Io类型和编解码器
impl StreamHandler<Result<ChatRequest, io::Error>> for ChatSession {
    /// This is main event loop for client requests
    /// 这是客户端请求的主要事件循环
    fn handle(&mut self, msg: Result<ChatRequest, io::Error>, ctx: &mut Context<Self>) {
        match msg {
            Ok(ChatRequest::List) => {
                // Send ListRooms message to chat server and wait for response
                // 向聊天服务器发送ListRooms消息并等待响应
                println!("List rooms");
                self.addr
                    .send(server::ListRooms)
                    .into_actor(self)
                    .then(|res, act, _| {
                        match res {
                            Ok(rooms) => {
                                act.framed.write(ChatResponse::Rooms(rooms));
                            }
                            _ => println!("Something is wrong"),
                        }
                        actix::fut::ready(())
                    })
                    .wait(ctx)
                // .wait(ctx) pauses all events in context,
                // so actor wont receive any new messages until it get list of rooms back
            }
            Ok(ChatRequest::Join(name)) => {
                println!("Join to room: {name}");
                self.room = name.clone();
                self.addr.do_send(server::Join {
                    id: self.id,
                    name: name.clone(),
                });
                self.framed.write(ChatResponse::Joined(name));
            }
            Ok(ChatRequest::Message(message)) => {
                // send message to chat server
                // 向聊天服务器发送消息
                println!("Peer message: {message}");
                self.addr.do_send(server::Message {
                    id: self.id,
                    msg: message,
                    room: self.room.clone(),
                })
            }
            // we update heartbeat time on ping from peer
            // 我们从对等端更新ping上的心跳时间
            Ok(ChatRequest::Ping) => self.hb = Instant::now(),
            _ => ctx.stop(),
        }
    }
}

/// Handler for Message, chat server sends this message, we just send string to
/// peer
/// 消息处理程序，聊天服务器发送此消息，我们只向对等方发送字符串
impl Handler<Message> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Context<Self>) {
        // send message to peer
        // 向对等方发送消息
        println!("msg: {:?}", msg.0);
        self.framed.write(ChatResponse::Message(msg.0));
    }
}

/// Helper methods
impl ChatSession {
    pub fn new(
        addr: Addr<ChatServer>,
        framed: actix::io::FramedWrite<ChatResponse, WriteHalf<TcpStream>, ChatCodec>,
    ) -> ChatSession {
        ChatSession {
            id: 0,
            addr,
            hb: Instant::now(),
            room: "main".to_owned(),
            framed,
        }
    }

    /// helper method that sends ping to client every second.
    /// 每秒向客户端发送ping的helper方法。
    ///
    /// also this method check heartbeats from client
    /// 该方法还检查客户端的心跳
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_interval(Duration::new(1, 0), |act, ctx| {
            // check client heartbeats
            // 检查客户端心跳
            if Instant::now().duration_since(act.hb) > Duration::new(10, 0) {
                // heartbeat timed out
                // 心跳超时
                println!("Client heartbeat failed, disconnecting!");

                // notify chat server
                // 通知聊天服务器
                act.addr.do_send(server::Disconnect { id: act.id });

                // stop actor
                // 停止演员
                ctx.stop();
            }

            act.framed.write(ChatResponse::Ping);
            // if we can not send message to sink, sink is closed (disconnected)
            // 如果我们不能向接收器发送消息，则接收器已关闭（断开连接）
        });
    }
}

/// Define TCP server that will accept incoming TCP connection and create
/// chat actors.
/// 定义将接受传入TCP连接并创建聊天参与者的TCP服务器。
pub fn tcp_server(_s: &str, server: Addr<ChatServer>) {
    // Create server listener
    // 创建服务器侦听器
    let addr = net::SocketAddr::from_str("127.0.0.1:12345").unwrap();

    spawn(async move {
        let listener = TcpListener::bind(&addr).await.unwrap();

        while let Ok((stream, addr)) = listener.accept().await {
            println!("客户连接: {}", addr);
            let server = server.clone();
            ChatSession::create(|ctx| {
                let (r, w) = split(stream);
                ChatSession::add_stream(FramedRead::new(r, ChatCodec), ctx);
                ChatSession::new(server, actix::io::FramedWrite::new(w, ChatCodec, ctx))
            });
        }
    });
}
