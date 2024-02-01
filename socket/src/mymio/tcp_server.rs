// You can run this example from the root of the mio repo:
// cargo run --example tcp_server --features="os-poll net"
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token, Waker};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::from_utf8;
use std::thread;
use std::time::Duration;
use std::sync::Arc;

// Setup some tokens to allow us to identify which event is for which socket.
// 设置一些令牌，使我们能够识别哪个事件用于哪个套接字。
const SERVER: Token = Token(0);

// Some data we'll send over the connection.
// 我们将通过连接发送一些数据。
const DATA: &[u8] = b"Hello world 5555555!\n";

#[cfg(not(target_os = "wasi"))]
pub fn test() -> io::Result<()> {

    env_logger::init();

    // Create a poll instance.
    // 创建一个轮询实例。
    let mut poll = Poll::new()?;
    // Create storage for events.
    // 为事件创建存储。
    let mut events = Events::with_capacity(128);

    // Setup the TCP server socket.
    // 设置TCP服务器套接字。
    let addr = "127.0.0.1:9000".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;
    
    // Register the server with poll we can receive events for it.
    // 使用轮询注册服务器，我们可以接收它的事件。
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE | Interest::WRITABLE)?;

    // Map of `Token` -> `TcpStream`.
    let mut connections = HashMap::new();
    // Unique token for each incoming connection.
    // 每个传入连接的唯一令牌。
    let mut unique_token = Token(SERVER.0 + 1);

    println!("You can connect to the server using `nc`:");
    println!(" $ nc 127.0.0.1 9000");
    println!("You'll see our welcome message and anything you type will be printed here.");

    loop {
        if let Err(err) = poll.poll(&mut events, None) {
            if interrupted(&err) {
                continue;
            }
            println!("退出");
            return Err(err);
        }

        for event in events.iter() {
            match event.token() {
                SERVER => loop {//等待连接
                    // Received an event for the TCP server socket, which
                    // indicates we can accept an connection.
                    // 收到TCP服务器套接字的事件，表示我们可以接受连接。
                    let (mut connection, address) = match server.accept() {
                        Ok((connection, address)) => (connection, address),
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // If we get a `WouldBlock` error we know our
                            // listener has no more incoming connections queued,
                            // so we can return to polling and wait for some
                            // more.
                            // 如果我们得到一个“WouldBlock”错误，我们知道我们的侦听器没有更多的传入连接排队，所以我们可以返回轮询并等待更多的连接。
                            println!("等待更多连接或事件");
                            break;
                        }
                        Err(e) => {
                            // If it was any other kind of error, something went
                            // wrong and we terminate with an error.
                            // 如果是其他类型的错误，就会出现问题错误，我们以错误终止。
                            println!("退出");
                            return Err(e);
                        }
                    };

                    println!("Accepted connection from: {}", address);

                    let token = next(&mut unique_token);
                    println!("注册连接");
                    poll.registry().register(
                        &mut connection,
                        token,
                        Interest::READABLE.add(Interest::WRITABLE),
                    )?;

                    connections.insert(token, connection);
                },
                token => {
                    // Maybe received an event for a TCP connection.
                    // 可能接收到TCP连接的事件。
                    println!("收到事件: {:?}", token);//连接/断开各一次.发消息也会触发
                    let done = if let Some(connection) = connections.get_mut(&token) {
                        handle_connection_event(poll.registry(), connection, event)?
                    } else {
                        // Sporadic events happen, we can safely ignore them.
                        // 偶发事件的发生，我们可以放心地忽略它们。
                        println!("忽略事件");
                        false
                    };
                    if done {
                        //使用轮询实例取消注册事件。
                        println!("取消注册");
                        if let Some(mut connection) = connections.remove(&token) {
                            poll.registry().deregister(&mut connection)?;
                        }
                    }
                }
            }
        }
    }
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

