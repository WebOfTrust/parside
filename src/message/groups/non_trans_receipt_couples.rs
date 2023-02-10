use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;
use crate::message::{Group, GroupItem};

#[derive(Debug, Clone, Default)]
pub struct NonTransReceiptCouples {
    pub value: Vec<NonTransReceiptCouple>,
}

impl Group<NonTransReceiptCouple> for NonTransReceiptCouples {
    const CODE: Codex = Codex::NonTransReceiptCouples;

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
        let (rest, body) = count(
            tuple((
                Parsers::verfer_parser(cold_code)?,
                Parsers::cigar_parser(cold_code)?,
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

impl GroupItem for NonTransReceiptCouple {
    fn qb64(&self) -> ParsideResult<String> {
        let mut out = String::new();
        out.push_str(&self.verfer.qb64()?);
        out.push_str(&self.cigar.qb64()?);
        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.verfer.qb64b()?);
        out.extend_from_slice(&self.cigar.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.verfer.qb2()?);
        out.extend_from_slice(&self.cigar.qb2()?);
        Ok(out)
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
