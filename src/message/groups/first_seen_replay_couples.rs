use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::groups::parsers::Parsers;
use cesride::{Counter, Matter};
use cesride::counter::Codex as CounterCodex;
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug, Clone, Default)]
pub struct FirstSeenReplayCouples {
    pub value: Vec<FirstSeenReplayCouple>,
}

impl FirstSeenReplayCouples {
    pub const CODE: CounterCodex = CounterCodex::FirstSeenReplayCouples;

    pub fn new(value: Vec<FirstSeenReplayCouple>) -> Self {
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
            out.push_str(&couple.dater.qb64()?);
            out.push_str(&couple.firner.qb64()?);
        }
        Ok(out)
    }

    pub fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.dater.qb64b()?);
            out.extend_from_slice(&couple.firner.qb64b()?);
        }
        Ok(out)
    }

    pub fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for couple in self.value.iter() {
            out.extend_from_slice(&couple.dater.qb2()?);
            out.extend_from_slice(&couple.firner.qb2()?);
        }
        Ok(out)
    }

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
