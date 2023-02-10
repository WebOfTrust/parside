use parside::error::ParsideResult;
use parside::Matter;
pub use parside::message::groups::{
    SadPathSigGroup,
};

pub fn sad_path_sig_group_create(value: Vec<Matter>) -> SadPathSigGroup {
    SadPathSigGroup::new(value)
}

pub fn sad_path_sig_group_qb64(sad_path_sig_group: &SadPathSigGroup) -> ParsideResult<String> {
    sad_path_sig_group.qb64()
}

pub fn sad_path_sig_group_qb64b(transferable_indexed_signatures_groups: &SadPathSigGroup) -> ParsideResult<Vec<u8>> {
    transferable_indexed_signatures_groups.qb64b()
}

pub fn sad_path_sig_group_qb2(transferable_indexed_signatures_groups: &SadPathSigGroup) -> ParsideResult<Vec<u8>> {
    transferable_indexed_signatures_groups.qb2()
}
