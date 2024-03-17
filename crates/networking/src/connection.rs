use serde::{de::DeserializeOwned, Serialize};
use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use crate::packet::Packet;

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
    pub async fn send<E: Serialize>(&mut self, packet: &Packet<E>) -> io::Result<usize> {
        match packet.encode() {
            Ok(packet) => self.stream.write(&packet).await,
            Err(error) => Err(io::Error::new(io::ErrorKind::InvalidInput, error))
        }
    }

    pub async fn recv<T: DeserializeOwned>(&mut self, packet: &mut [u8]) -> io::Result<Option<Packet<T>>> {
        match self.stream.read(packet).await {
            Ok(0) => Ok(None),
            Ok(bytes_read) => Packet::<T>::decode(&packet[..bytes_read]).map_or(Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to decode message")), |message| Ok(Some(message))),
            Err(error) => Err(io::Error::new(io::ErrorKind::InvalidInput, error))
        }
    }
}