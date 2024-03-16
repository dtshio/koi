use networking::message::Message;

use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:1422").await.expect("Failed to connect remote address.");
    let mut message_from_server = vec![0; 4096];

    match stream.read(&mut message_from_server).await {
        Ok(0) => {}
        Ok(bytes_read) => {
            let message = Message::<String>::decode(&message_from_server[..bytes_read]).expect("Failed to decode message.");

            println!("Received {} bytes from remote address: {:?}", bytes_read, message);
        },
        Err(error) => eprintln!("Failed to receive message from server, got {:?}", error)
    };
}
