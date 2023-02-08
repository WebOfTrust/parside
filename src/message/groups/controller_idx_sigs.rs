use crate::error::ParsideResult;
use crate::message::cold_code::ColdCodes;
use crate::utils::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;

#[derive(Debug, Clone, Default)]
pub struct ControllerIdxSigs {
    pub value: Vec<Matter>,
}

impl ControllerIdxSigs {
    pub(crate) fn code() -> String {
        Codex::ControllerIdxSigs.code().to_string()
    }

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCodes,
    ) -> ParsideResult<(&'a [u8], ControllerIdxSigs)> {
        let (rest, body) =
            count(Parsers::matter_parser(cold_code)?, counter.count() as usize)(bytes)?;
        return Ok((rest, ControllerIdxSigs { value: body }));
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_controller_idx_sigs() {
        let stream = br#"ABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O"#;

        let counter = Counter::new(ControllerIdxSigs::code(), 1);
        let (rest, group) =
            ControllerIdxSigs::from_stream_bytes(stream, &counter, &ColdCodes::CtB64).unwrap();

        assert!(rest.is_empty());
        assert_eq!(1, group.value.len());
        assert_eq!(
            MatterCodex::Ed25519_Seed.code().to_string(),
            group.value[0].code()
        );
    }
}
