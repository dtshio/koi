use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:1422")
        .await
        .expect("Failed to bind at port {}");

    while let Ok((mut socket, address)) = listener.accept().await {
        match socket.write(b"Hello").await {
            Ok(bytes_written) => println!("Write {:?} bytes to {}", bytes_written, address),
            Err(error) => eprintln!("Failed to write to {}: {:?}", address, error)
        }
    }
}
