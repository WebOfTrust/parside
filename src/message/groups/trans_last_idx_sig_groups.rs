use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::controller_idx_sigs::ControllerIdxSig;
use crate::message::groups::controller_idx_sigs::ControllerIdxSigs;
use crate::message::parsers::Parsers;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Matter, Prefixer};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroups {
    pub value: Vec<TransLastIdxSigGroup>,
}

impl Group<TransLastIdxSigGroup> for TransLastIdxSigGroups {
    const CODE: &'static str = Codex::TransLastIdxSigGroups;

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
            tuple((Parsers::prefixer_parser(cold_code)?, Parsers::siger_list_parser(cold_code)?)),
            counter.count() as usize,
        )(bytes)?;

        let body = body
            .into_iter()
            .map(|(prefixer, isigers)| TransLastIdxSigGroup {
                prefixer,
                isigers: ControllerIdxSigs::new(
                    isigers.into_iter().map(ControllerIdxSig::new).collect(),
                ),
            })
            .collect();

        Ok((rest, TransLastIdxSigGroups { value: body }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroup {
    pub prefixer: Prefixer,
    pub isigers: ControllerIdxSigs,
}

impl TransLastIdxSigGroup {
    pub fn new(prefixer: Prefixer, isigers: ControllerIdxSigs) -> Self {
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

        let counter =
            Counter::new(Some(1), None, Some(TransLastIdxSigGroups::CODE), None, None, None)
                .unwrap();
        let (rest, group) =
            TransLastIdxSigGroups::from_stream_bytes(stream, &counter, &ColdCode::CtB64).unwrap();
        assert!(rest.is_empty());
        assert_eq!(1, group.value.len());
        assert_eq!(MatterCodex::Blake3_256.to_string(), group.value[0].prefixer.code());
    }
}
