use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:1422").await.expect("Failed to connect remote address.");
    let mut message_from_server: [u8; 5] = [0; 5];

    match stream.read(&mut message_from_server).await {
        Ok(0) => {}
        Ok(bytes_read) => println!("Received {} bytes from remote address: {:?}", bytes_read, String::from_utf8_lossy(&message_from_server)),
        Err(error) => eprintln!("Failed to receive message from server, got {:?}", error)
    };
}
