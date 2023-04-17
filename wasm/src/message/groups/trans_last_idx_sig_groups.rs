use crate::error::JsResult;
use crate::error::*;
use crate::message::groups::ControllerIdxSigsWrapper;
use crate::utils::from_jsval;
use cesride_wasm::PrefixerWrapper;
use cesride_wasm::Wrap;
use js_sys::Array;
use parside_core::message::{GroupItem, TransLastIdxSigGroup, TransLastIdxSigGroups};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = TransLastIdxSigGroups)]
pub struct TransLastIdxSigGroupsWrapper(pub(crate) TransLastIdxSigGroups);

#[wasm_bindgen(js_class = TransLastIdxSigGroups)]
impl TransLastIdxSigGroupsWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<TransLastIdxSigGroupsWrapper> {
        let values = value
            .iter()
            .map(TransLastIdxSigGroupWrapper::from_jsvalue)
            .collect::<Result<Vec<TransLastIdxSigGroup>>>()?;
        Ok(TransLastIdxSigGroupsWrapper(TransLastIdxSigGroups::new(
            values,
        )))
    }

    pub fn size(&self) -> usize {
        self.0.value.len()
    }

    pub fn values(&self) -> Array {
        self.0
            .value
            .iter()
            .map(|value| JsValue::from(TransLastIdxSigGroupWrapper(value.clone())))
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<TransLastIdxSigGroupWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(TransLastIdxSigGroupWrapper(value.clone())),
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

#[wasm_bindgen(js_name = TransLastIdxSigGroup)]
pub struct TransLastIdxSigGroupWrapper(pub(crate) TransLastIdxSigGroup);

#[wasm_bindgen(js_class = TransLastIdxSigGroup)]
impl TransLastIdxSigGroupWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(
        prefixer: PrefixerWrapper,
        isigers: ControllerIdxSigsWrapper,
    ) -> TransLastIdxSigGroupWrapper {
        TransLastIdxSigGroupWrapper(TransLastIdxSigGroup::new(
            (*prefixer).clone(),
            (*isigers).clone(),
        ))
    }

    #[wasm_bindgen(getter)]
    pub fn prefixer(&self) -> PrefixerWrapper {
        PrefixerWrapper::wrap(&self.0.prefixer)
    }

    #[wasm_bindgen(getter)]
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

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<TransLastIdxSigGroup> {
        from_jsval::<TransLastIdxSigGroupWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for TransLastIdxSigGroupWrapper {
    type Target = TransLastIdxSigGroup;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
