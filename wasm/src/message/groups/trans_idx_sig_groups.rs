use crate::error::JsResult;
use crate::error::*;
use crate::message::groups::ControllerIdxSigsWrapper;
use crate::utils::from_jsval;
use cesride_wasm::Wrap;
use cesride_wasm::{PrefixerWrapper, SaiderWrapper, SeqnerWrapper};
use js_sys::Array;
use parside_core::message::{GroupItem, TransIdxSigGroup, TransIdxSigGroups};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = TransIdxSigGroups)]
pub struct TransIdxSigGroupsWrapper(pub(crate) TransIdxSigGroups);

#[wasm_bindgen(js_class = TransIdxSigGroups)]
impl TransIdxSigGroupsWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<TransIdxSigGroupsWrapper> {
        let values = value
            .iter()
            .map(TransIdxSigGroupWrapper::from_jsvalue)
            .collect::<Result<Vec<TransIdxSigGroup>>>()?;
        Ok(TransIdxSigGroupsWrapper(TransIdxSigGroups::new(values)))
    }

    pub fn size(&self) -> usize {
        self.0.value.len()
    }

    pub fn values(&self) -> Array {
        self.0
            .value
            .iter()
            .map(|value| JsValue::from(TransIdxSigGroupWrapper(value.clone())))
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<TransIdxSigGroupWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(TransIdxSigGroupWrapper(value.clone())),
            _ => None,
        }
    }

    pub fn qb64(&self) -> Result<String> {
        self.0.qb64().as_js().map_err(JsValue::from)
    }

    pub fn qb64b(&self) -> Result<Vec<u8>> {
        self.0.qb64b().as_js().map_err(JsValue::from)
    }

    pub fn qb2(&self) -> Result<Vec<u8>> {
        self.0.qb2().as_js().map_err(JsValue::from)
    }
}

#[wasm_bindgen(js_name = TransIdxSigGroup)]
pub struct TransIdxSigGroupWrapper(pub(crate) TransIdxSigGroup);

#[wasm_bindgen(js_class = TransIdxSigGroup)]
impl TransIdxSigGroupWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(
        prefixer: PrefixerWrapper,
        seqner: SeqnerWrapper,
        saider: SaiderWrapper,
        isigers: ControllerIdxSigsWrapper,
    ) -> TransIdxSigGroupWrapper {
        TransIdxSigGroupWrapper(TransIdxSigGroup::new(
            (*prefixer).clone(),
            (*seqner).clone(),
            (*saider).clone(),
            (*isigers).clone(),
        ))
    }

    pub fn prefixer(&self) -> PrefixerWrapper {
        PrefixerWrapper::wrap(&self.0.prefixer)
    }
    pub fn seqner(&self) -> SeqnerWrapper {
        SeqnerWrapper::wrap(&self.0.seqner)
    }
    pub fn saider(&self) -> SaiderWrapper {
        SaiderWrapper::wrap(&self.0.saider)
    }
    pub fn isigers(&self) -> ControllerIdxSigsWrapper {
        ControllerIdxSigsWrapper::wrap(&self.0.isigers)
    }

    pub fn qb64(&self) -> Result<String> {
        self.0.qb64().as_js().map_err(JsValue::from)
    }

    pub fn qb64b(&self) -> Result<Vec<u8>> {
        self.0.qb64b().as_js().map_err(JsValue::from)
    }

    pub fn qb2(&self) -> Result<Vec<u8>> {
        self.0.qb2().as_js().map_err(JsValue::from)
    }

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<TransIdxSigGroup> {
        from_jsval::<TransIdxSigGroupWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for TransIdxSigGroupWrapper {
    type Target = TransIdxSigGroup;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
