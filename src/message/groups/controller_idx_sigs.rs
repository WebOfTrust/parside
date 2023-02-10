use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::{Counter, Matter};
use cesride::counter::Codex as CounterCodex;
use nom::multi::count;
use crate::message::Group;

#[derive(Debug, Clone, Default)]
pub struct ControllerIdxSigs {
    pub value: Vec<Matter>,
}

impl Group<Matter> for ControllerIdxSigs {
    const CODE: CounterCodex = CounterCodex::ControllerIdxSigs;

    fn new(value: Vec<Matter>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<Matter> {
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
        return Ok((rest, ControllerIdxSigs { value: body }));
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_controller_idx_sigs() {
        let stream = br#"ABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O"#;

        let counter = Counter::new(ControllerIdxSigs::CODE.code(), 1);
        let (rest, group) =
            ControllerIdxSigs::from_stream_bytes(stream, &counter, &ColdCode::CtB64).unwrap();

        assert!(rest.is_empty());
        assert_eq!(1, group.value.len());
        assert_eq!(
            MatterCodex::Ed25519_Seed.code().to_string(),
            group.value[0].code()
        );
    }
}
