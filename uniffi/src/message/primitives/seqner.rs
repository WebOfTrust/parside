use parside::error::{ParsideError, ParsideResult};
use parside::{Matter, Seqner};

pub fn seqner_new_with_code_and_raw(code: &str, raw: &[u8]) -> ParsideResult<Seqner> {
    Seqner::new_with_code_and_raw(code, raw).map_err(ParsideError::from)
}

pub fn seqner_new_with_qb64(qb64: &str) -> ParsideResult<Seqner> {
    Seqner::new_with_qb64(qb64).map_err(ParsideError::from)
}

pub fn seqner_new_with_qb64b(qb64b: &[u8]) -> ParsideResult<Seqner> {
    Seqner::new_with_qb64b(qb64b).map_err(ParsideError::from)
}

pub fn seqner_new_with_qb2(qb2: &[u8]) -> ParsideResult<Seqner> {
    Seqner::new_with_qb2(qb2).map_err(ParsideError::from)
}

pub fn seqner_code(seqner: &Seqner) -> String {
    seqner.code()
}

pub fn seqner_size(seqner: &Seqner) -> u32 {
    seqner.size()
}

pub fn seqner_raw(seqner: &Seqner) -> Vec<u8> {
    seqner.raw()
}

pub fn seqner_qb64(seqner: &Seqner) -> ParsideResult<String> {
    seqner.qb64().map_err(ParsideError::from)
}

pub fn seqner_qb64b(seqner: &Seqner) -> ParsideResult<Vec<u8>> {
    seqner.qb64b().map_err(ParsideError::from)
}

pub fn seqner_qb2(seqner: &Seqner) -> ParsideResult<Vec<u8>> {
    seqner.qb2().map_err(ParsideError::from)
}
