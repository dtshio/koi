use serde::{de::DeserializeOwned, Serialize};
use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use crate::message::Message;

#[derive(Debug)]
pub struct Connection {
    stream: TcpStream
}

impl From<TcpStream> for Connection {
    fn from(stream: TcpStream) -> Connection {
        Connection { stream }
    }
}

impl Connection {
    pub async fn send<E: Serialize>(&mut self, message: &Message<E>) -> io::Result<usize> {
        match message.encode() {
            Ok(message) => self.stream.write(&message).await,
            Err(error) => Err(io::Error::new(io::ErrorKind::InvalidInput, error))
        }
    }

    pub async fn recv<T: DeserializeOwned>(&mut self, message: &mut [u8]) -> io::Result<Option<Message<T>>> {
        match self.stream.read(message).await {
            Ok(0) => Ok(None),
            Ok(bytes_read) => Message::<T>::decode(&message[..bytes_read]).map_or(Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to decode message")), |message| Ok(Some(message))),
            Err(error) => Err(io::Error::new(io::ErrorKind::InvalidInput, error))
        }
    }
}