use parside::error::{ParsideError, ParsideResult};
use parside::{Indexer, Siger};

pub fn siger_new_with_code_and_raw(
    code: &str,
    raw: &[u8],
    index: u32,
    ondex: Option<u32>,
) -> ParsideResult<Siger> {
    Siger::new_with_code_and_raw(code, raw, index, ondex).map_err(ParsideError::from)
}

pub fn siger_new_with_qb64(qb64: &str) -> ParsideResult<Siger> {
    Siger::new_with_qb64(qb64).map_err(ParsideError::from)
}

pub fn siger_new_with_qb64b(qb64b: &[u8]) -> ParsideResult<Siger> {
    Siger::new_with_qb64b(qb64b).map_err(ParsideError::from)
}

pub fn siger_new_with_qb2(qb2: &[u8]) -> ParsideResult<Siger> {
    Siger::new_with_qb2(qb2).map_err(ParsideError::from)
}

pub fn siger_code(siger: &Siger) -> String {
    siger.code()
}

pub fn siger_raw(siger: &Siger) -> Vec<u8> {
    siger.raw()
}

pub fn siger_qb64(siger: &Siger) -> ParsideResult<String> {
    siger.qb64().map_err(ParsideError::from)
}

pub fn siger_qb64b(siger: &Siger) -> ParsideResult<Vec<u8>> {
    siger.qb64b().map_err(ParsideError::from)
}

pub fn siger_qb2(siger: &Siger) -> ParsideResult<Vec<u8>> {
    siger.qb2().map_err(ParsideError::from)
}
