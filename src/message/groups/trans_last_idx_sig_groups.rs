use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;
use crate::message::{Group, GroupItem};
use crate::message::groups::controller_idx_sigs::ControllerIdxSigs;

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroups {
    pub value: Vec<TransLastIdxSigGroup>,
}

impl Group<TransLastIdxSigGroup> for TransLastIdxSigGroups {
    const CODE: Codex = Codex::TransLastIdxSigGroups;

    fn new(value: Vec<TransLastIdxSigGroup>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<TransLastIdxSigGroup> {
        &self.value
    }
}

impl TransLastIdxSigGroups {
    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], TransLastIdxSigGroups)> {
        let (rest, body) = count(
            tuple((
                Parsers::prefixer_parser(cold_code)?,
                Parsers::matter_list_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;

        let body = body
            .into_iter()
            .map(|(prefixer, isigers)| TransLastIdxSigGroup {
                prefixer,
                isigers: ControllerIdxSigs::new(isigers)
            })
            .collect();

        return Ok((rest, TransLastIdxSigGroups { value: body }));
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroup {
    pub prefixer: Matter,
    pub isigers: ControllerIdxSigs,
}

impl TransLastIdxSigGroup {
    pub fn new(prefixer: Matter, isigers: ControllerIdxSigs) -> Self {
        Self { prefixer, isigers }
    }
}

impl GroupItem for TransLastIdxSigGroup {
    fn qb64(&self) -> ParsideResult<String> {
        let mut out = String::new();
        out.push_str(&self.prefixer.qb64()?);
        out.push_str(&self.isigers.qb64()?);
        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.prefixer.qb64b()?);
        out.extend_from_slice(&self.isigers.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.prefixer.qb2()?);
        out.extend_from_slice(&self.isigers.qb2()?);
        Ok(out)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_trans_last_idx_sig_groups() {
        let stream = br#"EB1f36VmoizOIpBIBv3X4ZiWJQWjtKJ7TMmsZltT0B32-AABAAAKB9u6wyLS9kl_iGVGCqrs-3XqFbyGeOKuiOEA9JZpxI9GMv0GJv2wbY1-sOD_HOJcvXO7LSO8g8MSeRXjtL4I"#;

        let counter = Counter::new(TransLastIdxSigGroups::CODE.code(), 1);
        let (rest, group) =
            TransLastIdxSigGroups::from_stream_bytes(stream, &counter, &ColdCode::CtB64).unwrap();
        assert!(rest.is_empty());
        assert_eq!(1, group.value.len());
        assert_eq!(
            MatterCodex::Blake3_256.code().to_string(),
            group.value[0].prefixer.code()
        );
    }
}