use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use cesride::counter::Codex;
use cesride::{Counter, Matter};


// FIXME: Implement proper definition
#[derive(Debug, Clone, Default)]
pub struct AttachedMaterialQuadlets {
    pub value: Vec<Matter>,
}

impl AttachedMaterialQuadlets {
    pub const CODE: Codex = Codex::AttachedMaterialQuadlets;

    pub fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    pub fn counter(&self) -> Counter {
        Counter::new(&Self::CODE.code(), self.count())
    }

    pub fn count(&self) -> u32 {
        self.value.len() as u32
    }

    pub fn qb64(&self) -> ParsideResult<String> {
        let mut out = self.counter().qb64()?;
        for matter in self.value.iter() {
            out.push_str(&matter.qb64()?);
        }
        Ok(out)
    }

    pub fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for matter in self.value.iter() {
            out.extend_from_slice(&matter.qb64b()?);
        }
        Ok(out)
    }

    pub fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for matter in self.value.iter() {
            out.extend_from_slice(&matter.qb2()?);
        }
        Ok(out)
    }

    pub(crate) fn from_stream_bytes<'a>(
        _bytes: &'a [u8],
        _counter: &Counter,
        _cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], AttachedMaterialQuadlets)> {
        unimplemented!();
    }
}
