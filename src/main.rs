use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use tokio::task;

async fn handle_connection(mut socket: TcpStream) {
    let mut buf: [u8; 1024] = [0; 1024];
    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("error reading from socket, {}", e);
                return;
            }
        };
        if let Err(e) = socket.write_all(&buf[..n]).await {
            eprintln!("error writing to socket, {}", e);
            return;
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1024").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        task::spawn(handle_connection(socket));
    }
}
