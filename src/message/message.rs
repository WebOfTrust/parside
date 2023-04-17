use serde::de::DeserializeOwned;

use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::message::custom_payload::CustomPayload;
use crate::message::groups::CesrGroup;

/// Datastructures representing single parsed message which either custom payload or known CESR group
#[derive(Debug)]
pub enum Message {
    Custom { value: CustomPayload },
    Group { value: CesrGroup },
}

impl Message {
    /// Parse single message from provided bytes
    pub fn from_stream_bytes(bytes: &[u8]) -> ParsideResult<(&[u8], Message)> {
        if bytes.is_empty() {
            return Err(ParsideError::EmptyBytesStream);
        }

        let cold_code = ColdCode::try_from(bytes[0])?;
        match cold_code {
            ColdCode::CtB64 | ColdCode::CtOpB2 | ColdCode::OpB64 => {
                CesrGroup::from_stream_bytes(bytes)
                    .map(|(rest, value)| (rest, Message::Group { value }))
            }
            ColdCode::Json => CustomPayload::from_json_stream(bytes)
                .map(|(rest, value)| (rest, Message::Custom { value })),
            ColdCode::Cbor => CustomPayload::from_cbor_stream(bytes)
                .map(|(rest, value)| (rest, Message::Custom { value })),
            ColdCode::MGPK1 | ColdCode::MGPK2 => CustomPayload::from_mgpk_stream(bytes)
                .map(|(rest, value)| (rest, Message::Custom { value })),
            ColdCode::Free => {
                Err(ParsideError::Unexpected(format!("Unsupported cold code {}", bytes[0])))
            }
        }
    }

    /// Get custom payload from parsed message
    pub fn payload(&self) -> ParsideResult<&CustomPayload> {
        match self {
            Message::Group { .. } => Err(ParsideError::NotExist),
            Message::Custom { value } => Ok(value),
        }
    }

    /// Get custom payload converted to specific data type from parsed message
    pub fn typed_payload<D: DeserializeOwned>(&self) -> ParsideResult<D> {
        match self {
            Message::Group { .. } => Err(ParsideError::NotExist),
            Message::Custom { value } => value.to_typed_message::<D>(),
        }
    }

    /// Get CESR group from parsed message
    pub fn cesr_group(&self) -> ParsideResult<&CesrGroup> {
        match self {
            Message::Group { value } => Ok(value),
            Message::Custom { .. } => Err(ParsideError::NotExist),
        }
    }
}
