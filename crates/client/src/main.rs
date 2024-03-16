use networking::connection::Connection;

use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:1422").await.expect("Failed to connect remote address.");

    let mut connection = Connection::from(stream);
    let mut message_from_server = vec![0; 4096];

    match connection.recv::<String>(&mut message_from_server).await {
        Ok(None) => {}
        Ok(Some(message)) => println!("Received a message from remote address: {:?}", message),
        Err(error) => eprintln!("Failed to receive message from server, got {:?}", error)
    };
}
