use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::controller_idx_sigs::ControllerIdxSig;
use crate::message::groups::controller_idx_sigs::ControllerIdxSigs;
use crate::message::parsers::Parsers;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Matter, Prefixer, Saider, Seqner};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct TransIdxSigGroups {
    pub value: Vec<TransIdxSigGroup>,
}

impl Group<TransIdxSigGroup> for TransIdxSigGroups {
    const CODE: &'static str = Codex::TransIdxSigGroups;

    fn new(value: Vec<TransIdxSigGroup>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<TransIdxSigGroup> {
        &self.value
    }
}

impl TransIdxSigGroups {
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
                Parsers::siger_list_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;

        let body = body
            .into_iter()
            .map(|(prefixer, seqner, saider, isigers)| TransIdxSigGroup {
                prefixer,
                seqner,
                saider,
                isigers: ControllerIdxSigs::new(
                    isigers.into_iter().map(ControllerIdxSig::new).collect(),
                ),
            })
            .collect();

        Ok((rest, TransIdxSigGroups { value: body }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransIdxSigGroup {
    pub prefixer: Prefixer,
    pub seqner: Seqner,
    pub saider: Saider,
    pub isigers: ControllerIdxSigs,
}

impl TransIdxSigGroup {
    pub fn new(
        prefixer: Prefixer,
        seqner: Seqner,
        saider: Saider,
        isigers: ControllerIdxSigs,
    ) -> Self {
        Self { prefixer, seqner, saider, isigers }
    }
}

impl GroupItem for TransIdxSigGroup {
    fn qb64(&self) -> ParsideResult<String> {
        let mut out = "\0".repeat(self.full_size()? as usize);
        let mut offset = 0;
        let mut len = self.prefixer.full_size()? as usize;
        unsafe { out[offset..len].as_bytes_mut() }
            .copy_from_slice(self.prefixer.qb64()?.as_bytes());
        offset += len;
        len = self.seqner.full_size()? as usize;
        unsafe { out[offset..len].as_bytes_mut() }.copy_from_slice(self.seqner.qb64()?.as_bytes());
        offset += len;
        len = self.saider.full_size()? as usize;
        unsafe { out[offset..len].as_bytes_mut() }.copy_from_slice(self.saider.qb64()?.as_bytes());
        offset += len;
        len = self.isigers.full_size()? as usize;
        unsafe { out[offset..len].as_bytes_mut() }.copy_from_slice(self.isigers.qb64()?.as_bytes());
        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = vec![0u8; self.full_size()? as usize];
        let mut offset = 0;
        let mut len = self.prefixer.full_size()? as usize;
        out[offset..len].copy_from_slice(&self.prefixer.qb64b()?);
        offset += len;
        len = self.seqner.full_size()? as usize;
        out[offset..len].copy_from_slice(&self.seqner.qb64b()?);
        offset += len;
        len = self.saider.full_size()? as usize;
        out[offset..len].copy_from_slice(&self.saider.qb64b()?);
        offset += len;
        len = self.isigers.full_size()? as usize;
        out[offset..len].copy_from_slice(&self.isigers.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = vec![0u8; self.full_size()? as usize / 4 * 3];
        let mut offset = 0;
        let mut len = self.prefixer.full_size()? as usize / 4 * 3;
        out[offset..len].copy_from_slice(&self.prefixer.qb2()?);
        offset += len;
        len = self.seqner.full_size()? as usize / 4 * 3;
        out[offset..len].copy_from_slice(&self.seqner.qb2()?);
        offset += len;
        len = self.saider.full_size()? as usize / 4 * 3;
        out[offset..len].copy_from_slice(&self.saider.qb2()?);
        offset += len;
        len = self.isigers.full_size()? as usize / 4 * 3;
        out[offset..len].copy_from_slice(&self.isigers.qb2()?);
        Ok(out)
    }

    fn full_size(&self) -> ParsideResult<u32> {
        let size = self.prefixer.full_size()?
            + self.seqner.full_size()?
            + self.saider.full_size()?
            + self.isigers.full_size()?;
        Ok(size)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_trans_idx_sig_groups() {
        let stream = br#"EFhg5my9DuMU6gw1CVk6QgkmZKBttWSXDzVzWVmxh0_K0AAAAAAAAAAAAAAAAAAAAAAAEFhg5my9DuMU6gw1CVk6QgkmZKBttWSXDzVzWVmxh0_K-AABAADghKct9eYTuSgSd5wdPSYG06tGX7ZRp_BDnrgbSxJpsJtrA-fP7Pa1W602gHeMrO6HZsD1z3tWV5jGlApFmVIB"#;

        let counter =
            Counter::new(Some(1), None, Some(TransIdxSigGroups::CODE), None, None, None).unwrap();
        let (rest, group) =
            TransIdxSigGroups::from_stream_bytes(stream, &counter, &ColdCode::CtB64).unwrap();
        assert!(rest.is_empty());
        assert_eq!(1, group.value.len());
        assert_eq!(MatterCodex::Blake3_256.to_string(), group.value[0].prefixer.code());
        assert_eq!(MatterCodex::Salt_128.to_string(), group.value[0].seqner.code());
        assert_eq!(MatterCodex::Blake3_256.to_string(), group.value[0].saider.code());
    }
}
