use rmp_serde as serde_mgpk;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::io::Cursor;

use crate::error::{ParsideError, ParsideResult};

#[derive(Debug)]
pub struct CustomPayload {
    pub value: JsonValue,
}

impl CustomPayload {
    pub fn to_typed_message<'de, D: DeserializeOwned>(&self) -> ParsideResult<D> {
        serde_json::from_value::<D>(self.value.to_owned())
            .map_err(|err| ParsideError::PayloadDeserializeError(err.to_string()))
    }

    pub(crate) fn from_json_stream(s: &[u8]) -> ParsideResult<(&[u8], CustomPayload)> {
        let mut stream = serde_json::Deserializer::from_slice(s).into_iter::<JsonValue>();
        match stream.next() {
            Some(Ok(value)) => Ok((&s[stream.byte_offset()..], CustomPayload { value })),
            Some(Err(err)) => Err(ParsideError::PayloadDeserializeError(err.to_string())),
            None => Err(ParsideError::PayloadDeserializeError(
                "End of stream".to_string(),
            )),
        }
    }

    pub(crate) fn from_cbor_stream(s: &[u8]) -> ParsideResult<(&[u8], CustomPayload)> {
        let mut stream = serde_cbor::Deserializer::from_slice(s).into_iter::<JsonValue>();
        match stream.next() {
            Some(Ok(value)) => Ok((&s[stream.byte_offset()..], CustomPayload { value })),
            Some(Err(err)) => Err(ParsideError::PayloadDeserializeError(err.to_string())),
            None => Err(ParsideError::PayloadDeserializeError(
                "End of stream".to_string(),
            )),
        }
    }

    pub(crate) fn from_mgpk_stream(s: &[u8]) -> ParsideResult<(&[u8], CustomPayload)> {
        let mut deser = serde_mgpk::Deserializer::new(Cursor::new(s));
        match Deserialize::deserialize(&mut deser) {
            Ok(value) => Ok((
                &s[deser.get_ref().position() as usize..],
                CustomPayload { value },
            )),
            Err(err) => Err(ParsideError::PayloadDeserializeError(err.to_string())),
        }
    }
}
