use crate::error::*;
use crate::message::groups::{
    ControllerIdxSigsWrapper, FirstSeenReplayCouplesWrapper, NonTransReceiptCouplesWrapper,
    PathedMaterialQuadletsWrapper, SadPathSigGroupsWrapper, SadPathSigsWrapper,
    SealSourceCouplesWrapper, TransIdxSigGroupsWrapper, TransLastIdxSigGroupsWrapper,
    TransReceiptQuadruplesWrapper, WitnessIdxSigsWrapper,
};
use parside_core::{CesrGroup, CustomPayload, Message as ParsideMessage};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = MessageParsingResult)]
pub struct MessageParsingResult {
    pub(crate) rest: Vec<u8>,
    pub(crate) message: MessageWrapper,
}

#[wasm_bindgen(js_name = Message)]
#[derive(Clone)]
pub struct MessageWrapper {
    pub(crate) group: Option<CesrGroupWrapper>,
    pub(crate) payload: Option<CustomPayloadWrapper>,
}

#[wasm_bindgen(js_class = Message)]
impl MessageWrapper {
    pub fn from_stream_bytes(value: Vec<u8>) -> Result<MessageParsingResult> {
        let (rest, parsed_message) = ParsideMessage::from_stream_bytes(&value)
            .as_js()
            .map_err(JsValue::from)?;
        let parsed_result = MessageParsingResult {
            rest: rest.to_vec(),
            message: MessageWrapper::from(parsed_message),
        };
        Ok(parsed_result)
    }

    pub fn group(&self) -> Option<CesrGroupWrapper> {
        self.group.clone()
    }

    pub fn payload(&self) -> Option<CustomPayloadWrapper> {
        self.payload.clone()
    }
}

#[wasm_bindgen(js_class = MessageParsingResult)]
impl MessageParsingResult {
    pub fn message(&self) -> MessageWrapper {
        self.message.clone()
    }

    pub fn rest(&self) -> Vec<u8> {
        self.rest.clone()
    }
}

#[wasm_bindgen(js_name = CesrGroup)]
#[derive(Clone)]
pub struct CesrGroupWrapper(pub(crate) CesrGroup);

#[wasm_bindgen(js_name = CustomPayload)]
#[derive(Clone)]
pub struct CustomPayloadWrapper(pub(crate) CustomPayload);

#[wasm_bindgen(js_class = CesrGroup)]
impl CesrGroupWrapper {
    #[wasm_bindgen(js_name = controllerIdxSigs)]
    pub fn controller_idx_sigs(&self) -> Option<ControllerIdxSigsWrapper> {
        match &self.0 {
            CesrGroup::ControllerIdxSigsVariant { value } => {
                Some(ControllerIdxSigsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = firstSeenReplayCouples)]
    pub fn first_seen_replay_couples(&self) -> Option<FirstSeenReplayCouplesWrapper> {
        match &self.0 {
            CesrGroup::FirstSeenReplayCouplesVariant { value } => {
                Some(FirstSeenReplayCouplesWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = nonTransReceipt)]
    pub fn non_trans_receipt_couples(&self) -> Option<NonTransReceiptCouplesWrapper> {
        match &self.0 {
            CesrGroup::NonTransReceiptCouplesVariant { value } => {
                Some(NonTransReceiptCouplesWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = pathedMaterialQuadlets)]
    pub fn pathed_material_quadlets_wrapper(&self) -> Option<PathedMaterialQuadletsWrapper> {
        match &self.0 {
            CesrGroup::PathedMaterialQuadletsVariant { value } => {
                Some(PathedMaterialQuadletsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = sadPathSigs)]
    pub fn sad_path_sigs(&self) -> Option<SadPathSigsWrapper> {
        match &self.0 {
            CesrGroup::SadPathSigVariant { value } => Some(SadPathSigsWrapper(value.clone())),
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = sadPathSigGroups)]
    pub fn sad_path_sig_groups(&self) -> Option<SadPathSigGroupsWrapper> {
        match &self.0 {
            CesrGroup::SadPathSigGroupVariant { value } => {
                Some(SadPathSigGroupsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = sealSourceCouples)]
    pub fn seal_source_couples(&self) -> Option<SealSourceCouplesWrapper> {
        match &self.0 {
            CesrGroup::SealSourceCouplesVariant { value } => {
                Some(SealSourceCouplesWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = transIdxSigGroups)]
    pub fn trans_idx_sig_groups(&self) -> Option<TransIdxSigGroupsWrapper> {
        match &self.0 {
            CesrGroup::TransIdxSigGroupsVariant { value } => {
                Some(TransIdxSigGroupsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = transLastIdxSigGroups)]
    pub fn trans_last_idx_sig_groups(&self) -> Option<TransLastIdxSigGroupsWrapper> {
        match &self.0 {
            CesrGroup::TransLastIdxSigGroupsVariant { value } => {
                Some(TransLastIdxSigGroupsWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = transReceiptQuadruples)]
    pub fn trans_receipt_quadruples(&self) -> Option<TransReceiptQuadruplesWrapper> {
        match &self.0 {
            CesrGroup::TransReceiptQuadruplesVariant { value } => {
                Some(TransReceiptQuadruplesWrapper(value.clone()))
            }
            _ => None,
        }
    }

    #[wasm_bindgen(js_name = witnessIdxSigs)]
    pub fn witness_idx_sigs(&self) -> Option<WitnessIdxSigsWrapper> {
        match &self.0 {
            CesrGroup::WitnessIdxSigsVariant { value } => {
                Some(WitnessIdxSigsWrapper(value.clone()))
            }
            _ => None,
        }
    }
}

impl From<ParsideMessage> for MessageWrapper {
    fn from(message: ParsideMessage) -> Self {
        MessageWrapper {
            group: message
                .cesr_group()
                .ok()
                .cloned()
                .map(|group| CesrGroupWrapper(group)),
            payload: message
                .payload()
                .ok()
                .cloned()
                .map(|group| CustomPayloadWrapper(group)),
        }
    }
}