/// Returns `true` if the connection is done.
// 如果连接完成，则返回“true”。
fn handle_connection_event(
    registry: &Registry,
    connection: &mut TcpStream,
    event: &Event,
) -> io::Result<bool> {
    //写消息
    // loop {
        if event.is_writable() {//如果事件包含可写就绪状态，则返回true。
            // We can (maybe) write to the connection.
            // 我们可以（也许）写信给连接。
            match connection.write(DATA) {
                // We want to write the entire `DATA` buffer in a single go. If we
                // write less we'll return a short write error (same as
                // `io::Write::write_all` does).
                // 我们希望一次性写入整个“DATA”缓冲区。如果我们写得更少，我们将返回一个短写错误（与`io:：write:：write_all`相同）。
                Ok(n) if n < DATA.len() => {
                    // 这通常意味着，只有写入特定数量的字节，操作才能成功，但只能写入较小数量的字节。
                    println!("写入失败");
                    return Err(io::ErrorKind::WriteZero.into())
                },
                Ok(size) => {
                    // After we've written something we'll reregister the connection
                    // to only respond to readable events.
                    // 在我们写了一些东西之后，我们将重新注册连接，以便只响应可读事件。
                    println!("消息写入: {}", size);
                    registry.reregister(connection, event.token(), Interest::READABLE)?
                }
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                // 会阻塞“错误”是操作系统表示连接实际上还没有准备好执行此I/O操作的方式。
                Err(ref err) if would_block(err) => {
                    println!("{}", err);
                }
                // Got interrupted (how rude!), we'll try again.
                // 被打断了（多么粗鲁！），我们再试一次。
                Err(ref err) if interrupted(err) => {
                    println!("重试一次");
                    return handle_connection_event(registry, connection, event)
                }
                // Other errors we'll consider fatal.
                // 其他我们认为是致命的错误。
                Err(err) => {
                    println!("写入错误: {:?}", err);//对方断开连接触发
                    // return Err(err)
                    // break;
                },
            }
        }
    // }

    //读消息
    if event.is_readable() {
        let mut connection_closed = false;
        let mut received_data = vec![0; 4096];
        let mut bytes_read = 0;
        // We can (maybe) read from the connection.
        // 我们可以（也许）从连接中读取。
        loop {
            match connection.read(&mut received_data[bytes_read..]) {
                Ok(0) => {
                    // Reading 0 bytes means the other side has closed the
                    // connection or is done writing, then so are we.
                    // 读取0字节意味着另一方已关闭连接或已完成写入，那么我们也是。
                    connection_closed = true;
                    println!("连接断开");//对方发送空消息时触发
                    break;
                }
                Ok(n) => {
                    println!("消息读取: {}", n);
                    bytes_read += n;
                    if bytes_read == received_data.len() {
                        received_data.resize(received_data.len() + 1024, 0);
                    }
                }
                // Would block "errors" are the OS's way of saying that the
                // connection is not actually ready to perform this I/O operation.
                // 会阻塞“错误”是操作系统表示连接实际上还没有准备好执行此I/O操作的方式。
                Err(ref err) if would_block(err) => break,
                Err(ref err) if interrupted(err) => continue,
                // Other errors we'll consider fatal.
                // 其他我们认为是致命的错误。
                Err(err) => {
                    println!("读取错误: {:?}", err);//对方断开连接触发
                    // return Err(err)
                    break;
                },
            }
        }

        if bytes_read != 0 {
            let received_data = &received_data[..bytes_read];
            if let Ok(str_buf) = from_utf8(received_data) {
                println!("Received data: {}", str_buf.trim_end());
            } else {
                let sparkle_heart = unsafe {
                    String::from_utf8_unchecked(received_data.to_vec())
                };
                println!("Received (none UTF-8) data: {:?}", sparkle_heart);
            }
        }

        if connection_closed {
            println!("Connection closed");
            return Ok(true);
        }
    }

    Ok(false)
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}

#[cfg(target_os = "wasi")]
fn main() {
    panic!("can't bind to an address with wasi")
}





const WAKE_TOKEN: Token = Token(10);
pub fn test2() -> io::Result<()>{
    println!("===========");
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(2);

    let waker = Arc::new(Waker::new(poll.registry(), WAKE_TOKEN)?);

    // We need to keep the Waker alive, so we'll create a clone for the thread we create below.
    // 我们需要保持唤醒程序的活力，所以我们将为下面创建的线程创建一个克隆。
    let waker1 = waker.clone();
    let handle = thread::spawn(move || {
        // Working hard, or hardly working?
        // 努力工作，或几乎不工作
        thread::sleep(Duration::from_millis(500));

        // Now we'll wake the queue on the other thread.
        // 现在我们将唤醒另一个线程上的队列。
        waker1.wake().expect("unable to wake");
    });

    // On our current thread we'll poll for events, without a timeout.
    // 在我们当前的线程上，我们将轮询事件，不超时。
    poll.poll(&mut events, None)?;

    // After about 500 milliseconds we should be awoken by the other thread and get a single event.
    // 大约500毫秒后，我们应该被另一个线程唤醒，并获得一个事件。
    assert!(!events.is_empty());
    let waker_event = events.iter().next().unwrap();
    assert!(waker_event.is_readable());
    assert_eq!(waker_event.token(), WAKE_TOKEN);
    Ok(())
}



