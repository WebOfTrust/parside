use crate::error::JsResult;
use crate::error::*;
use crate::utils::from_jsval;
use cesride_wasm::SigerWrapper;
use cesride_wasm::Wrap;
use js_sys::Array;
use parside_core::message::{ControllerIdxSig, ControllerIdxSigs, GroupItem};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = ControllerIdxSigs)]
pub struct ControllerIdxSigsWrapper(pub(crate) ControllerIdxSigs);

#[wasm_bindgen(js_class = ControllerIdxSigs)]
impl ControllerIdxSigsWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<ControllerIdxSigsWrapper> {
        let values = value
            .iter()
            .map(ControllerIdxSigWrapper::from_jsvalue)
            .collect::<Result<Vec<ControllerIdxSig>>>()?;
        Ok(ControllerIdxSigsWrapper(ControllerIdxSigs::new(values)))
    }

    pub fn size(&self) -> usize {
        self.0.value.len()
    }

    pub fn values(&self) -> Array {
        self.0
            .value
            .iter()
            .map(|value| JsValue::from(ControllerIdxSigWrapper(value.clone())))
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<ControllerIdxSigWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(ControllerIdxSigWrapper(value.clone())),
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

    pub(crate) fn wrap(isigers: &ControllerIdxSigs) -> ControllerIdxSigsWrapper {
        ControllerIdxSigsWrapper(isigers.clone())
    }
}

impl Deref for ControllerIdxSigsWrapper {
    type Target = ControllerIdxSigs;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[wasm_bindgen(js_name = ControllerIdxSig)]
pub struct ControllerIdxSigWrapper(pub(crate) ControllerIdxSig);

#[wasm_bindgen(js_class = ControllerIdxSig)]
impl ControllerIdxSigWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(siger: SigerWrapper) -> ControllerIdxSigWrapper {
        ControllerIdxSigWrapper(ControllerIdxSig::new((*siger).clone()))
    }

    pub fn siger(&self) -> SigerWrapper {
        SigerWrapper::wrap(&self.0.siger)
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

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<ControllerIdxSig> {
        from_jsval::<ControllerIdxSigWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for ControllerIdxSigWrapper {
    type Target = ControllerIdxSig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
