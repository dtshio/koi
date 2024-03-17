use std::sync::Arc;

use networking::connection::Connection;
use networking::packet::Packet;

use protocol::message_handler::ServerMessageHandler;
use protocol::message_type::MessageType;

use tokio::net::TcpListener;

struct Handler;

impl ServerMessageHandler<Connection> for Handler {
    async fn handle_connection(&self, mut connection: Connection, address: String) {
        println!("\r\nconnection :: {}", address);

        let packet = Packet::<MessageType> { 
            contents: MessageType::Event(String::from("hello"))
        };

        match connection.send(&packet).await {
            Ok(bytes_written) => println!("connection => write {:?} bytes to {}", bytes_written, address),
            Err(error) => eprintln!("connection => failed to write to {}: {:?}", address, error)
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1422")
        .await
        .expect("Failed to bind at port {}");
    
    let handler = Arc::from(Handler);
    
    while let Ok((stream, address)) = listener.accept().await {
        let handler = handler.clone();
        
        tokio::spawn(async move {
            let connection = Connection::from(stream);

            handler.handle_connection(connection, address.to_string()).await;
        });
    }
}
