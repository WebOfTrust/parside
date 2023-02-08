use nom::error::ErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsideError {
    #[error("Payload deserialize error: {0}")]
    PayloadDeserializeError(String),

    #[error("Unable to deserialize stream: {0}")]
    StreamDeserializationError(String),

    #[error("Empty bytes stream passed for parsing")]
    Empty,

    #[error("Requested variant does not exists")]
    NotExist,

    #[error("Unexpected variant")]
    Unexpected(String),

    #[error("Common error")]
    Common(String),
}

impl<E> From<nom::Err<E>> for ParsideError {
    fn from(_: nom::Err<E>) -> ParsideError {
        ParsideError::StreamDeserializationError(ErrorKind::IsNot.description().to_string())
    }
}

impl From<Box<dyn std::error::Error>> for ParsideError {
    fn from(err: Box<dyn std::error::Error>) -> ParsideError {
        ParsideError::Common(err.to_string())
    }
}

pub type ParsideResult<T> = Result<T, ParsideError>;
