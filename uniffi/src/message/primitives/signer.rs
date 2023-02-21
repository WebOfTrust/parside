use parside::error::{ParsideError, ParsideResult};
use parside::{Matter, Signer};

pub fn signer_new_with_code_and_raw(
    code: &str,
    raw: &[u8],
    transferable: bool,
) -> ParsideResult<Signer> {
    Signer::new_with_code_and_raw(code, raw, transferable).map_err(ParsideError::from)
}

pub fn signer_new_with_qb64(qb64: &str, transferable: bool) -> ParsideResult<Signer> {
    Signer::new_with_qb64(qb64, transferable).map_err(ParsideError::from)
}

pub fn signer_new_with_qb64b(qb64b: &[u8], transferable: bool) -> ParsideResult<Signer> {
    Signer::new_with_qb64b(qb64b, transferable).map_err(ParsideError::from)
}

pub fn signer_new_with_qb2(qb2: &[u8], transferable: bool) -> ParsideResult<Signer> {
    Signer::new_with_qb2(qb2, transferable).map_err(ParsideError::from)
}

pub fn signer_code(signer: &Signer) -> String {
    signer.code()
}

pub fn signer_size(signer: &Signer) -> u32 {
    signer.size()
}

pub fn signer_raw(signer: &Signer) -> Vec<u8> {
    signer.raw()
}

pub fn signer_qb64(signer: &Signer) -> ParsideResult<String> {
    signer.qb64().map_err(ParsideError::from)
}

pub fn signer_qb64b(signer: &Signer) -> ParsideResult<Vec<u8>> {
    signer.qb64b().map_err(ParsideError::from)
}

pub fn signer_qb2(signer: &Signer) -> ParsideResult<Vec<u8>> {
    signer.qb2().map_err(ParsideError::from)
}
