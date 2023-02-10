use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::{Counter, Matter};
use cesride::counter::Codex as CounterCodex;
use nom::multi::count;
use nom::sequence::tuple;
use crate::message::{Group, GroupItem};

#[derive(Debug, Clone, Default)]
pub struct FirstSeenReplayCouples {
    pub value: Vec<FirstSeenReplayCouple>,
}

impl Group<FirstSeenReplayCouple> for FirstSeenReplayCouples {
    const CODE: CounterCodex = CounterCodex::FirstSeenReplayCouples;

    fn new(value: Vec<FirstSeenReplayCouple>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<FirstSeenReplayCouple> {
        &self.value
    }
}

impl FirstSeenReplayCouples {
    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        counter: &Counter,
        cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], FirstSeenReplayCouples)> {
        let (rest, body) = count(
            tuple((
                Parsers::matter_parser(cold_code)?,
                Parsers::matter_parser(cold_code)?,
            )),
            counter.count() as usize,
        )(bytes)?;
        let body = body
            .into_iter()
            .map(|(firner, dater)| FirstSeenReplayCouple { firner, dater })
            .collect();

        return Ok((rest, FirstSeenReplayCouples { value: body }));
    }
}

#[derive(Debug, Clone, Default)]
pub struct FirstSeenReplayCouple {
    pub firner: Matter,
    pub dater: Matter,
}

impl FirstSeenReplayCouple {
    pub fn new(firner: Matter, dater: Matter) -> Self {
        Self { firner, dater }
    }
}


impl GroupItem for FirstSeenReplayCouple {
    fn qb64(&self) -> ParsideResult<String> {
        let mut out = String::new();
        out.push_str(&self.dater.qb64()?);
        out.push_str(&self.firner.qb64()?);
        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.dater.qb64b()?);
        out.extend_from_slice(&self.firner.qb64b()?);
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.dater.qb2()?);
        out.extend_from_slice(&self.firner.qb2()?);
        Ok(out)
    }
}
