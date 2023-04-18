use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use crate::error::{ParsideError, ParsideResult};

/// Cold code defining the serialization format: https://weboftrust.github.io/ietf-cesr/draft-ssmith-cesr.html#section-3.6
#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, Debug, Clone)]
pub(crate) enum ColdCode {
    // not taken
    Free = 0b000,
    // CountCode Base64
    CtB64 = 0b001,
    // OpCode Base64
    OpB64 = 0b010,
    // JSON Map Event Start
    Json = 0b011,
    // MGPK Fixed Map Event Start
    MGPK1 = 0b100,
    // CBOR Map Event Start
    Cbor = 0b101,
    // MGPK Big 16 or 32 Map Event Start
    MGPK2 = 0b110,
    // CountCode or OpCode Base2
    CtOpB2 = 0b111,
}

impl TryFrom<u8> for ColdCode {
    type Error = ParsideError;

    fn try_from(byte: u8) -> ParsideResult<Self> {
        let tritet = byte >> 5;
        FromPrimitive::from_u8(tritet).ok_or_else(|| {
            ParsideError::PayloadDeserializeError(
                "Unable to parse Message cold start code".to_string(),
            )
        })
    }
}
