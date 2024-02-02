//! A chat server that broadcasts a message to all connections.
//!
//! This example is explicitly more verbose than it has to be. This is to
//! illustrate more concepts.
//!
//! A chat server for telnet clients. After a telnet client connects, the first
//! line should contain the client's name. After that, all lines sent by a
//! client are broadcasted to all other connected clients.
//!
//! Because the client is telnet, lines are delimited by "\r\n".
//!
//! You can test this out by running:
//!
//!     cargo run --example chat
//!
//! And then in another terminal run:
//!
//!     telnet localhost 6142
//!
//! You can run the `telnet` command in any number of additional windows.
//!
//! You can run the second command in multiple windows and then chat between the
//! two, seeing the messages from the other client as they're received. For all
//! connected clients they'll all join the same room and see everyone else's
//! messages.

#![warn(rust_2018_idioms)]

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
    // Configure a `tracing` subscriber that logs traces emitted by the chat server.
    // 配置一个“跟踪”订阅者，记录聊天服务器发出的跟踪。
    tracing_subscriber::fmt()
        // Filter what traces are displayed based on the RUST_LOG environment variable.
        //根据RUST_LOG环境变量筛选显示的跟踪。
        // Traces emitted by the example code will always be displayed. You
        // can set `RUST_LOG=tokio=trace` to enable additional traces emitted by
        // Tokio itself.
        // 示例代码发出的痕迹将始终显示。您可以设置“RUST_LOG=tokio=trace”以启用tokio本身发出的其他跟踪
        .with_env_filter(EnvFilter::from_default_env().add_directive("chat=info".parse()?))
        // Log events when `tracing` spans are created, entered, exited, or
        // closed. When Tokio's internal tracing support is enabled (as
        // described above), this can be used to track the lifecycle of spawned
        // tasks on the Tokio runtime.
        // 在创建、输入、退出或关闭“跟踪”跨度时记录事件。当启用Tokio的内部跟踪支持时（如上所述），这可以用于跟踪Tokio运行时上派生任务的生命周期。
        .with_span_events(FmtSpan::FULL)
        // Set this subscriber as the default, to collect all traces emitted by
        // the program.
        // 将此订阅者设置为默认值，以收集程序发出的所有跟踪。
        .init();

    // Create the shared state. This is how all the peers communicate.
    //创建共享状态。这就是所有对等方的沟通方式。
    // The server task will hold a handle to this. For every new client, the
    // `state` handle is cloned and passed into the task that processes the
    // client connection.
    // 服务器任务将持有对此的句柄。对于每个新客户端，“状态”句柄都会被克隆并传递到处理客户端连接的任务中。
    let state = Arc::new(Mutex::new(Shared::new()));

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());

    // Bind a TCP listener to the socket address.
    //
    // Note that this is the Tokio TcpListener, which is fully async.
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("server running on {}", addr);

    loop {
        // Asynchronously wait for an inbound TcpStream.
        let (stream, addr) = listener.accept().await?;
        tracing::info!("客户: {} 连接", addr);
        // Clone a handle to the `Shared` state for the new connection.
        let state = Arc::clone(&state);

        // Spawn our handler to be run asynchronously.
        // 生成要异步运行的处理程序。
        tokio::spawn(async move {
            tracing::debug!("accepted connection");
            if let Err(e) = process(state, stream, addr).await {
                tracing::info!("an error occurred; error = {:?}", e);
            }
        });
    }
}

/// Shorthand for the transmit half of the message channel.
// / 短手，用于传输消息通道的一半。
type Tx = mpsc::UnboundedSender<String>;

/// Shorthand for the receive half of the message channel.
/// 用于接收消息通道的一半的Shorthand。
type Rx = mpsc::UnboundedReceiver<String>;

/// Data that is shared between all peers in the chat server.
///
/// This is the set of `Tx` handles for all connected clients. Whenever a
/// message is received from a client, it is broadcasted to all peers by
/// iterating over the `peers` entries and sending a copy of the message on each
/// `Tx`.
struct Shared {
    peers: HashMap<SocketAddr, Tx>,
}

/// The state for each connected client.
struct Peer {
    /// The TCP socket wrapped with the `Lines` codec, defined below.
    ///
    /// This handles sending and receiving data on the socket. When using
    /// `Lines`, we can work at the line level instead of having to manage the
    /// raw byte operations.
    lines: Framed<TcpStream, LinesCodec>,

