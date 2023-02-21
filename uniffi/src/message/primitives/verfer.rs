use parside::error::{ParsideError, ParsideResult};
use parside::{Matter, Verfer};

pub fn verfer_new_with_code_and_raw(code: &str, raw: &[u8]) -> ParsideResult<Verfer> {
    Verfer::new_with_code_and_raw(code, raw).map_err(ParsideError::from)
}

pub fn verfer_new_with_qb64(qb64: &str) -> ParsideResult<Verfer> {
    Verfer::new_with_qb64(qb64).map_err(ParsideError::from)
}

pub fn verfer_new_with_qb64b(qb64b: &[u8]) -> ParsideResult<Verfer> {
    Verfer::new_with_qb64b(qb64b).map_err(ParsideError::from)
}

pub fn verfer_new_with_qb2(qb2: &[u8]) -> ParsideResult<Verfer> {
    Verfer::new_with_qb2(qb2).map_err(ParsideError::from)
}

pub fn verfer_code(verfer: &Verfer) -> String {
    verfer.code()
}

pub fn verfer_size(verfer: &Verfer) -> u32 {
    verfer.size()
}

pub fn verfer_raw(verfer: &Verfer) -> Vec<u8> {
    verfer.raw()
}

pub fn verfer_qb64(verfer: &Verfer) -> ParsideResult<String> {
    verfer.qb64().map_err(ParsideError::from)
}

pub fn verfer_qb64b(verfer: &Verfer) -> ParsideResult<Vec<u8>> {
    verfer.qb64b().map_err(ParsideError::from)
}

pub fn verfer_qb2(verfer: &Verfer) -> ParsideResult<Vec<u8>> {
    verfer.qb2().map_err(ParsideError::from)
}
