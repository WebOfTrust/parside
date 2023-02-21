use parside::error::{ParsideError, ParsideResult};
use parside::{Cigar, Matter, Verfer};

pub fn cigar_new_with_code_and_raw(
    verfer: &Verfer,
    code: &str,
    raw: &[u8],
) -> ParsideResult<Cigar> {
    Cigar::new_with_code_and_raw(verfer, code, raw).map_err(ParsideError::from)
}

pub fn cigar_new_with_qb64(verfer: &Verfer, qb64: &str) -> ParsideResult<Cigar> {
    Cigar::new_with_qb64(verfer, qb64).map_err(ParsideError::from)
}

pub fn cigar_new_with_qb64b(verfer: &Verfer, qb64b: &[u8]) -> ParsideResult<Cigar> {
    Cigar::new_with_qb64b(verfer, qb64b).map_err(ParsideError::from)
}

pub fn cigar_new_with_qb2(verfer: &Verfer, qb2: &[u8]) -> ParsideResult<Cigar> {
    Cigar::new_with_qb2(verfer, qb2).map_err(ParsideError::from)
}

pub fn cigar_code(cigar: &Cigar) -> String {
    cigar.code()
}

pub fn cigar_size(cigar: &Cigar) -> u32 {
    cigar.size()
}

pub fn cigar_raw(cigar: &Cigar) -> Vec<u8> {
    cigar.raw()
}

pub fn cigar_qb64(cigar: &Cigar) -> ParsideResult<String> {
    cigar.qb64().map_err(ParsideError::from)
}

pub fn cigar_qb64b(cigar: &Cigar) -> ParsideResult<Vec<u8>> {
    cigar.qb64b().map_err(ParsideError::from)
}

pub fn cigar_qb2(cigar: &Cigar) -> ParsideResult<Vec<u8>> {
    cigar.qb2().map_err(ParsideError::from)
}
