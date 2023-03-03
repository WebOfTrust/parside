use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Indexer, Siger};

// FIXME: Implement proper definition
#[derive(Debug, Clone, Default)]
pub struct SadPathSigs {
    pub value: Vec<SadPathSig>,
}

impl Group<SadPathSig> for SadPathSigs {
    const CODE: &'static str = Codex::SadPathSig;

    fn new(value: Vec<SadPathSig>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<SadPathSig> {
        &self.value
    }
}

impl SadPathSigs {
    pub(crate) fn from_stream_bytes<'a>(
        _bytes: &'a [u8],
        _counter: &Counter,
        _cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], SadPathSigs)> {
        unimplemented!();
    }
}

#[derive(Debug, Clone, Default)]
pub struct SadPathSig {
    pub siger: Siger,
}

impl SadPathSig {
    pub fn new(siger: Siger) -> Self {
        Self { siger }
    }
}

impl GroupItem for SadPathSig {
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
