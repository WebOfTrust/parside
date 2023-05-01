use crate::error::{ParsideError, ParsideResult};
use cesride::{counter::Codex, Counter};

pub trait Group<T: GroupItem> {
    /// Code associated with the group
    const CODE: &'static str;

    /// Group constructor
    fn new(value: Vec<T>) -> Self;

    /// Get group values
    fn value(&self) -> &Vec<T>;

    /// Get group counter
    fn counter(&self) -> ParsideResult<Counter> {
        Counter::new_with_code_and_count(Self::CODE, self.count()?).map_err(ParsideError::from)
    }

    /// Get count of items in the group
    fn count(&self) -> ParsideResult<u32> {
        match Self::CODE {
            Codex::AttachedMaterialQuadlets | Codex::BigAttachedMaterialQuadlets => {
                Ok(self.full_size()? as u32 / 4 - 1)
            }
            _ => Ok(self.value().len() as u32),
        }
    }

    /// Get qb64 representation of the group
    fn qb64(&self) -> ParsideResult<String> {
        let mut out = self.counter()?.qb64()?;
        for value in self.value().iter() {
            out.push_str(&value.qb64()?);
        }
        Ok(out)
    }

    /// Get qb64b representation of the group
    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter()?.qb64b()?;
        for value in self.value().iter() {
            out.extend_from_slice(&value.qb64b()?);
        }
        Ok(out)
    }

    /// Get qb2 representation of the group
    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        let mut out = self.counter()?.qb2()?;
        for value in self.value().iter() {
            out.extend_from_slice(&value.qb2()?);
        }
        Ok(out)
    }

    /// Get total size of the group
    fn full_size(&self) -> ParsideResult<usize> {
        let mut size = match Self::CODE {
            Codex::AttachedMaterialQuadlets | Codex::BigAttachedMaterialQuadlets => 4usize,
            _ => self.counter()?.full_size()?,
        };

        for value in self.value().iter() {
            size += value.full_size()?;
        }
        Ok(size)
    }
}

pub trait GroupItem {
    /// Get qb64 representation of the group item
    fn qb64(&self) -> ParsideResult<String>;

    /// Get qb64b representation of the group item
    fn qb64b(&self) -> ParsideResult<Vec<u8>>;

    /// Get qb2 representation of the group item
    fn qb2(&self) -> ParsideResult<Vec<u8>>;

    /// Get total size of the group item
    fn full_size(&self) -> ParsideResult<usize>;
}
