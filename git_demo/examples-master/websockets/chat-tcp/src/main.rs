use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_files::NamedFile;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpServer, Responder};
use actix_web_actors::ws;

mod codec;
mod server;
mod session;

/// How often heartbeat pings are sent
// / 发送心跳信号的频率
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
/// 缺少客户端响应导致超时的时间
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

async fn index() -> impl Responder {
    //尝试以只读模式异步打开文件。
    println!("打开文件...");
    NamedFile::open_async("./static/index.html").await.unwrap()
}

/// Entry point for our route
/// 我们路线的入口
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<impl Responder, Error> {
    ws::start(//执行WebSocket握手并启动actor。
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

//会话
struct WsChatSession {
    /// unique session id
    /// 唯一会话id
    id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    /// 客户端必须至少每10秒发送一次ping（Client_TIMEOUT），否则我们将断开连接。
    hb: Instant,
    /// joined room
    room: String,
    /// peer name
    /// 对等方名称
    name: Option<String>,
    /// Chat server
    /// 聊天服务器
    addr: Addr<server::ChatServer>,
}


//客户连接入口
impl Actor for WsChatSession {
    // WebSockets参与者的执行上下文
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    /// 方法是在actor启动时调用的。我们向ChatServer注册ws会话
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        // 我们将在会话开始时启动心跳过程。
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // 在聊天服务器中注册自己`AsyncContext:：wait`在上下文中注册future，但上下文要等到该future解析后才能处理任何其他事件。
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        // HttpContext:：state（）是WsChatSessionState的实例，状态在应用程序内的所有路由上共享
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
                fut::ready(())
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

/// 发送消息入口
/// Handle messages from chat server, we simply send it to peer websocket
/// 处理来自聊天服务器的消息，我们只需将其发送到对等网络套接字
/// 描述如何处理特定类型的消息。
/// 实现Handler是处理传入消息、流和期货的通用方法。
/// 类型M是可以由参与者处理的消息。
impl Handler<session::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: session::Message, ctx: &mut Self::Context) {
        // 发送文本
        ctx.text(msg.0);
    }
}



/// 客户请求处理入口
/// WebSocket message handler
/// WebSocket消息处理程序
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        log::debug!("WEBSOCKET MESSAGE: {msg:?}");
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                // we check for /sss type of messages
                // 我们检查/sss类型的消息
                // 如果给定的模式与此字符串切片的前缀匹配，则返回true。
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/list" => {
                            // Send ListRooms message to chat server and wait for
                            // response
                            //向聊天服务器发送ListRooms消息并等待响应
                            println!("List rooms");
                            self.addr
                                .send(server::ListRooms)
                                .into_actor(self)
                                .then(|res, _, ctx| {
                                    match res {
                                        Ok(rooms) => {
                                            for room in rooms {
                                                ctx.text(room);
                                            }
                                        }
                                        _ => println!("Something is wrong"),
                                    }
                                    fut::ready(())
                                })
                                .wait(ctx)
                            // .wait(ctx) pauses all events in context,
                            // so actor wont receive any new messages until it get list
                            // of rooms back
                        }
                        "/join" => {
                            if v.len() == 2 {
                                self.room = v[1].to_owned();
                                self.addr.do_send(server::Join {
                                    id: self.id,
                                    name: self.room.clone(),
                                });

                                ctx.text("joined");
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }
                        "/name" => {
                            if v.len() == 2 {
                                self.name = Some(v[1].to_owned());
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        _ => ctx.text(format!("!!! unknown command: {m:?}")),
                    }
                } else {
                    let msg = if let Some(ref name) = self.name {
                        format!("{name}: {m}")
                    } else {
                        m.to_owned()
                    };
                    // send message to chat server
                    // 向聊天服务器发送消息
                    self.addr.do_send(server::Message {
                        id: self.id,
                        msg,
                        room: self.room.clone(),
                    })
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl WsChatSession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    /// helper方法，每5秒向客户端发送一次ping（HEARTBEAT_INTERVAL）。
    ///
    /// also this method checks heartbeats from client
    /// 该方法还检查客户端的心跳
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        // 生成一个作业，以指定的固定间隔定期执行给定的闭包。
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            // 检查客户端心跳
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                // 心跳超时
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                // 通知聊天服务器
                act.addr.do_send(server::Disconnect { id: act.id });

                // stop actor
                // 停止演员
                ctx.stop();

                // don't try to send a ping
                // 不要尝试发送ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // start chat server actor
    // 启动聊天服务器actor
    let server = server::ChatServer::default().start();

    // start TCP server in separate thread
    // 在单独的线程中启动TCP服务器
    let srv = server.clone();
    session::tcp_server("127.0.0.1:12345", srv);

    log::info!("starting HTTP+WebSocket server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            // WebSocket UI HTML file
            .service(web::resource("/").to(index))
            // websocket
            .service(web::resource("/ws").to(chat_route))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
