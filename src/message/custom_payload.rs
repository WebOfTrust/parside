use rmp_serde as serde_mgpk;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::io::Cursor;

use crate::error::{ParsideError, ParsideResult};

/// Datastructures representing custom payload
#[derive(Debug)]
pub struct CustomPayload {
    pub value: JsonValue,
}

impl CustomPayload {
    /// Convert custom payload to specific type
    pub fn to_typed_message<D>(&self) -> ParsideResult<D>
    where
        D: DeserializeOwned,
    {
        serde_json::from_value::<D>(self.value.to_owned())
            .map_err(|err| ParsideError::PayloadDeserializeError(err.to_string()))
    }

    /// Parse custom payload from JSON representation
    pub(crate) fn from_json_stream(s: &[u8]) -> ParsideResult<(&[u8], CustomPayload)> {
        let mut stream = serde_json::Deserializer::from_slice(s).into_iter::<JsonValue>();
        match stream.next() {
            Some(Ok(value)) => Ok((&s[stream.byte_offset()..], CustomPayload { value })),
            Some(Err(err)) => Err(ParsideError::PayloadDeserializeError(err.to_string())),
            None => Err(ParsideError::PayloadDeserializeError("End of stream".to_string())),
        }
    }

    /// Parse custom payload from CBOR representation
    pub(crate) fn from_cbor_stream(s: &[u8]) -> ParsideResult<(&[u8], CustomPayload)> {
        let mut stream = serde_cbor::Deserializer::from_slice(s).into_iter::<JsonValue>();
        match stream.next() {
            Some(Ok(value)) => Ok((&s[stream.byte_offset()..], CustomPayload { value })),
            Some(Err(err)) => Err(ParsideError::PayloadDeserializeError(err.to_string())),
            None => Err(ParsideError::PayloadDeserializeError("End of stream".to_string())),
        }
    }

    /// Parse custom payload from MessagePack representation
    pub(crate) fn from_mgpk_stream(s: &[u8]) -> ParsideResult<(&[u8], CustomPayload)> {
        let mut deser = serde_mgpk::Deserializer::new(Cursor::new(s));
        match Deserialize::deserialize(&mut deser) {
            Ok(value) => Ok((&s[deser.get_ref().position() as usize..], CustomPayload { value })),
            Err(err) => Err(ParsideError::PayloadDeserializeError(err.to_string())),
        }
    }
}
