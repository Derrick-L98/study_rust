//! `ChatServer` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatServer`.

use std::collections::{HashMap, HashSet};

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};

use crate::session;

/// Message for chat server communications
/// 聊天服务器通信消息

/// New chat session is created
/// 创建了新的聊天会话
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<session::Message>,
}

/// Session is disconnected
/// 会话已断开连接
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Send message to specific room
/// 将消息发送到特定房间
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message {
    /// Id of the client session
    /// 客户端会话的Id
    pub id: usize,
    /// Peer message
    /// 对等消息
    pub msg: String,
    /// Room name
    pub room: String,
}

/// List of available rooms
/// 可用房间列表
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

/// Join room, if room does not exists create new one.
/// 加入房间，如果房间不存在，创建一个新房间。
#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client id
    /// 客户端ID
    pub id: usize,
    /// Room name
    pub name: String,
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
/// `ChatServer`管理聊天室并负责协调聊天会话。实现是超基元的
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<session::Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,//对线程本地生成器的引用
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        // default room
        // 默认房间
        let mut rooms = HashMap::new();
        rooms.insert("main".to_owned(), HashSet::new());

        ChatServer {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
        }
    }
}

impl ChatServer {
    /// Send message to all users in the room
    /// 向会议室中的所有用户发送消息
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {//从房间获取所有用户
            for id in sessions {
                if *id != skip_id {//除了发消息的人,都通知
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(session::Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    /// 我们将使用简单的上下文，我们只需要沟通的能力
    /// 与其他演员合作。
    type Context = Context<Self>;
}

/// Handler for Connect message.
/// 连接消息的处理程序。
///
/// Register new session and assign unique id to this session
/// 注册新会话并为此会话分配唯一id
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // notify all users in same room
        // 通知同一房间中的所有用户
        self.send_message("main", "Someone joined", 0);

        // register session with random id
        // 使用随机id注册会话
        let id = self.rng.gen::<usize>();
        println!("id: {:?}, msg: {:?}", id, msg.addr);
        self.sessions.insert(id, msg.addr);

        // auto join session to main room
        // 自动加入到主会议室的会话
        self.rooms.get_mut("main").unwrap().insert(id);

        // send id back
        // 发回id
        id
    }
}

/// Handler for Disconnect message.
/// 断开连接消息的处理程序。
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        // remove address
        // 删除地址
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            // 从所有文件室中删除会话
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        // send message to other users
        // 向其他用户发送消息
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}

/// Handler for Message message.
/// 消息消息的处理程序。
impl Handler<Message> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Context<Self>) {
        println!("id: {:?}", msg.id);
        self.send_message(&msg.room, msg.msg.as_str(), msg.id);
    }
}

/// Handler for `ListRooms` message.
/// “ListRooms”消息的句柄。
impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
/// 加入聊天室，向旧聊天室发送断开连接消息向新聊天室发送加入消息
impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        // remove session from all rooms
        // 从所有文件室中删除会话
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }
        // send message to other users
        // 向其他用户发送消息
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }

        if self.rooms.get_mut(&name).is_none() {
            self.rooms.insert(name.clone(), HashSet::new());
        }
        self.send_message(&name, "Someone connected", id);
        self.rooms.get_mut(&name).unwrap().insert(id);
    }
}
