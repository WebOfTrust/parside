use parside::error::{ParsideError, ParsideResult};
use parside::{Matter, Prefixer};

pub fn prefixer_new_with_code_and_raw(code: &str, raw: &[u8]) -> ParsideResult<Prefixer> {
    Prefixer::new_with_code_and_raw(code, raw).map_err(ParsideError::from)
}

pub fn prefixer_new_with_qb64(qb64: &str) -> ParsideResult<Prefixer> {
    Prefixer::new_with_qb64(qb64).map_err(ParsideError::from)
}

pub fn prefixer_new_with_qb64b(qb64b: &[u8]) -> ParsideResult<Prefixer> {
    Prefixer::new_with_qb64b(qb64b).map_err(ParsideError::from)
}

pub fn prefixer_new_with_qb2(qb2: &[u8]) -> ParsideResult<Prefixer> {
    Prefixer::new_with_qb2(qb2).map_err(ParsideError::from)
}

pub fn prefixer_code(prefixer: &Prefixer) -> String {
    prefixer.code()
}

pub fn prefixer_size(prefixer: &Prefixer) -> u32 {
    prefixer.size()
}

pub fn prefixer_raw(prefixer: &Prefixer) -> Vec<u8> {
    prefixer.raw()
}

pub fn prefixer_qb64(prefixer: &Prefixer) -> ParsideResult<String> {
    prefixer.qb64().map_err(ParsideError::from)
}

pub fn prefixer_qb64b(prefixer: &Prefixer) -> ParsideResult<Vec<u8>> {
    prefixer.qb64b().map_err(ParsideError::from)
}

pub fn prefixer_qb2(prefixer: &Prefixer) -> ParsideResult<Vec<u8>> {
    prefixer.qb2().map_err(ParsideError::from)
}
