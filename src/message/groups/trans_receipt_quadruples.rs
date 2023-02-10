use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;
use crate::message::{Group, GroupItem};

#[derive(Debug, Clone, Default)]
pub struct TransReceiptQuadruples {
    pub value: Vec<TransReceiptQuadruple>,
}

impl Group<TransReceiptQuadruple> for TransReceiptQuadruples {
    const CODE: Codex = Codex::TransReceiptQuadruples;

    fn new(value: Vec<TransReceiptQuadruple>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<TransReceiptQuadruple> {
        &self.value
    }
}

impl TransReceiptQuadruples {
    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], TransReceiptQuadruples)> {
        let (rest, body) = count(
            tuple((
                Parsers::prefixer_parser(cold_code)?,
                Parsers::seqner_parser(cold_code)?,
                Parsers::saider_parser(cold_code)?,
                Parsers::siger_parser(cold_code)?,
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
        Self {
            prefixer,
            seqner,
            saider,
            siger,
        }
    }
}

impl GroupItem for TransReceiptQuadruple {
    fn qb64(&self) -> ParsideResult<String> {
        let mut out = String::new();
        out.push_str(&self.prefixer.qb64()?);
        out.push_str(&self.seqner.qb64()?);
        out.push_str(&self.saider.qb64()?);
        out.push_str(&self.siger.qb64()?);
        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.prefixer.qb64b()?);
        out.extend_from_slice(&self.seqner.qb64b()?);
        out.extend_from_slice(&self.saider.qb64b()?);
        out.extend_from_slice(&self.siger.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.prefixer.qb2()?);
        out.extend_from_slice(&self.seqner.qb2()?);
        out.extend_from_slice(&self.saider.qb2()?);
        out.extend_from_slice(&self.siger.qb2()?);
        Ok(out)
    }
}
