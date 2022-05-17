use std::net::{TcpListener, TcpStream};
use std::io::Read;
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();//读取
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // ...
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;//绑定
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())//返回值
}