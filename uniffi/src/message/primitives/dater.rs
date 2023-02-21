use parside::error::{ParsideError, ParsideResult};
use parside::{Dater, Matter};

pub fn dater_new_with_code_and_raw(code: &str, raw: &[u8]) -> ParsideResult<Dater> {
    Dater::new_with_code_and_raw(code, raw).map_err(ParsideError::from)
}

pub fn dater_new_with_qb64(qb64: &str) -> ParsideResult<Dater> {
    Dater::new_with_qb64(qb64).map_err(ParsideError::from)
}

pub fn dater_new_with_qb64b(qb64b: &[u8]) -> ParsideResult<Dater> {
    Dater::new_with_qb64b(qb64b).map_err(ParsideError::from)
}

pub fn dater_new_with_qb2(qb2: &[u8]) -> ParsideResult<Dater> {
    Dater::new_with_qb2(qb2).map_err(ParsideError::from)
}

pub fn dater_code(dater: &Dater) -> String {
    dater.code()
}

pub fn dater_size(dater: &Dater) -> u32 {
    dater.size()
}

pub fn dater_raw(dater: &Dater) -> Vec<u8> {
    dater.raw()
}

pub fn dater_qb64(dater: &Dater) -> ParsideResult<String> {
    dater.qb64().map_err(ParsideError::from)
}

pub fn dater_qb64b(dater: &Dater) -> ParsideResult<Vec<u8>> {
    dater.qb64b().map_err(ParsideError::from)
}

pub fn dater_qb2(dater: &Dater) -> ParsideResult<Vec<u8>> {
    dater.qb2().map_err(ParsideError::from)
}
