use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::nomify;
use cesride::{Counter, Matter, Diger, Verfer, Dater, Saider, Cigar, Siger, Indexer, Prefixer, Seqner};
use cesride::matter::raw_size;
use nom::multi::count;

macro_rules! matter_wrapper {
    ($func:expr, $bytes:ident) => ({
        let matter = $func($bytes)?;
        let size = raw_size(&matter.code())? as usize;
        Ok((&$bytes[size..], matter))
    })
}

pub struct Parsers {}

impl Parsers {
    #[allow(unused)]
    pub(crate) fn diger_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Diger>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::diger_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::diger_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn diger_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Diger)> {
        matter_wrapper!(Diger::new_with_qb64b, bytes)
    }

    fn diger_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Diger)> {
        matter_wrapper!(Diger::new_with_qb2, bytes)
    }

    pub(crate) fn siger_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Siger>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::siger_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::siger_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn siger_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Siger)> {
        let matter = Siger::new_with_qb64b(bytes, None)?;
        let size = raw_size(&matter.code())? as usize;
        Ok((&bytes[size..], matter))
    }

    fn siger_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Siger)> {
        let matter = Siger::new_with_qb2(bytes, None)?;
        let size = raw_size(&matter.code())? as usize;
        Ok((&bytes[size..], matter))
    }

    pub(crate) fn cigar_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Cigar>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::cigar_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::cigar_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn cigar_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Cigar)> {
        let verfer = Verfer::new_with_qb64b(bytes)?;
        let size = raw_size(&verfer.code())? as usize;
        let bytes = &bytes[size..];
        let cigar = Cigar::new_with_qb64b(bytes, Some(&verfer))?;
        let size = raw_size(&cigar.code())? as usize;
        Ok((&bytes[size..], cigar))
    }

    fn cigar_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Cigar)> {
        let verfer = Verfer::new_with_qb2(bytes)?;
        let size = raw_size(&verfer.code())? as usize;
        let bytes = &bytes[size..];
        let cigar = Cigar::new_with_qb2(bytes, Some(&verfer))?;
        let size = raw_size(&cigar.code())? as usize;
        Ok((&bytes[size..], cigar))
    }

    pub(crate) fn prefixer_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Prefixer>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::prefixer_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::prefixer_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn prefixer_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Prefixer)> {
        matter_wrapper!(Prefixer::new_with_qb64b, bytes)
    }

    fn prefixer_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Prefixer)> {
        matter_wrapper!(Prefixer::new_with_qb2, bytes)
    }

    pub(crate) fn seqner_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Seqner>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::seqner_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::seqner_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn seqner_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Seqner)> {
        matter_wrapper!(Seqner::new_with_qb64b, bytes)
    }

    fn seqner_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Seqner)> {
        matter_wrapper!(Seqner::new_with_qb2, bytes)
    }

    pub(crate) fn dater_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Dater>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::dater_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::dater_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn dater_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Dater)> {
        matter_wrapper!(Dater::new_with_qb64b, bytes)
    }

    fn dater_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Dater)> {
        matter_wrapper!(Dater::new_with_qb2, bytes)
    }

    pub(crate) fn saider_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Saider>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::saider_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::saider_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn saider_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Saider)> {
        matter_wrapper!(Saider::new_with_qb64b, bytes)
    }

    fn saider_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Saider)> {
        matter_wrapper!(Saider::new_with_qb2, bytes)
    }

    pub(crate) fn counter_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Counter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::counter_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::counter_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn counter_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Counter)> {
        let counter = Counter::new(None, None, None, Some(bytes), None, None)?;
        let size = raw_size(&counter.code())? as usize;
        Ok((&bytes[size..], counter))
    }

    fn counter_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Counter)> {
        let counter = Counter::new(None, None, None, None, None, Some(bytes))?;
        let size = raw_size(&counter.code())? as usize;
        Ok((&bytes[size..], counter))
    }

    pub(crate) fn siger_list_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Vec<Siger>>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::siger_list_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::siger_list_from_q2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn siger_list_from_qb64b<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], Vec<Siger>)> {
        let (rest, counter) = Self::counter_from_qb64b(&bytes)?;
        let (rest, values) = count(
            nomify!(Parsers::siger_from_qb64b),
            counter.count() as usize,
        )(rest)?;
        Ok((rest, values))
    }

    fn siger_list_from_q2<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], Vec<Siger>)> {
        let (rest, counter) = Parsers::counter_from_qb2(&bytes)?;
        let (rest, values) =
            count(nomify!(Parsers::siger_from_qb2), counter.count() as usize)(rest)?;
        Ok((rest, values))
    }
}