    /// Receive half of the message channel.
    ///
    /// This is used to receive messages from peers. When a message is received
    /// off of this `Rx`, it will be written to the socket.
    rx: Rx,
}

impl Shared {
    /// Create a new, empty, instance of `Shared`.
    /// 新建一个空的“Shared”实例。
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }

    /// Send a `LineCodec` encoded message to every peer, except
    /// for the sender.
    // / 将“LineCodec”编码的消息发送给除发送方之外的所有对等方。
    async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
        tracing::info!("客户: {}, 消息: {}", sender, message);
        tracing::info!("在线客户: {:?}", self.peers);
        for peer in self.peers.iter_mut() {
            if *peer.0 != sender {
                let _ = peer.1.send(message.into());
            }
        }
    }
}

impl Peer {
    /// Create a new instance of `Peer`.
    // / 创建“对等”的新实例。
    async fn new(
        state: Arc<Mutex<Shared>>,
        lines: Framed<TcpStream, LinesCodec>,
    ) -> io::Result<Peer> {
        // Get the client socket address
        // 获取客户端套接字地址
        let addr = lines.get_ref().peer_addr()?;
        let c = lines.codec();
        tracing::info!("客户 {} 进入", addr);
        // Create a channel for this peer
        // 为此对等方创建通道
        let (tx, rx) = mpsc::unbounded_channel();

        // Add an entry for this `Peer` in the shared state map.
        // 在共享状态映射中为此“对等”添加一个条目。
        state.lock().await.peers.insert(addr, tx);

        Ok(Peer { lines, rx })
    }
}

/// Process an individual chat client
/// 处理个人聊天客户端
async fn process(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    // Send a prompt to the client to enter their username.
    // 向客户端发送提示以输入其用户名。
    lines.send("Please enter your username:").await?;

    // Read the first line from the `LineCodec` stream to get the username.
    // 读取“LineCodec”流中的第一行以获取用户名。
    let username = match lines.next().await {
        Some(Ok(line)) => line,
        // We didn't get a line so we return early here.
        // 我们没有接到电话，所以我们很早就回来了。
        _ => {
            tracing::error!("Failed to get username from {}. Client disconnected.", addr);
            return Ok(());
        }
    };

    // Register our peer with state which internally sets up some channels.
    // 向国家注册我们的同行，国家内部设立了一些渠道。
    let mut peer = Peer::new(state.clone(), lines).await?;

    // A client has connected, let's let everyone know.
    // 一个客户端已经连接，让我们让每个人都知道。
    {
        let mut state = state.lock().await;
        let msg = format!("{} has joined the chat", username);
        tracing::info!("{}", msg);
        state.broadcast(addr, &msg).await;
    }
    tracing::info!("==========================");
    // Process incoming messages until our stream is exhausted by a disconnect.
    // 处理传入消息，直到我们的流因断开连接而耗尽。
    loop {
        tokio::select! {
            // A message was received from a peer. Send it to the current user.
            // 处理传入消息，直到我们的流因断开连接而耗尽。
            Some(msg) = peer.rx.recv() => {
                tracing::info!("接收消息: {}", msg);
                // 发送给客户
                peer.lines.send(&msg).await?;
            }
            // 第二次接收终端输入
            result = peer.lines.next() => match result {
                // A message was received from the current user, we should
                // broadcast this message to the other users.
                // 收到来自当前用户的消息，我们应该将此消息广播给其他用户。
                Some(Ok(msg)) => {
                    let mut state = state.lock().await;
                    // let msg = format!("{}", username, msg);
                    tracing::info!("{}: {}", username, msg);
                    state.broadcast(addr, &msg).await;
                }
                // An error occurred.
                Some(Err(e)) => {
                    tracing::error!(
                        "an error occurred while processing messages for {}; error = {:?}",
                        username,
                        e
                    );
                }
                // The stream has been exhausted.
                None => break,
            },
        }
    }

    // If this section is reached it means that the client was disconnected!
    // Let's let everyone still connected know about it.
    // 如果达到此部分，则表示客户端已断开连接！让我们让所有仍有联系的人都知道这件事。
    {
        let mut state = state.lock().await;
        state.peers.remove(&addr);

        let msg = format!("{} has left the chat", username);
        tracing::info!("{}", msg);
        state.broadcast(addr, &msg).await;
    }

    Ok(())
}
