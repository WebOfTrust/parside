use parside::error::ParsideResult;
pub use parside::message::groups::{SadPathSigGroup, SadPathSigGroups};
use parside::Group;
use parside::Siger;

pub fn sad_path_sig_group_create(siger: Siger) -> SadPathSigGroup {
    SadPathSigGroup::new(siger)
}

pub fn sad_path_sig_groups_create(value: Vec<SadPathSigGroup>) -> SadPathSigGroups {
    SadPathSigGroups::new(value)
}

pub fn sad_path_sig_groups_qb64(sad_path_sig_group: &SadPathSigGroups) -> ParsideResult<String> {
    sad_path_sig_group.qb64()
}

pub fn sad_path_sig_groups_qb64b(
    transferable_indexed_signatures_groups: &SadPathSigGroups,
) -> ParsideResult<Vec<u8>> {
    transferable_indexed_signatures_groups.qb64b()
}

pub fn sad_path_sig_groups_qb2(
    transferable_indexed_signatures_groups: &SadPathSigGroups,
) -> ParsideResult<Vec<u8>> {
    transferable_indexed_signatures_groups.qb2()
}
