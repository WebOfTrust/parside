use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;
use crate::message::{Group, GroupItem};

#[derive(Debug, Clone, Default)]
pub struct SealSourceCouples {
    pub value: Vec<SealSourceCouple>,
}

impl Group<SealSourceCouple> for SealSourceCouples {
    const CODE: Codex = Codex::SealSourceCouples;

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
            tuple((
                Parsers::seqner_parser(cold_code)?,
                Parsers::saider_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;
        let body = body
            .into_iter()
            .map(|(seqner, saider)| SealSourceCouple { seqner, saider })
            .collect();

        return Ok((rest, SealSourceCouples { value: body }));
    }
}

#[derive(Debug, Clone, Default)]
pub struct SealSourceCouple {
    pub seqner: Matter,
    pub saider: Matter,
}

impl SealSourceCouple {
    pub fn new(seqner: Matter, saider: Matter) -> Self {
        Self { seqner, saider }
    }
}

impl GroupItem for SealSourceCouple {
    fn qb64(&self) -> ParsideResult<String> {
        let mut out = String::new();
        out.push_str(&self.seqner.qb64()?);
        out.push_str(&self.saider.qb64()?);
        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.seqner.qb64b()?);
        out.extend_from_slice(&self.saider.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.seqner.qb2()?);
        out.extend_from_slice(&self.saider.qb2()?);
        Ok(out)
    }
}
