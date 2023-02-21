use parside::error::ParsideResult;
pub use parside::message::groups::{SadPathSig, SadPathSigs};
use parside::Group;
use parside::Siger;

pub fn sad_path_sig_create(siger: Siger) -> SadPathSig {
    SadPathSig::new(siger)
}

pub fn sad_path_sigs_create(value: Vec<SadPathSig>) -> SadPathSigs {
    SadPathSigs::new(value)
}

pub fn sad_path_sigs_qb64(sad_path_sig: &SadPathSigs) -> ParsideResult<String> {
    sad_path_sig.qb64()
}

pub fn sad_path_sigs_qb64b(sad_path_sig: &SadPathSigs) -> ParsideResult<Vec<u8>> {
    sad_path_sig.qb64b()
}

pub fn sad_path_sigs_qb2(sad_path_sig: &SadPathSigs) -> ParsideResult<Vec<u8>> {
    sad_path_sig.qb2()
}
