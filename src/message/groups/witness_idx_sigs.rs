use crate::error::ParsideResult;
use crate::message::cold_code::ColdCodes;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;

#[derive(Debug, Clone, Default)]
pub struct WitnessIdxSigs {
    pub value: Vec<Matter>,
}

impl WitnessIdxSigs {
    pub const CODE: Codex = Codex::WitnessIdxSigs;

    pub fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCodes,
    ) -> ParsideResult<(&'a [u8], WitnessIdxSigs)> {
        let (rest, body) =
            count(Parsers::matter_parser(cold_code)?, counter.count() as usize)(bytes)?;
        return Ok((rest, WitnessIdxSigs { value: body }));
    }
}
