use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::nomify;
use cesride::{Counter, Matter, Diger, Verfer};
use nom::multi::count;

macro_rules! matter_wrapper {
    ($func:expr, $bytes:ident) => ({
        let matter = $func($bytes)?;
        let size = matter.full_size()? as usize;
        Ok((&$bytes[size..], matter))
    })
}

macro_rules! counter_wrapper {
    ($func:expr, $bytes:ident) => ({
        let counter = $func($bytes)?;
        let size = counter.sizage()?.fs as usize;
        Ok((&$bytes[size..], counter))
    })
}

pub struct Parsers {}

impl Parsers {
    pub(crate) fn matter_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::matter_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::matter_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn matter_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb64b, bytes)
    }

    fn matter_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb2, bytes)
    }

    #[allow(unused)]
    pub(crate) fn diger_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::diger_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::diger_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn diger_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(<Matter as Diger>::new_with_qb64b, bytes)
    }

    fn diger_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(<Matter as Diger>::new_with_qb2, bytes)
    }

    pub(crate) fn verfer_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::verfer_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::verfer_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn verfer_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(<Matter as Verfer>::new_with_qb64b, bytes)
    }

    fn verfer_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(<Matter as Verfer>::new_with_qb2, bytes)
    }

    pub(crate) fn siger_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::siger_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::siger_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn siger_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(<Matter as Verfer>::new_with_qb64b, bytes) // FIXME: Siger instead Matter
    }

    fn siger_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(<Matter as Verfer>::new_with_qb2, bytes) // FIXME: Siger instead Matter
    }

    pub(crate) fn cigar_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::cigar_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::cigar_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn cigar_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb64b, bytes) // FIXME: Cigar instead Matter
    }

    fn cigar_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb2, bytes) // FIXME: Cigar instead Matter
    }

    pub(crate) fn prefixer_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::prefixer_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::prefixer_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn prefixer_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb64b, bytes) // FIXME: Prefixer instead Matter
    }

    fn prefixer_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb2, bytes) // FIXME: Prefixer instead Matter
    }

    pub(crate) fn seqner_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::seqner_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::seqner_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn seqner_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb64b, bytes) // FIXME: Seqner instead Matter
    }

    fn seqner_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb2, bytes) // FIXME: Seqner instead Matter
    }

    pub(crate) fn saider_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::saider_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::saider_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn saider_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb64b, bytes) // FIXME: Saider instead Matter
    }

    fn saider_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb2, bytes) // FIXME: Saider instead Matter
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
        counter_wrapper!(Counter::new_with_qb64b, bytes)
    }

    fn counter_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Counter)> {
        counter_wrapper!(Counter::new_with_qb2, bytes)
    }

    #[allow(unused)]
    pub(crate) fn indexer_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::indexer_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::indexer_from_q2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn indexer_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb64b, bytes) // FIXME: here should be Indexer parser
    }

    fn indexer_from_q2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        matter_wrapper!(Matter::new_with_qb2, bytes) // FIXME: here should be Indexer parser
    }

    pub(crate) fn matter_list_parser<'a>(
        cold_code: &ColdCode,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Vec<Matter>>> {
        match cold_code {
            ColdCode::CtB64 | ColdCode::OpB64 => Ok(nomify!(Self::matter_list_from_qb64b)),
            ColdCode::CtOpB2 => Ok(nomify!(Self::matter_list_from_q2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn matter_list_from_qb64b<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], Vec<Matter>)> {
        let (rest, counter) = Self::counter_from_qb64b(&bytes)?;
        let (rest, values) = count(
            nomify!(Parsers::indexer_from_qb64b),
            counter.count() as usize,
        )(rest)?;
        Ok((rest, values))
    }

    fn matter_list_from_q2<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], Vec<Matter>)> {
        let (rest, counter) = Parsers::counter_from_qb2(&bytes)?;
        let (rest, values) =
            count(nomify!(Parsers::indexer_from_q2), counter.count() as usize)(rest)?;
        Ok((rest, values))
    }
}