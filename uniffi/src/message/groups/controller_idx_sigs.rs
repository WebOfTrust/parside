use parside::error::ParsideResult;
use parside::Matter;
pub use parside::message::groups::ControllerIdxSigs;

pub fn controller_idx_sigs_create(value: Vec<Matter>) -> ControllerIdxSigs {
    ControllerIdxSigs::new(value)
}

pub fn controller_idx_sigs_qb64(controller_idx_sigs: &ControllerIdxSigs) -> ParsideResult<String> {
    controller_idx_sigs.qb64()
}

pub fn controller_idx_sigs_qb64b(controller_idx_sigs: &ControllerIdxSigs) -> ParsideResult<Vec<u8>> {
    controller_idx_sigs.qb64b()
}

pub fn controller_idx_sigs_qb2(controller_idx_sigs: &ControllerIdxSigs) -> ParsideResult<Vec<u8>> {
    controller_idx_sigs.qb2()
}
