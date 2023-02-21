use parside::error::{ParsideError, ParsideResult};
use parside::{Diger, Matter};

pub fn diger_new_with_code_and_raw(code: &str, raw: &[u8]) -> ParsideResult<Diger> {
    Diger::new_with_code_and_raw(code, raw).map_err(ParsideError::from)
}

pub fn diger_new_with_qb64(qb64: &str) -> ParsideResult<Diger> {
    Diger::new_with_qb64(qb64).map_err(ParsideError::from)
}

pub fn diger_new_with_qb64b(qb64b: &[u8]) -> ParsideResult<Diger> {
    Diger::new_with_qb64b(qb64b).map_err(ParsideError::from)
}

pub fn diger_new_with_qb2(qb2: &[u8]) -> ParsideResult<Diger> {
    Diger::new_with_qb2(qb2).map_err(ParsideError::from)
}

pub fn diger_code(diger: &Diger) -> String {
    diger.code()
}

pub fn diger_size(diger: &Diger) -> u32 {
    diger.size()
}

pub fn diger_raw(diger: &Diger) -> Vec<u8> {
    diger.raw()
}

pub fn diger_qb64(diger: &Diger) -> ParsideResult<String> {
    diger.qb64().map_err(ParsideError::from)
}

pub fn diger_qb64b(diger: &Diger) -> ParsideResult<Vec<u8>> {
    diger.qb64b().map_err(ParsideError::from)
}

pub fn diger_qb2(diger: &Diger) -> ParsideResult<Vec<u8>> {
    diger.qb2().map_err(ParsideError::from)
}
