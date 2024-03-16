use networking::message::Message;

use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1422")
        .await
        .expect("Failed to bind at port {}");

    
    while let Ok((mut socket, address)) = listener.accept().await {
        let message = Message::<String> { 
            payload: String::from("hello")
        };

        match socket.write(&message.encode().expect("Failed to encode message")).await {
            Ok(bytes_written) => println!("Write {:?} bytes to {}", bytes_written, address),
            Err(error) => eprintln!("Failed to write to {}: {:?}", address, error)
        }
    }
}
