use crate::error::{ParsideError, ParsideResult};
use cesride::Counter;

pub trait Group<T: GroupItem> {
    const CODE: &'static str;

    fn new(value: Vec<T>) -> Self;

    fn value(&self) -> &Vec<T>;

    fn counter(&self) -> ParsideResult<Counter> {
        Counter::new(Some(self.count()), None, Some(&Self::CODE), None, None, None)
            .map_err(ParsideError::from)
    }

    fn count(&self) -> u32 {
        self.value().len() as u32
    }

    fn qb64(&self) -> ParsideResult<String> {
        let mut out = self.counter()?.qb64()?;
        for value in self.value().iter() {
            out.push_str(&value.qb64()?);
        }
        Ok(out)
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter()?.qb64b()?;
        for value in self.value().iter() {
            out.extend_from_slice(&value.qb64b()?);
        }
        Ok(out)
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter()?.qb2()?;
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