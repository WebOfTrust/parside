use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::nomify;
use cesride::{
    Cigar, Counter, Dater, Diger, Indexer, Matter, Prefixer, Saider, Seqner, Siger, Verfer,
};
use nom::multi::count;

pub struct Parsers {}

pub type ParserRet<'a, T> = fn(&'a [u8]) -> nom::IResult<&'a [u8], T>;

impl Parsers {
    #[allow(unused)]
    pub(crate) fn diger_parser<'a>(cold_code: &ColdCode) -> ParsideResult<ParserRet<'a, Diger>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::diger_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::diger_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn diger_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Diger)> {
        let matter = (Diger::new_with_qb64b)(bytes)?;
        let size = matter.full_size()? as usize;
        Ok((&bytes[size..], matter))
    }

    fn diger_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Diger)> {
        let matter = (Diger::new_with_qb2)(bytes)?;
        let size = matter.full_size()? as usize / 4 * 3;
        Ok((&bytes[size..], matter))
    }

    pub(crate) fn siger_parser<'a>(cold_code: &ColdCode) -> ParsideResult<ParserRet<'a, Siger>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::siger_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::siger_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn siger_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Siger)> {
        let matter = Siger::new_with_qb64b(bytes, None)?;
        let size = matter.full_size()? as usize;
        Ok((&bytes[size..], matter))
    }

    fn siger_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Siger)> {
        let matter = Siger::new_with_qb2(bytes, None)?;
        let size = matter.full_size()? as usize / 4 * 3;
        Ok((&bytes[size..], matter))
    }

    pub(crate) fn cigar_parser<'a>(cold_code: &ColdCode) -> ParsideResult<ParserRet<'a, Cigar>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::cigar_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::cigar_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn cigar_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Cigar)> {
        let verfer = Verfer::new_with_qb64b(bytes)?;
        let size = verfer.full_size()? as usize;
        let bytes = &bytes[size..];
        let cigar = Cigar::new_with_qb64b(bytes, Some(&verfer))?;
        let size = cigar.full_size()? as usize;
        Ok((&bytes[size..], cigar))
    }

    fn cigar_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Cigar)> {
        let verfer = Verfer::new_with_qb2(bytes)?;
        let size = verfer.full_size()? as usize / 4 * 3;
        let bytes = &bytes[size..];
        let cigar = Cigar::new_with_qb2(bytes, Some(&verfer))?;
        let size = cigar.full_size()? as usize / 4 * 3;
        Ok((&bytes[size..], cigar))
    }

    pub(crate) fn prefixer_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<ParserRet<'a, Prefixer>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::prefixer_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::prefixer_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn prefixer_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Prefixer)> {
        let matter = (Prefixer::new_with_qb64b)(bytes)?;
        let size = matter.full_size()? as usize;
        Ok((&bytes[size..], matter))
    }

    fn prefixer_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Prefixer)> {
        let matter = (Prefixer::new_with_qb2)(bytes)?;
        let size = matter.full_size()? as usize / 4 * 3;
        Ok((&bytes[size..], matter))
    }

    pub(crate) fn seqner_parser<'a>(cold_code: &ColdCode) -> ParsideResult<ParserRet<'a, Seqner>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::seqner_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::seqner_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn seqner_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Seqner)> {
        let matter = (Seqner::new_with_qb64b)(bytes)?;
        let size = matter.full_size()? as usize;
        Ok((&bytes[size..], matter))
    }

    fn seqner_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Seqner)> {
        let matter = (Seqner::new_with_qb2)(bytes)?;
        let size = matter.full_size()? as usize / 4 * 3;
        Ok((&bytes[size..], matter))
    }

    pub(crate) fn dater_parser<'a>(cold_code: &ColdCode) -> ParsideResult<ParserRet<'a, Dater>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::dater_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::dater_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn dater_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Dater)> {
        let matter = (Dater::new_with_qb64b)(bytes)?;
        let size = matter.full_size()? as usize;
        Ok((&bytes[size..], matter))
    }

    fn dater_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Dater)> {
        let matter = (Dater::new_with_qb2)(bytes)?;
        let size = matter.full_size()? as usize / 4 * 3;
        Ok((&bytes[size..], matter))
    }

    pub(crate) fn saider_parser<'a>(cold_code: &ColdCode) -> ParsideResult<ParserRet<'a, Saider>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::saider_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::saider_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn saider_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Saider)> {
        let matter = (Saider::new_with_qb64b)(bytes)?;
        let size = matter.full_size()? as usize;
        Ok((&bytes[size..], matter))
    }

    fn saider_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Saider)> {
        let matter = (Saider::new_with_qb2)(bytes)?;
        let size = matter.full_size()? as usize / 4 * 3;
        Ok((&bytes[size..], matter))
    }

    pub(crate) fn counter_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<ParserRet<'a, Counter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::counter_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::counter_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn counter_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Counter)> {
        let counter = Counter::new(None, None, None, Some(bytes), None, None)?;
        let size = counter.full_size()? as usize;
        Ok((&bytes[size..], counter))
    }

    fn counter_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Counter)> {
        let counter = Counter::new(None, None, None, None, None, Some(bytes))?;
        let size = counter.full_size()? as usize / 4 * 3;
        Ok((&bytes[size..], counter))
    }

    pub(crate) fn siger_list_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<ParserRet<'a, Vec<Siger>>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::siger_list_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::siger_list_from_q2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn siger_list_from_qb64b<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], Vec<Siger>)> {
        let (rest, counter) = Self::counter_from_qb64b(bytes)?;
        let (rest, values) =
            count(nomify!(Parsers::siger_from_qb64b), counter.count() as usize)(rest)?;
        Ok((rest, values))
    }

    fn siger_list_from_q2<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], Vec<Siger>)> {
        let (rest, counter) = Parsers::counter_from_qb2(bytes)?;
        let (rest, values) =
            count(nomify!(Parsers::siger_from_qb2), counter.count() as usize)(rest)?;
        Ok((rest, values))
    }
}
