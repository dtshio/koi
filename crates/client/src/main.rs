use networking::connection::Connection;
use protocol::{message_handler::ClientMessageHandler, message_type::MessageType};

use tokio::net::TcpStream;

struct Handler;

impl ClientMessageHandler<Connection> for Handler {
    async fn handle_event(&self, _: Connection, message: String) {
        println!("event :: {}", message);
    }
}

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:1422").await.expect("Failed to connect remote address.");

    let mut connection = Connection::from(stream);
    let mut packet_from_server = vec![0; 4096];

    let message_handler = Handler;

    match connection.recv::<MessageType>(&mut packet_from_server).await {
        Ok(None) => {}
        Ok(Some(packet)) => match packet.contents {
            MessageType::Event(event) => message_handler.handle_event(connection, event).await
        },
        Err(error) => {
            eprintln!("Failed to receive packet from server, got {:?}", error)
        }
    };
}
