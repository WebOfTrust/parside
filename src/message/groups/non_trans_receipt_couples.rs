use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::message::parsers::Parsers;
use crate::message::{Group, GroupItem};
use cesride::counter::Codex;
use cesride::{Cigar, Counter, Matter};
use nom::multi::count;

#[derive(Debug, Clone, Default)]
pub struct NonTransReceiptCouples {
    pub value: Vec<NonTransReceiptCouple>,
}

impl Group<NonTransReceiptCouple> for NonTransReceiptCouples {
    const CODE: &'static str = Codex::NonTransReceiptCouples;

    fn new(value: Vec<NonTransReceiptCouple>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<NonTransReceiptCouple> {
        &self.value
    }
}

impl NonTransReceiptCouples {
    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], NonTransReceiptCouples)> {
        let (rest, body) =
            count(Parsers::cigar_parser(cold_code)?, counter.count() as usize)(bytes)?;
        let body = body.into_iter().map(|cigar| NonTransReceiptCouple { cigar }).collect();
        Ok((rest, NonTransReceiptCouples { value: body }))
    }
}

#[derive(Debug, Clone, Default)]
pub struct NonTransReceiptCouple {
    pub cigar: Cigar,
}

impl NonTransReceiptCouple {
    pub fn new(cigar: Cigar) -> Self {
        Self { cigar }
    }
}

impl GroupItem for NonTransReceiptCouple {
    fn qb64(&self) -> ParsideResult<String> {
        self.cigar.qb64().map_err(ParsideError::from)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        self.cigar.qb64b().map_err(ParsideError::from)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        self.cigar.qb2().map_err(ParsideError::from)
    }

    fn full_size(&self) -> ParsideResult<u32> {
        let size = self.cigar.full_size()?;
        Ok(size)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_non_trans_receipt_couples() {
        let stream = br#"BD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG"#;

        let counter =
            Counter::new(Some(1), None, Some(NonTransReceiptCouples::CODE), None, None, None)
                .unwrap();
        let (rest, group) =
            NonTransReceiptCouples::from_stream_bytes(stream, &counter, &ColdCode::CtB64).unwrap();
        assert!(rest.is_empty());
        assert_eq!(1, group.value.len());
        assert_eq!(MatterCodex::Ed25519_Sig.to_string(), group.value[0].cigar.code());
    }
}
