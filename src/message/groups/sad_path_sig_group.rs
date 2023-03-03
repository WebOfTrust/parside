use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Indexer, Siger};

// FIXME: Implement proper definition
#[derive(Debug, Clone, Default)]
pub struct SadPathSigGroups {
    pub value: Vec<SadPathSigGroup>,
}

impl Group<SadPathSigGroup> for SadPathSigGroups {
    const CODE: &'static str = Codex::SadPathSigGroup;

    fn new(value: Vec<SadPathSigGroup>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<SadPathSigGroup> {
        &self.value
    }
}

impl SadPathSigGroups {
    pub(crate) fn from_stream_bytes<'a>(
        _bytes: &'a [u8],
        _counter: &Counter,
        _cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], SadPathSigGroups)> {
        unimplemented!();
    }
}

#[derive(Debug, Clone, Default)]
pub struct SadPathSigGroup {
    pub siger: Siger,
}

impl SadPathSigGroup {
    pub fn new(siger: Siger) -> Self {
        Self { siger }
    }
}

impl GroupItem for SadPathSigGroup {
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
