use crate::error::ParsideResult;
use crate::message::cold_code::ColdCodes;
use crate::utils::parsers::Parsers;
use cesride::counter::Codex;
use cesride::{Counter, Matter};
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct TransReceiptQuadruples {
    pub value: Vec<NonTransReceiptCouple>,
}

impl TransReceiptQuadruples {
    pub(crate) fn code() -> String {
        Codex::TransReceiptQuadruples.code().to_string()
    }

    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCodes,
    ) -> ParsideResult<(&'a [u8], TransReceiptQuadruples)> {
        let (rest, body) = count(
            tuple((
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;
        let body = body
            .into_iter()
            .map(|(prefixer, seqner, saider, siger)| NonTransReceiptCouple {
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
pub struct NonTransReceiptCouple {
    pub prefixer: Matter,
    pub seqner: Matter,
    pub saider: Matter,
    pub siger: Matter,
}
