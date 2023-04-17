use crate::error::*;
use crate::utils::from_jsval;
use parside_core::CesrGroup;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

pub mod attached_material_quadlets;
pub mod controller_idx_sigs;
pub mod first_seen_replay_couples;
pub mod non_trans_receipt_couples;
pub mod pathed_material_quadlets;
pub mod sad_path_sig;
pub mod sad_path_sig_group;
pub mod seal_source_couples;
pub mod trans_idx_sig_groups;
pub mod trans_last_idx_sig_groups;
pub mod trans_receipt_quadruples;
pub mod witness_idx_sigs;

pub use self::attached_material_quadlets::AttachedMaterialQuadletsWrapper;
pub use self::controller_idx_sigs::{ControllerIdxSigWrapper, ControllerIdxSigsWrapper};
pub use self::first_seen_replay_couples::{
    FirstSeenReplayCoupleWrapper, FirstSeenReplayCouplesWrapper,
};
pub use self::non_trans_receipt_couples::{
    NonTransReceiptCoupleWrapper, NonTransReceiptCouplesWrapper,
};
pub use self::pathed_material_quadlets::{
    PathedMaterialQuadletWrapper, PathedMaterialQuadletsWrapper,
};
pub use self::sad_path_sig::{SadPathSigWrapper, SadPathSigsWrapper};
pub use self::sad_path_sig_group::{SadPathSigGroupWrapper, SadPathSigGroupsWrapper};
pub use self::seal_source_couples::{SealSourceCoupleWrapper, SealSourceCouplesWrapper};
pub use self::trans_idx_sig_groups::{TransIdxSigGroupWrapper, TransIdxSigGroupsWrapper};
pub use self::trans_last_idx_sig_groups::{
    TransLastIdxSigGroupWrapper, TransLastIdxSigGroupsWrapper,
};
pub use self::trans_receipt_quadruples::{
    TransReceiptQuadrupleWrapper, TransReceiptQuadruplesWrapper,
};
pub use self::witness_idx_sigs::{WitnessIdxSigWrapper, WitnessIdxSigsWrapper};

#[wasm_bindgen(js_name = CesrGroup)]
#[derive(Clone)]
pub struct CesrGroupWrapper(pub(crate) CesrGroup);

