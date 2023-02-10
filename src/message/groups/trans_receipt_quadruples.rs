use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct TransReceiptQuadruples {
    pub value: Vec<TransReceiptQuadruple>,
}

impl TransReceiptQuadruples {
    pub const CODE: Codex = Codex::TransReceiptQuadruples;

    pub fn new(value: Vec<TransReceiptQuadruple>) -> Self {
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
            out.push_str(&couple.siger.qb64()?);
        }
        Ok(out)
    }

    pub fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.prefixer.qb64b()?);
            out.extend_from_slice(&couple.seqner.qb64b()?);
            out.extend_from_slice(&couple.saider.qb64b()?);
            out.extend_from_slice(&couple.siger.qb64b()?);
        }
        Ok(out)
    }

    pub fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.prefixer.qb2()?);
            out.extend_from_slice(&couple.seqner.qb2()?);
            out.extend_from_slice(&couple.saider.qb2()?);
            out.extend_from_slice(&couple.siger.qb2()?);
        }
        Ok(out)
    }

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], TransReceiptQuadruples)> {
        let (rest, body) = count(
            tuple((
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;
        let body = body
            .into_iter()
            .map(|(prefixer, seqner, saider, siger)| TransReceiptQuadruple {
                prefixer,
                seqner,
                saider,
                siger,
            })
            .collect();

        return Ok((rest, TransReceiptQuadruples { value: body }));
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransReceiptQuadruple {
    pub prefixer: Matter,
    pub seqner: Matter,
    pub saider: Matter,
    pub siger: Matter,
}

impl TransReceiptQuadruple {
    pub fn new(prefixer: Matter, seqner: Matter, saider: Matter, siger: Matter) -> Self {
        Self { prefixer, seqner, saider, siger }
    }
}
