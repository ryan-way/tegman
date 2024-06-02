pub enum Error {
    SerializationError(String),
    SendError(String),
    DeserializationError(String),
}

pub type Result<T> = core::result::Result<T, Error>;
