use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use crate::message::Group;

// FIXME: Implement proper definition
#[derive(Debug, Clone, Default)]
pub struct SadPathSig {
    pub value: Vec<Matter>,
}

impl Group<Matter> for SadPathSig {
    const CODE: Codex = Codex::SadPathSig;

    fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<Matter> {
        &self.value
    }
}

impl SadPathSig {
    pub(crate) fn from_stream_bytes<'a>(
        _bytes: &'a [u8],
        _counter: &Counter,
        _cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], SadPathSig)> {
        unimplemented!();
    }
}
