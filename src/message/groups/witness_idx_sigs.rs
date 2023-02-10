use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use crate::message::Group;

#[derive(Debug, Clone, Default)]
pub struct WitnessIdxSigs {
    pub value: Vec<Matter>,
}

impl Group<Matter> for WitnessIdxSigs {
    const CODE: Codex = Codex::WitnessIdxSigs;

    fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<Matter> {
        &self.value
    }
}

impl WitnessIdxSigs {
    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], WitnessIdxSigs)> {
        let (rest, body) =
            count(Parsers::siger_parser(cold_code)?, counter.count() as usize)(bytes)?;
        return Ok((rest, WitnessIdxSigs { value: body }));
    }
}