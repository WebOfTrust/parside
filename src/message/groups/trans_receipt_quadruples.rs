use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::parsers::Parsers;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Counter, Indexer, Matter, Prefixer, Saider, Seqner, Siger};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct TransReceiptQuadruples {
    pub value: Vec<TransReceiptQuadruple>,
}

impl Group<TransReceiptQuadruple> for TransReceiptQuadruples {
    const CODE: &'static str = Codex::TransReceiptQuadruples;

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

        Ok((rest, TransReceiptQuadruples { value: body }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct TransReceiptQuadruple {
    pub prefixer: Prefixer,
    pub seqner: Seqner,
    pub saider: Saider,
    pub siger: Siger,
}

impl TransReceiptQuadruple {
    pub fn new(prefixer: Prefixer, seqner: Seqner, saider: Saider, siger: Siger) -> Self {
        Self { prefixer, seqner, saider, siger }
    }
}

impl GroupItem for TransReceiptQuadruple {
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
        len = self.siger.full_size()? as usize;
        unsafe { out[offset..len].as_bytes_mut() }.copy_from_slice(self.siger.qb64()?.as_bytes());
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
        len = self.siger.full_size()? as usize;
        out[offset..len].copy_from_slice(&self.siger.qb64b()?);
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
        len = self.siger.full_size()? as usize / 4 * 3;
        out[offset..len].copy_from_slice(&self.siger.qb2()?);
        Ok(out)
    }

    fn full_size(&self) -> ParsideResult<u32> {
        let size = self.prefixer.full_size()?
            + self.seqner.full_size()?
            + self.saider.full_size()?
            + self.siger.full_size()?;
        Ok(size)
    }
}
