mod base;
pub mod command;
pub mod prelude;
mod test;
mod util;

use base::Sender;
use serde::{de::DeserializeOwned, Serialize};
use util::*;

pub trait Client {
    fn send(data: &str) -> Result<String>;
    fn serialize<T: Serialize>(data: &T) -> Result<String>;
    fn deserialize<T: DeserializeOwned>(data: &str) -> Result<T>;
}

impl<T> Sender for T
where
    T: Client,
{
    fn send(data: &str) -> Result<String> {
        Self::send(data)
    }
    fn serialize<S: Serialize>(data: &S) -> Result<String> {
        Self::serialize::<S>(data)
    }
    fn deserialize<D: DeserializeOwned>(data: &str) -> Result<D> {
        Self::deserialize::<D>(data)
    }
}
