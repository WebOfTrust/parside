use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use crate::message::Group;

// FIXME: Implement proper definition
#[derive(Debug, Clone, Default)]
pub struct SadPathSigGroup {
    pub value: Vec<Matter>,
}

impl Group<Matter> for SadPathSigGroup {
    const CODE: Codex = Codex::SadPathSigGroup;

    fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<Matter> {
        &self.value
    }
}

impl SadPathSigGroup {
    pub(crate) fn from_stream_bytes<'a>(
        _bytes: &'a [u8],
        _counter: &Counter,
        _cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], SadPathSigGroup)> {
        unimplemented!();
    }
}
