use tokio::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// #[tokio::main]
pub async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;

    loop {
        // 接受来自此侦听器的新传入连接。
        // 一旦建立了新的TCP连接，就会产生此函数。当建立时，将返回相应的[TcpStream]和远程对等方的地址。
        let (mut stream, addr) = listener.accept().await?;

		tokio::spawn(async move {
            let (mut rd, mut wr) = stream.split();
            println!("{addr}");
			println!("read and write socket spilt success");
			let data = "Hello, world!\n";
			let _ = wr.write_all(data.as_bytes()).await;
			println!("write data success");

    		let mut buf = vec![0; 1024];
    		match rd.read(&mut buf).await {
        		Ok(n) => {
            		let msg = String::from_utf8_lossy(&buf[..n]);
            		println!("read data : {}", msg);
        		},
        		Err(e) => eprintln!("Failed to read from socket; err = {}", e),
   			}
        });
	}
}

// #[tokio::main]
pub async fn client() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9000").await?;
    let (mut rd, mut wr) = stream.split();

    let mut buf = vec![0; 1024];
    loop {
        let n = match rd.read(&mut buf).await {
            Ok(n) if n == 0 => {
				println!("read zero data, finish");
				return Ok(());
			}
            Ok(n) => {
				let msg = String::from_utf8_lossy(&buf[0..buf.len()]).to_string();
                let str = "03690_XHKG";
                let value = serde_json::to_string(&str).unwrap();
                buf = value.as_bytes().to_vec();//字符串转换为Ascii码
				println!("read {} bytes'data : {}", n, msg);
				n
			}
            Err(e) => {
                println!("failed to read from socket; err = {}", e);
                return Err(e);
            }
        };

        // for i in 0..10{
            match wr.write_all(&buf[0..buf.len()]).await {
                Ok(_) => {
                    println!("write data to server success");
                    ()
                }
                Err(e) => {
                    println!("failed to write to socket; err = {}", e);
                    return Err(e);
                }
            }
        // }
        wr.flush().await?;
    }
}
