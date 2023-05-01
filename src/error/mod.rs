use nom::error::ErrorKind;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParsideError {
    #[error("Payload deserialize error: {0}")]
    PayloadDeserializeError(String),

    #[error("Nom error")]
    StreamDeserializationError(ErrorKind),

    #[error("Empty bytes stream passed for parsing")]
    EmptyBytesStream,

    #[error("Requested variant does not exists")]
    NotExist,

    #[error("Unexpected variant")]
    Unexpected(String),

    #[error("Common error")]
    Common(String),
}

impl<E> From<nom::Err<E>> for ParsideError {
    fn from(_: nom::Err<E>) -> ParsideError {
        ParsideError::StreamDeserializationError(ErrorKind::IsNot)
    }
}

impl From<anyhow::Error> for ParsideError {
    fn from(err: anyhow::Error) -> ParsideError {
        ParsideError::Common(err.to_string())
    }
}

pub type ParsideResult<T> = Result<T, ParsideError>;