#[wasm_bindgen(js_class = CesrGroup)]
impl CesrGroupWrapper {
    pub fn new_with_attached_material_quadlets(
        attached_material_quadlets: AttachedMaterialQuadletsWrapper,
    ) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::AttachedMaterialQuadletsVariant {
            value: attached_material_quadlets.0,
        })
    }

    pub fn new_with_controller_idx_sigs(
        controller_idx_sigs: ControllerIdxSigsWrapper,
    ) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::ControllerIdxSigsVariant {
            value: controller_idx_sigs.0,
        })
    }

    pub fn new_with_first_seen_replay_couples(
        first_seen_replay_couples: FirstSeenReplayCouplesWrapper,
    ) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::FirstSeenReplayCouplesVariant {
            value: first_seen_replay_couples.0,
        })
    }

    pub fn new_with_non_trans_receipt_couples(
        non_trans_receipt_couples: NonTransReceiptCouplesWrapper,
    ) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::NonTransReceiptCouplesVariant {
            value: non_trans_receipt_couples.0,
        })
    }

    pub fn new_with_pathed_material_quadlets(
        pathed_material_quadlets: PathedMaterialQuadletsWrapper,
    ) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::PathedMaterialQuadletsVariant {
            value: pathed_material_quadlets.0,
        })
    }

    pub fn new_with_sad_path_sig(sad_path_sig: SadPathSigsWrapper) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::SadPathSigVariant {
            value: sad_path_sig.0,
        })
    }

    pub fn new_with_sad_path_sig_group(
        sad_path_sig_group: SadPathSigGroupsWrapper,
    ) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::SadPathSigGroupVariant {
            value: sad_path_sig_group.0,
        })
    }

    pub fn new_with_seal_source_couples(
        seal_source_couples: SealSourceCouplesWrapper,
    ) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::SealSourceCouplesVariant {
            value: seal_source_couples.0,
        })
    }

    pub fn new_with_trans_idx_sig_groups(
        trans_idx_sig_groups: TransIdxSigGroupsWrapper,
    ) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::TransIdxSigGroupsVariant {
            value: trans_idx_sig_groups.0,
        })
    }

    pub fn new_with_trans_last_idx_sig_groups(
        trans_last_idx_sig_groups: TransLastIdxSigGroupsWrapper,
    ) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::TransLastIdxSigGroupsVariant {
            value: trans_last_idx_sig_groups.0,
        })
    }

    pub fn new_with_trans_receipt_quadruples(
        trans_receipt_quadruples: TransReceiptQuadruplesWrapper,
    ) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::TransReceiptQuadruplesVariant {
            value: trans_receipt_quadruples.0,
        })
    }

    pub fn new_with_witness_idx_sigs(witness_idx_sigs: WitnessIdxSigsWrapper) -> CesrGroupWrapper {
        CesrGroupWrapper(CesrGroup::WitnessIdxSigsVariant {
            value: witness_idx_sigs.0,
        })
    }

    #[wasm_bindgen(js_name = controllerIdxSigs, getter)]
    pub fn controller_idx_sigs(&self) -> Option<ControllerIdxSigsWrapper> {
        match &self.0 {
            CesrGroup::ControllerIdxSigsVariant { value } => {
                Some(ControllerIdxSigsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = firstSeenReplayCouples, getter)]
    pub fn first_seen_replay_couples(&self) -> Option<FirstSeenReplayCouplesWrapper> {
        match &self.0 {
            CesrGroup::FirstSeenReplayCouplesVariant { value } => {
                Some(FirstSeenReplayCouplesWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = nonTransReceipt, getter)]
    pub fn non_trans_receipt_couples(&self) -> Option<NonTransReceiptCouplesWrapper> {
        match &self.0 {
            CesrGroup::NonTransReceiptCouplesVariant { value } => {
                Some(NonTransReceiptCouplesWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = pathedMaterialQuadlets, getter)]
    pub fn pathed_material_quadlets_wrapper(&self) -> Option<PathedMaterialQuadletsWrapper> {
        match &self.0 {
            CesrGroup::PathedMaterialQuadletsVariant { value } => {
                Some(PathedMaterialQuadletsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = sadPathSigs, getter)]
    pub fn sad_path_sigs(&self) -> Option<SadPathSigsWrapper> {
        match &self.0 {
            CesrGroup::SadPathSigVariant { value } => Some(SadPathSigsWrapper(value.clone())),
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = sadPathSigGroups, getter)]
    pub fn sad_path_sig_groups(&self) -> Option<SadPathSigGroupsWrapper> {
        match &self.0 {
            CesrGroup::SadPathSigGroupVariant { value } => {
                Some(SadPathSigGroupsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = sealSourceCouples, getter)]
    pub fn seal_source_couples(&self) -> Option<SealSourceCouplesWrapper> {
        match &self.0 {
            CesrGroup::SealSourceCouplesVariant { value } => {
                Some(SealSourceCouplesWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = transIdxSigGroups, getter)]
    pub fn trans_idx_sig_groups(&self) -> Option<TransIdxSigGroupsWrapper> {
        match &self.0 {
            CesrGroup::TransIdxSigGroupsVariant { value } => {
                Some(TransIdxSigGroupsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = transLastIdxSigGroups, getter)]
    pub fn trans_last_idx_sig_groups(&self) -> Option<TransLastIdxSigGroupsWrapper> {
        match &self.0 {
            CesrGroup::TransLastIdxSigGroupsVariant { value } => {
                Some(TransLastIdxSigGroupsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = transReceiptQuadruples, getter)]
    pub fn trans_receipt_quadruples(&self) -> Option<TransReceiptQuadruplesWrapper> {
        match &self.0 {
            CesrGroup::TransReceiptQuadruplesVariant { value } => {
                Some(TransReceiptQuadruplesWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = witnessIdxSigs, getter)]
    pub fn witness_idx_sigs(&self) -> Option<WitnessIdxSigsWrapper> {
        match &self.0 {
            CesrGroup::WitnessIdxSigsVariant { value } => {
                Some(WitnessIdxSigsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<CesrGroup> {
        from_jsval::<CesrGroupWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for CesrGroupWrapper {
    type Target = CesrGroup;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
