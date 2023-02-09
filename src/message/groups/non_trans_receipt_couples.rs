use crate::error::ParsideResult;
use crate::message::cold_code::ColdCodes;
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

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCodes,
    ) -> ParsideResult<(&'a [u8], NonTransReceiptCouples)> {
        let (rest, body) = count(
            tuple((
                Parsers::matter_parser(cold_code)?,
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
            NonTransReceiptCouples::from_stream_bytes(stream, &counter, &ColdCodes::CtB64).unwrap();
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
