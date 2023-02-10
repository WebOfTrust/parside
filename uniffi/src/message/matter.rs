pub use parside::error::{ParsideError, ParsideResult};
pub use parside::Matter;

pub fn matter_new_with_code_and_raw(code: &str, raw: &[u8], raw_size: u64) -> ParsideResult<Matter> {
    Matter::new_with_code_and_raw(code, raw, raw_size as usize).map_err(ParsideError::from)
}

pub fn matter_new_with_qb64(qb64: &str) -> ParsideResult<Matter> {
    Matter::new_with_qb64(qb64).map_err(ParsideError::from)
}

pub fn matter_new_with_qb64b(qb64b: &[u8]) -> ParsideResult<Matter> {
    Matter::new_with_qb64b(qb64b).map_err(ParsideError::from)
}

pub fn matter_new_with_qb2(qb2: &[u8]) -> ParsideResult<Matter> {
    Matter::new_with_qb2(qb2).map_err(ParsideError::from)
}

pub fn matter_code(matter: &Matter) -> String {
    matter.code()
}

pub fn matter_size(matter: &Matter) -> u32 {
    matter.size()
}

pub fn matter_raw(matter: &Matter) -> Vec<u8> {
    matter.raw()
}

pub fn matter_qb64(matter: &Matter) -> ParsideResult<String> {
    matter.qb64().map_err(ParsideError::from)
}

pub fn matter_qb64b(matter: &Matter) -> ParsideResult<Vec<u8>> {
    matter.qb64b().map_err(ParsideError::from)
}

pub fn matter_qb2(matter: &Matter) -> ParsideResult<Vec<u8>> {
    matter.qb2().map_err(ParsideError::from)
}

pub fn matter_full_size(matter: &Matter) -> ParsideResult<u32> {
    matter.full_size().map_err(ParsideError::from)
}