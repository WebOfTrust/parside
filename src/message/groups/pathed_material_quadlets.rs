use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Indexer, Siger};

// FIXME: Implement proper definition
#[derive(Debug, Clone, Default)]
pub struct PathedMaterialQuadlets {
    pub value: Vec<PathedMaterialQuadlet>,
}

impl Group<PathedMaterialQuadlet> for PathedMaterialQuadlets {
    const CODE: &'static str = Codex::PathedMaterialQuadlets;

    fn new(value: Vec<PathedMaterialQuadlet>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<PathedMaterialQuadlet> {
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

#[derive(Debug, Clone, Default)]
pub struct PathedMaterialQuadlet {
    pub siger: Siger,
}

impl PathedMaterialQuadlet {
    pub fn new(siger: Siger) -> Self {
        Self { siger }
    }
}

impl GroupItem for PathedMaterialQuadlet {
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
