use parside::error::ParsideResult;
pub use parside::message::groups::{ControllerIdxSig, ControllerIdxSigs};
use parside::Group;
use parside::Siger;

pub fn controller_idx_sig_create(siger: Siger) -> ControllerIdxSig {
    ControllerIdxSig::new(siger)
}

pub fn controller_idx_sigs_create(value: Vec<ControllerIdxSig>) -> ControllerIdxSigs {
    ControllerIdxSigs::new(value)
}

pub fn controller_idx_sigs_qb64(controller_idx_sigs: &ControllerIdxSigs) -> ParsideResult<String> {
    controller_idx_sigs.qb64()
}

pub fn controller_idx_sigs_qb64b(
    controller_idx_sigs: &ControllerIdxSigs,
) -> ParsideResult<Vec<u8>> {
    controller_idx_sigs.qb64b()
}

pub fn controller_idx_sigs_qb2(controller_idx_sigs: &ControllerIdxSigs) -> ParsideResult<Vec<u8>> {
    controller_idx_sigs.qb2()
}
