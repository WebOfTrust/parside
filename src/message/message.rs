use serde::de::DeserializeOwned;

use crate::error::{ParsideError, ParsideResult};
use crate::message::CesrGroup;
use crate::message::custom_payload::CustomPayload;
use crate::message::cold_code::ColdCodes;

#[derive(Debug)]
pub enum Message {
    Custom { value: CustomPayload },
    Group { value: CesrGroup },
}

impl Message {
    pub fn from_stream_bytes(bytes: &[u8]) -> ParsideResult<(&[u8], Message)> {
        if bytes.is_empty() {
            return Err(ParsideError::Empty);
        }

        let cold_code = ColdCodes::try_from(bytes[0])?;
        match cold_code {
            ColdCodes::CtB64 | ColdCodes::CtOpB2 | ColdCodes::OpB64 => {
                CesrGroup::from_stream_bytes(bytes)
                    .map(|(rest, value)| (rest, Message::Group { value }))
            }
            ColdCodes::JSON => CustomPayload::from_json_stream(bytes)
                .map(|(rest, value)| (rest, Message::Custom { value })),
            ColdCodes::CBOR => CustomPayload::from_cbor_stream(bytes)
                .map(|(rest, value)| (rest, Message::Custom { value })),
            ColdCodes::MGPK1 | ColdCodes::MGPK2 => CustomPayload::from_mgpk_stream(bytes)
                .map(|(rest, value)| (rest, Message::Custom { value })),
            ColdCodes::Free => Err(ParsideError::Unexpected(format!(
                "Unsupported cold code {}",
                bytes[0]
            ))),
        }
    }

    pub fn payload(&self) -> ParsideResult<&CustomPayload> {
        match self {
            Message::Group { .. } => Err(ParsideError::NotExist),
            Message::Custom { value } => Ok(value),
        }
    }

    pub fn typed_payload<'de, D: DeserializeOwned>(&self) -> ParsideResult<D> {
        match self {
            Message::Group { .. } => Err(ParsideError::NotExist),
            Message::Custom { value } => value.to_typed_message::<D>(),
        }
    }

    pub fn cesr_group(&self) -> ParsideResult<&CesrGroup> {
        match self {
            Message::Group { value } => Ok(value),
            Message::Custom { .. } => Err(ParsideError::NotExist),
        }
    }
}
