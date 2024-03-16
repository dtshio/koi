use networking::connection::Connection;
use networking::message::Message;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1422")
        .await
        .expect("Failed to bind at port {}");
    
    while let Ok((stream, address)) = listener.accept().await {
        tokio::spawn(async move {
            let mut connection = Connection::from(stream);
            let message = Message::<String> { 
                payload: String::from("hello")
            };

            match connection.send(&message).await {
                Ok(bytes_written) => println!("Write {:?} bytes to {}", bytes_written, address),
                Err(error) => eprintln!("Failed to write to {}: {:?}", address, error)
           }
        });
    }
}
