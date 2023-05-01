use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::parsers::Parsers;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Matter, Saider, Seqner};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct SealSourceCouples {
    pub value: Vec<SealSourceCouple>,
}

impl Group<SealSourceCouple> for SealSourceCouples {
    const CODE: &'static str = Codex::SealSourceCouples;

    fn new(value: Vec<SealSourceCouple>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<SealSourceCouple> {
        &self.value
    }
}

impl SealSourceCouples {
    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], SealSourceCouples)> {
        let (rest, body) = count(
            tuple((Parsers::seqner_parser(cold_code)?, Parsers::saider_parser(cold_code)?)),
            counter.count() as usize,
        )(bytes)?;
        let body =
            body.into_iter().map(|(seqner, saider)| SealSourceCouple { seqner, saider }).collect();

        Ok((rest, SealSourceCouples { value: body }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct SealSourceCouple {
    pub seqner: Seqner,
    pub saider: Saider,
}

impl SealSourceCouple {
    pub fn new(seqner: Seqner, saider: Saider) -> Self {
        Self { seqner, saider }
    }
}

impl GroupItem for SealSourceCouple {
    fn qb64(&self) -> ParsideResult<String> {
        let mut out = String::new();
        out += &self.seqner.qb64()?;
        out += &self.saider.qb64()?;
        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = vec![0u8; self.full_size()?];
        let mut offset = 0;
        let mut len = self.seqner.full_size()?;
        out[offset..len].copy_from_slice(&self.seqner.qb64b()?);
        offset += len;
        len = self.saider.full_size()?;
        out[offset..len].copy_from_slice(&self.saider.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = vec![0u8; self.full_size()? / 4 * 3];
        let mut offset = 0;
        let mut len = self.seqner.full_size()? / 4 * 3;
        out[offset..len].copy_from_slice(&self.seqner.qb2()?);
        offset += len;
        len = self.saider.full_size()? / 4 * 3;
        out[offset..len].copy_from_slice(&self.saider.qb2()?);
        Ok(out)
    }

    fn full_size(&self) -> ParsideResult<usize> {
        let size = self.seqner.full_size()? + self.saider.full_size()?;
        Ok(size)
    }
}
