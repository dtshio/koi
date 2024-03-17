use std::fmt::Debug;

use rmp_serde::{self as rmps, encode::Error as EncodeError, decode::Error as DecodeError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet<T> {
    #[serde(flatten)]
    pub contents: T,
}

impl<T: Serialize + DeserializeOwned> From<T> for Packet<T> {
    fn from(contents: T) -> Packet<T> {
        Packet { contents }
    }
}

impl<T: Serialize> Packet<T> {
    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        rmps::to_vec(self)
    }
}

impl<T: DeserializeOwned> Packet<T> {
    pub fn decode(message: &[u8]) -> Result<Packet<T>, DecodeError> {
        rmps::from_slice(message)
    } 
}