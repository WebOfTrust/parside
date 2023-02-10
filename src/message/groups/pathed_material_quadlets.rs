use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use crate::message::Group;

// FIXME: Implement proper definition
#[derive(Debug, Clone, Default)]
pub struct PathedMaterialQuadlets {
    pub value: Vec<Matter>,
}

impl Group<Matter> for PathedMaterialQuadlets {
    const CODE: Codex = Codex::PathedMaterialQuadlets;

    fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<Matter> {
        &self.value
    }
}

impl PathedMaterialQuadlets {
    pub(crate) fn from_stream_bytes<'a>(
        _bytes: &'a [u8],
        _counter: &Counter,
        _cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], PathedMaterialQuadlets)> {
        unimplemented!();
    }
}
