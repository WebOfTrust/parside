use crate::error::ParsideResult;
use crate::message::cold_code::ColdCodes;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct TransIdxSigGroups {
    pub value: Vec<TransIdxSigGroup>,
}

impl TransIdxSigGroups {
    pub const CODE: Codex = Codex::TransIdxSigGroups;

    pub fn new(value: Vec<TransIdxSigGroup>) -> Self {
        Self { value }
    }

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCodes,
    ) -> ParsideResult<(&'a [u8], TransIdxSigGroups)> {
        let (rest, body) = count(
            tuple((
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_list_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;

        let body = body
            .into_iter()
            .map(|(prefixer, seqner, saider, isigers)| TransIdxSigGroup {
                prefixer,
                seqner,
                saider,
                isigers,
            })
            .collect();

        return Ok((rest, TransIdxSigGroups { value: body }));
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransIdxSigGroup {
    pub prefixer: Matter,
    pub seqner: Matter,
    pub saider: Matter,
    pub isigers: Vec<Matter>,
}

impl TransIdxSigGroup {
    pub fn new(prefixer: Matter, seqner: Matter, saider: Matter, isigers: Vec<Matter>) -> Self {
        Self { prefixer, seqner, saider, isigers }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_trans_idx_sig_groups() {
        let stream = br#"EFhg5my9DuMU6gw1CVk6QgkmZKBttWSXDzVzWVmxh0_K0AAAAAAAAAAAAAAAAAAAAAAAEFhg5my9DuMU6gw1CVk6QgkmZKBttWSXDzVzWVmxh0_K-AABAADghKct9eYTuSgSd5wdPSYG06tGX7ZRp_BDnrgbSxJpsJtrA-fP7Pa1W602gHeMrO6HZsD1z3tWV5jGlApFmVIB"#;

        let counter = Counter::new(TransIdxSigGroups::CODE.code(), 1);

        let (rest, group) =
            TransIdxSigGroups::from_stream_bytes(stream, &counter, &ColdCodes::CtB64).unwrap();
        assert!(rest.is_empty());
        assert_eq!(1, group.value.len());
        assert_eq!(
            MatterCodex::Blake3_256.code().to_string(),
            group.value[0].prefixer.code()
        );
        assert_eq!(
            MatterCodex::Salt_128.code().to_string(),
            group.value[0].seqner.code()
        );
        assert_eq!(
            MatterCodex::Blake3_256.code().to_string(),
            group.value[0].saider.code()
        );
    }
}
