use crate::error::ParsideResult;
use crate::message::cold_code::ColdCodes;
use cesride::counter::Codex;
use cesride::{Counter, Matter};

#[derive(Debug, Clone, Default)]
pub struct PathedMaterialQuadlets {
    pub value: Vec<Matter>,
}

impl PathedMaterialQuadlets {
    pub const CODE: Codex = Codex::PathedMaterialQuadlets;

    pub fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    pub(crate) fn from_stream_bytes<'a>(
        _bytes: &'a [u8],
        _counter: &Counter,
        _cold_code: &ColdCodes,
    ) -> ParsideResult<(&'a [u8], PathedMaterialQuadlets)> {
        unimplemented!();
    }
}
