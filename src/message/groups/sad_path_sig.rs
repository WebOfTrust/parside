use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::parsers::Parsers;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Matter, Pather, Prefixer, Saider, Seqner};
use nom::multi::count;
use nom::sequence::tuple;

use super::{ControllerIdxSig, ControllerIdxSigs};

// FIXME: Implement proper definition
#[derive(Debug, Clone, Default)]
pub struct SadPathSigs {
    pub value: Vec<SadPathSig>,
}

impl Group<SadPathSig> for SadPathSigs {
    const CODE: &'static str = Codex::SadPathSig;

    fn new(value: Vec<SadPathSig>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<SadPathSig> {
        &self.value
    }
}

impl SadPathSigs {
    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], SadPathSigs)> {
        let (rest, body) = count(
            tuple((
                Parsers::pather_parser(cold_code)?,
                Parsers::counter_parser(cold_code)?,
                Parsers::prefixer_parser(cold_code)?,
                Parsers::seqner_parser(cold_code)?,
                Parsers::saider_parser(cold_code)?,
                Parsers::siger_list_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;

        let body = body
            .into_iter()
            .map(|(pather, tcounter, prefixer, seqner, saider, sigers)| SadPathSig {
                pather,
                tcounter,
                prefixer,
                seqner,
                saider,
                sigers: ControllerIdxSigs::new(
                    sigers.into_iter().map(ControllerIdxSig::new).collect(),
                ),
            })
            .collect();

        Ok((rest, SadPathSigs { value: body }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SadPathSig {
    pub pather: Pather,
    pub tcounter: Counter,
    pub prefixer: Prefixer,
    pub seqner: Seqner,
    pub saider: Saider,
    pub sigers: ControllerIdxSigs,
}

impl GroupItem for SadPathSig {
    fn qb64(&self) -> ParsideResult<String> {
        let mut out = String::new();

        out += &self.pather.qb64()?;
        out += &self.tcounter.qb64()?;
        out += &self.prefixer.qb64()?;
        out += &self.seqner.qb64()?;
        out += &self.saider.qb64()?;
        out += &self.sigers.qb64()?;

        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = vec![0u8; self.full_size()?];
        let mut offset = 0;
        let mut len = self.pather.full_size()?;
        out[offset..offset + len].copy_from_slice(&self.pather.qb64b()?);
        offset += len;
        len = self.tcounter.full_size()?;
        out[offset..offset + len].copy_from_slice(&self.tcounter.qb64b()?);
        offset += len;
        len = self.prefixer.full_size()?;
        out[offset..offset + len].copy_from_slice(&self.prefixer.qb64b()?);
        offset += len;
        len = self.seqner.full_size()?;
        out[offset..offset + len].copy_from_slice(&self.seqner.qb64b()?);
        offset += len;
        len = self.saider.full_size()?;
        out[offset..offset + len].copy_from_slice(&self.saider.qb64b()?);
        offset += len;
        len = self.sigers.full_size()?;
        out[offset..offset + len].copy_from_slice(&self.sigers.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = vec![0u8; self.full_size()? / 4 * 3];
        let mut offset = 0;
        let mut len = self.pather.full_size()? / 4 * 3;
        out[offset..len].copy_from_slice(&self.pather.qb2()?);
        offset += len;
        len = self.tcounter.full_size()? / 4 * 3;
        out[offset..offset + len].copy_from_slice(&self.tcounter.qb2()?);
        offset += len;
        len = self.prefixer.full_size()? / 4 * 3;
        out[offset..len].copy_from_slice(&self.prefixer.qb2()?);
        offset += len;
        len = self.seqner.full_size()? / 4 * 3;
        out[offset..len].copy_from_slice(&self.seqner.qb2()?);
        offset += len;
        len = self.saider.full_size()? / 4 * 3;
        out[offset..len].copy_from_slice(&self.saider.qb2()?);
        offset += len;
        len = self.sigers.full_size()? / 4 * 3;
        out[offset..len].copy_from_slice(&self.sigers.qb2()?);
        Ok(out)
    }

    fn full_size(&self) -> ParsideResult<usize> {
        let mut size = 0usize;

        size += self.pather.full_size()?;
        size += self.tcounter.full_size()?;
        size += self.prefixer.full_size()?;
        size += self.seqner.full_size()?;
        size += self.saider.full_size()?;
        size += self.sigers.full_size()?;

        Ok(size)
    }
}
