use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCodes;
use crate::{nomify, utils};
use cesride::{Counter, Matter};
use nom::multi::count;

pub struct Parsers {}

impl Parsers {
    pub(crate) fn matter_parser<'a>(
        cold_code: &ColdCodes,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCodes::CtB64 | ColdCodes::OpB64 => Ok(nomify!(Self::matter_from_qb64b)),
            ColdCodes::CtOpB2 => Ok(nomify!(Self::matter_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn matter_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        let matter = Matter::new_with_qb64b(bytes)?;
        let size = matter.full_size()? as usize;
        Ok((&bytes[size..], matter))
    }

    fn matter_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        let matter = Matter::new_with_qb2(bytes)?;
        let size = matter.full_size()? as usize;
        Ok((&bytes[size..], matter))
    }

    pub(crate) fn counter_parser<'a>(
        cold_code: &ColdCodes,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Counter>> {
        match cold_code {
            ColdCodes::CtB64 | ColdCodes::OpB64 => Ok(nomify!(Self::counter_from_qb64b)),
            ColdCodes::CtOpB2 => Ok(nomify!(Self::counter_from_qb2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn counter_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Counter)> {
        let counter = Counter::new_with_qb64b(&bytes)?;
        let table = counter.sizage()?;
        let (rest, _) = utils::nom::take_bytes(bytes, table.fs)?;
        Ok((rest, counter))
    }

    fn counter_from_qb2(bytes: &[u8]) -> ParsideResult<(&[u8], Counter)> {
        let counter = Counter::new_with_qb2(&bytes)?;
        let table = counter.sizage()?;
        let (rest, _) = utils::nom::take_bytes(bytes, table.fs)?;
        Ok((rest, counter))
    }

    pub(crate) fn indexer_parser<'a>(
        cold_code: &ColdCodes,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Matter>> {
        match cold_code {
            ColdCodes::CtB64 | ColdCodes::OpB64 => Ok(nomify!(Self::indexer_from_qb64b)),
            ColdCodes::CtOpB2 => Ok(nomify!(Self::indexer_from_q2)),
            _ => Err(ParsideError::Unexpected("Unexpected cold code".to_string())),
        }
    }

    fn indexer_from_qb64b(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        let matter = Matter::new_with_qb64b(bytes)?; // FIXME: here should be another method to parse
        let size = matter.full_size()? as usize;
        Ok((&bytes[size..], matter))
    }

    fn indexer_from_q2(bytes: &[u8]) -> ParsideResult<(&[u8], Matter)> {
        let matter = Matter::new_with_qb2(bytes)?; // FIXME: here should be another method to parse
        let size = matter.full_size()? as usize;
        Ok((&bytes[size..], matter))
    }

    pub(crate) fn matter_list_parser<'a>(
        cold_code: &ColdCodes,
    ) -> ParsideResult<fn(&'a [u8]) -> nom::IResult<&'a [u8], Vec<Matter>>> {
        match cold_code {
            ColdCodes::CtB64 | ColdCodes::OpB64 => Ok(nomify!(Self::matter_list_from_qb64b)),
            ColdCodes::CtOpB2 => Ok(nomify!(Self::matter_list_from_q2)),
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
