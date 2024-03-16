use rmp_serde::{self as rmps, encode::Error as EncodeError, decode::Error as DecodeError};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<T> {
    pub payload: T,
}

impl<T: Serialize> Message<T> {
    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        rmps::to_vec(self)
    }
}

impl<T: DeserializeOwned> Message<T> {
    pub fn decode(message: &[u8]) -> Result<Message<T>, DecodeError> {
        rmps::from_slice(message)
    } 
}