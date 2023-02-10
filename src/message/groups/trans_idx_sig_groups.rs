use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;
use crate::message::groups::controller_idx_sigs::ControllerIdxSigs;

#[derive(Debug, Clone, Default)]
pub struct TransIdxSigGroups {
    pub value: Vec<TransIdxSigGroup>,
}

impl TransIdxSigGroups {
    pub const CODE: Codex = Codex::TransIdxSigGroups;

    pub fn new(value: Vec<TransIdxSigGroup>) -> Self {
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
            out.push_str(&couple.seqner.qb64()?);
            out.push_str(&couple.saider.qb64()?);
            out.push_str(&couple.isigers.qb64()?);
        }
        Ok(out)
    }

    pub fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.seqner.qb64b()?);
            out.extend_from_slice(&couple.seqner.qb64b()?);
            out.extend_from_slice(&couple.saider.qb64b()?);
            out.extend_from_slice(&couple.isigers.qb64b()?);
        }
        Ok(out)
    }

    pub fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.seqner.qb2()?);
            out.extend_from_slice(&couple.seqner.qb2()?);
            out.extend_from_slice(&couple.saider.qb2()?);
            out.extend_from_slice(&couple.isigers.qb2()?);
        }
        Ok(out)
    }

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], TransIdxSigGroups)> {
        let (rest, body) = count(
            tuple((
                Parsers::prefixer_parser(cold_code)?,
                Parsers::seqner_parser(cold_code)?,
                Parsers::saider_parser(cold_code)?,
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
                isigers: ControllerIdxSigs::new(isigers),
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
    pub isigers: ControllerIdxSigs,
}

impl TransIdxSigGroup {
    pub fn new(prefixer: Matter, seqner: Matter, saider: Matter, isigers: ControllerIdxSigs) -> Self {
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
            TransIdxSigGroups::from_stream_bytes(stream, &counter, &ColdCode::CtB64).unwrap();
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
