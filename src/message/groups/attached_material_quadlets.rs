use crate::error::ParsideResult;
use crate::message::cold_code::ColdCodes;
use cesride::counter::Codex;
use cesride::{Counter, Matter};

#[derive(Debug, Clone, Default)]
pub struct AttachedMaterialQuadlets {
    pub value: Vec<Matter>,
}

impl AttachedMaterialQuadlets {
    pub(crate) fn code() -> String {
        Codex::AttachedMaterialQuadlets.code().to_string()
    }

    pub(crate) fn from_stream_bytes<'a>(
        _bytes: &'a [u8],
        _counter: &Counter,
        _format: &ColdCodes,
    ) -> ParsideResult<(&'a [u8], AttachedMaterialQuadlets)> {
        unimplemented!();
    }
}
