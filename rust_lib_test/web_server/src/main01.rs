// fn main() {
//     println!("Hello, world!");
// }

use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    println!("收到了");
    // ...
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;//绑定
    //let listener = TcpListener::bind("127.0.0.1:8080").unwrap();//绑定

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
        //handle_client(stream.unwrap());
    }
    Ok(())//返回值
}