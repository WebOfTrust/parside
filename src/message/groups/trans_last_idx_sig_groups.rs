use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;
use crate::message::groups::controller_idx_sigs::ControllerIdxSigs;

#[derive(Debug, Clone, Default)]
pub struct TransLastIdxSigGroups {
    pub value: Vec<TransLastIdxSigGroup>,
}

impl TransLastIdxSigGroups {
    pub const CODE: Codex = Codex::TransLastIdxSigGroups;

    pub fn new(value: Vec<TransLastIdxSigGroup>) -> Self {
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
        for couple in self.value.iter() {
            out.push_str(&couple.prefixer.qb64()?);
            out.push_str(&couple.isigers.qb64()?);
        }
        Ok(out)
    }

    pub fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.prefixer.qb64b()?);
            out.extend_from_slice(&couple.isigers.qb64b()?);
        }
        Ok(out)
    }

    pub fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.prefixer.qb2()?);
            out.extend_from_slice(&couple.isigers.qb2()?);
        }
        Ok(out)
    }

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
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
