use parside::error::{ParsideError, ParsideResult};
use parside::{Matter, Saider};

pub fn saider_new_with_qb64(qb64: &str) -> ParsideResult<Saider> {
    Saider::new_with_qb64(qb64).map_err(ParsideError::from)
}

pub fn saider_new_with_qb64b(qb64b: &[u8]) -> ParsideResult<Saider> {
    Saider::new_with_qb64b(qb64b).map_err(ParsideError::from)
}

pub fn saider_new_with_qb2(qb2: &[u8]) -> ParsideResult<Saider> {
    Saider::new_with_qb2(qb2).map_err(ParsideError::from)
}

pub fn saider_code(saider: &Saider) -> String {
    saider.code()
}

pub fn saider_size(saider: &Saider) -> u32 {
    saider.size()
}

pub fn saider_raw(saider: &Saider) -> Vec<u8> {
    saider.raw()
}

pub fn saider_qb64(saider: &Saider) -> ParsideResult<String> {
    saider.qb64().map_err(ParsideError::from)
}

pub fn saider_qb64b(saider: &Saider) -> ParsideResult<Vec<u8>> {
    saider.qb64b().map_err(ParsideError::from)
}

pub fn saider_qb2(saider: &Saider) -> ParsideResult<Vec<u8>> {
    saider.qb2().map_err(ParsideError::from)
}
