use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct NonTransReceiptCouples {
    pub value: Vec<NonTransReceiptCouple>,
}

impl NonTransReceiptCouples {
    pub const CODE: Codex = Codex::NonTransReceiptCouples;

    pub fn new(value: Vec<NonTransReceiptCouple>) -> Self {
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
            out.push_str(&couple.verfer.qb64()?);
            out.push_str(&couple.cigar.qb64()?);
        }
        Ok(out)
    }

    pub fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.verfer.qb64b()?);
            out.extend_from_slice(&couple.cigar.qb64b()?);
        }
        Ok(out)
    }

    pub fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.verfer.qb2()?);
            out.extend_from_slice(&couple.cigar.qb2()?);
        }
        Ok(out)
    }

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], NonTransReceiptCouples)> {
        let (rest, body) = count(
            tuple((
                Parsers::verfer_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;
        let body = body
            .into_iter()
            .map(|(verfer, cigar)| NonTransReceiptCouple { verfer, cigar })
            .collect();
        return Ok((rest, NonTransReceiptCouples { value: body }));
    }
}

#[derive(Debug, Clone, Default)]
pub struct NonTransReceiptCouple {
    pub verfer: Matter,
    pub cigar: Matter,
}

impl NonTransReceiptCouple {
    pub fn new(verfer: Matter, cigar: Matter) -> Self {
        Self { verfer, cigar }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_non_trans_receipt_couples() {
        let stream = br#"BD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG"#;

        let counter = Counter::new(NonTransReceiptCouples::CODE.code(), 1);
        let (rest, group) =
            NonTransReceiptCouples::from_stream_bytes(stream, &counter, &ColdCode::CtB64).unwrap();
        assert!(rest.is_empty());
        assert_eq!(1, group.value.len());
        assert_eq!(
            MatterCodex::Ed25519N.code().to_string(),
            group.value[0].verfer.code()
        );
        assert_eq!(
            MatterCodex::Ed25519_Sig.code().to_string(),
            group.value[0].cigar.code()
        );
    }
}