// After this number of sockets is accepted, the server will shutdown.
const MAX_SOCKETS: usize = 32;

// Pick a token that will not be used by any other socket and use that one
// for the listener.
const LISTENER: Token = Token(1024);


pub fn test3() -> io::Result<()> {
    println!("=========");
    // Used to store the sockets.
    let mut sockets = HashMap::new();

    // This is used to generate a unique token for a socket
    let mut next_socket_index = 0;

    // The `Poll` instance
    let mut poll = Poll::new()?;

    // Tcp listener
    let mut listener = TcpListener::bind("127.0.0.1:9000".parse().unwrap())?;

    // Register the listener
    poll.registry().register(&mut listener, LISTENER, Interest::READABLE)?;

    // Spawn a thread that will connect a bunch of sockets then close them
    let addr = listener.local_addr()?;
    thread::spawn(move || {
        use std::net::TcpStream;

        // +1 here is to connect an extra socket to signal the socket to close
        for _ in 0..(MAX_SOCKETS+1) {
            // Connect then drop the socket
            let _ = TcpStream::connect(addr).unwrap();
        }
    });

    // Event storage
    let mut events = Events::with_capacity(1024);

    // Read buffer, this will never actually get filled
    let mut buf = [0; 256];

    // The main event loop
    loop {
        // Wait for events
        poll.poll(&mut events, None)?;

        for event in &events {
            match event.token() {
                LISTENER => {
                    // Perform operations in a loop until `WouldBlock` is
                    // encountered.
                    loop {
                        match listener.accept() {
                            Ok((mut socket, _)) => {
                                // Shutdown the server
                                if next_socket_index == MAX_SOCKETS {
                                    return Ok(());
                                }

                                // Get the token for the socket
                                let token = Token(next_socket_index);
                                next_socket_index += 1;

                                // Register the new socket w/ poll
                                poll.registry().register(&mut socket, token, Interest::READABLE)?;

                                // Store the socket
                                sockets.insert(token, socket);
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // Socket is not ready anymore, stop accepting
                                break;
                            }
                            e => panic!("err={:?}", e), // Unexpected error
                        }
                    }
                }
                token => {
                    // Always operate in a loop
                    loop {
                        match sockets.get_mut(&token).unwrap().read(&mut buf) {
                            Ok(0) => {
                                // Socket is closed, remove it from the map
                                sockets.remove(&token);
                                break;
                            }
                            // Data is not actually sent in this example
                            Ok(_) => unreachable!(),
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // Socket is not ready anymore, stop reading
                                break;
                            }
                            e => panic!("err={:?}", e), // Unexpected error
                        }
                    }
                }
            }
        }
    }
}


const CLIENT: Token = Token(1);
pub fn test4()-> Result<(), Box<dyn Error>>  {
    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the server socket.
    let addr = "127.0.0.1:13265".parse()?;
    let mut server = TcpListener::bind(addr)?;
    // Start listening for incoming connections.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // Setup the client socket.
    let mut client = TcpStream::connect(addr)?;
    // Register the socket.
    poll.registry()
        .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    // Start an event loop.
    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            // We can use the token we previously provided to `register` to
            // determine for which socket the event is.
            match event.token() {
                SERVER => {
                    // If this is an event for the server, it means a connection
                    // is ready to be accepted.
                    //
                    // Accept the connection and drop it immediately. This will
                    // close the socket and notify the client of the EOF.
                    let connection = server.accept();
                    drop(connection);
                }
                CLIENT => {
                    if event.is_writable() {
                        // We can (likely) write to the socket without blocking.
                    }

                    if event.is_readable() {
                        // We can (likely) read from the socket without blocking.
                    }

                    // Since the server just shuts down the connection, let's
                    // just exit from our event loop.
                    return Ok(());
                }
                // We don't expect any events with tokens other than those we provided.
                _ => unreachable!(),
            }
        }
    }
}