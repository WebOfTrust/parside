use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::message::parsers::Parsers;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Indexer, Siger};
use nom::multi::count;

#[derive(Debug, Clone, Default)]
pub struct WitnessIdxSigs {
    pub value: Vec<WitnessIdxSig>,
}

impl Group<WitnessIdxSig> for WitnessIdxSigs {
    const CODE: &'static str = Codex::WitnessIdxSigs;

    fn new(value: Vec<WitnessIdxSig>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<WitnessIdxSig> {
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
        let body = body.into_iter().map(|siger| WitnessIdxSig { siger }).collect();
        return Ok((rest, WitnessIdxSigs { value: body }));
    }
}

#[derive(Debug, Clone, Default)]
pub struct WitnessIdxSig {
    pub siger: Siger,
}

impl WitnessIdxSig {
    pub fn new(siger: Siger) -> Self {
        Self { siger }
    }
}

impl GroupItem for WitnessIdxSig {
    fn qb64(&self) -> ParsideResult<String> {
        self.siger.qb64().map_err(ParsideError::from)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        self.siger.qb64b().map_err(ParsideError::from)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        self.siger.qb2().map_err(ParsideError::from)
    }
}
