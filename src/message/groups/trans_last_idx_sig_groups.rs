use crate::error::ParsideResult;
use crate::message::cold_code::ColdCodes;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroups {
    pub value: Vec<TransLastIdxSigGroup>,
}

impl TransLastIdxSigGroups {
    pub(crate) fn code() -> String {
        Codex::TransLastIdxSigGroups.code().to_string()
    }

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCodes,
    ) -> ParsideResult<(&'a [u8], TransLastIdxSigGroups)> {
        let (rest, body) = count(
            tuple((
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_list_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;

        let body = body
            .into_iter()
            .map(|(prefixer, isigers)| TransLastIdxSigGroup { prefixer, isigers })
            .collect();

        return Ok((rest, TransLastIdxSigGroups { value: body }));
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroup {
    pub prefixer: Matter,
    pub isigers: Vec<Matter>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_trans_last_idx_sig_groups() {
        let stream = br#"EB1f36VmoizOIpBIBv3X4ZiWJQWjtKJ7TMmsZltT0B32-AABAAAKB9u6wyLS9kl_iGVGCqrs-3XqFbyGeOKuiOEA9JZpxI9GMv0GJv2wbY1-sOD_HOJcvXO7LSO8g8MSeRXjtL4I"#;

        let counter = Counter::new(TransLastIdxSigGroups::code(), 1);
        let (rest, group) =
            TransLastIdxSigGroups::from_stream_bytes(stream, &counter, &ColdCodes::CtB64).unwrap();
        assert!(rest.is_empty());
        assert_eq!(1, group.value.len());
        assert_eq!(
            MatterCodex::Blake3_256.code().to_string(),
            group.value[0].prefixer.code()
        );
    }
}
