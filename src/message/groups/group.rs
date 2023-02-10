use crate::error::{ParsideError, ParsideResult};
use cesride::{Counter, Matter};
use cesride::counter::Codex;

pub trait Group<T: GroupItem> {
    const CODE: Codex;

    fn new(value: Vec<T>) -> Self;

    fn value(&self) -> &Vec<T>;

    fn counter(&self) -> Counter {
        Counter::new(&Self::CODE.code(), self.count())
    }

    fn count(&self) -> u32 {
        self.value().len() as u32
    }

    fn qb64(&self) -> ParsideResult<String> {
        let mut out = self.counter().qb64()?;
        for value in self.value().iter() {
            out.push_str(&value.qb64()?);
        }
        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb64b()?;
        for value in self.value().iter() {
            out.extend_from_slice(&value.qb64b()?);
        }
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter().qb2()?;
        for value in self.value().iter() {
            out.extend_from_slice(&value.qb2()?);
        }
        Ok(out)
    }
}

pub trait GroupItem {
    fn qb64(&self) -> ParsideResult<String>;
    fn qb64b(&self) -> ParsideResult<Vec<u8>>;
    fn qb2(&self) -> ParsideResult<Vec<u8>>;
}

impl GroupItem for Matter {
    fn qb64(&self) -> ParsideResult<String> {
        self.qb64().map_err(ParsideError::from)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        self.qb64b().map_err(ParsideError::from)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        self.qb2().map_err(ParsideError::from)
    }
}