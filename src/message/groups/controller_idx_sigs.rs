use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::message::parsers::Parsers;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex as CounterCodex;
use cesride::{Counter, Indexer, Siger};
use nom::multi::count;

#[derive(Debug, Clone, Default)]
pub struct ControllerIdxSigs {
    pub value: Vec<ControllerIdxSig>,
}

impl Group<ControllerIdxSig> for ControllerIdxSigs {
    const CODE: &'static str = CounterCodex::ControllerIdxSigs;

    fn new(value: Vec<ControllerIdxSig>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<ControllerIdxSig> {
        &self.value
    }
}

impl ControllerIdxSigs {
    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], ControllerIdxSigs)> {
        let (rest, body) =
            count(Parsers::siger_parser(cold_code)?, counter.count() as usize)(bytes)?;
        let body = body.into_iter().map(|siger| ControllerIdxSig { siger }).collect();
        Ok((rest, ControllerIdxSigs { value: body }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct ControllerIdxSig {
    pub siger: Siger,
}

impl ControllerIdxSig {
    pub fn new(siger: Siger) -> Self {
        Self { siger }
    }
}

impl GroupItem for ControllerIdxSig {
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_controller_idx_sigs() {
        let stream = br#"AABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O"#;

        let counter =
            Counter::new(Some(1), None, Some(ControllerIdxSigs::CODE), None, None, None).unwrap();
        let (rest, group) =
            ControllerIdxSigs::from_stream_bytes(stream, &counter, &ColdCode::CtB64).unwrap();

        assert!(rest.is_empty());
        assert_eq!(1, group.value.len());
        assert_eq!(MatterCodex::Ed25519_Seed.to_string(), group.value[0].siger.code());
    }
}
