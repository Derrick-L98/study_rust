use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();//读取
    println!("Request: {}\n", String::from_utf8_lossy(&buffer[..]));
    
    //响应
    let content = fs::read_to_string("main.html").unwrap();
    //let response = "THHP/1.1 200 OK\r\n\r\n";
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();//刷新
} 

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;//绑定
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())//返回值
}