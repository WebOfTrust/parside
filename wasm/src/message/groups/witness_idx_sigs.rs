use std::ops::Deref;

use crate::error::JsResult;
use crate::error::*;
use crate::utils::from_jsval;
use cesride_wasm::SigerWrapper;
use cesride_wasm::Wrap;
use js_sys::Array;
use parside_core::message::{GroupItem, WitnessIdxSig, WitnessIdxSigs};
use parside_core::Group;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = WitnessIdxSigs)]
pub struct WitnessIdxSigsWrapper(pub(crate) WitnessIdxSigs);

#[wasm_bindgen(js_class = WitnessIdxSigs)]
impl WitnessIdxSigsWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<WitnessIdxSigsWrapper> {
        let values = value
            .iter()
            .map(WitnessIdxSigWrapper::from_jsvalue)
            .collect::<Result<Vec<WitnessIdxSig>>>()?;
        Ok(WitnessIdxSigsWrapper(WitnessIdxSigs::new(values)))
    }

    pub fn size(&self) -> usize {
        self.0.value.len()
    }

    pub fn values(&self) -> Array {
        self.0
            .value
            .iter()
            .map(|value| WitnessIdxSigWrapper(value.clone()))
            .map(JsValue::from)
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<WitnessIdxSigWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(WitnessIdxSigWrapper(value.clone())),
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

#[wasm_bindgen(js_name = WitnessIdxSig)]
pub struct WitnessIdxSigWrapper(pub(crate) WitnessIdxSig);

#[wasm_bindgen(js_class = WitnessIdxSig)]
impl WitnessIdxSigWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(siger: SigerWrapper) -> WitnessIdxSigWrapper {
        WitnessIdxSigWrapper(WitnessIdxSig::new((*siger).clone()))
    }

    pub fn value(&self) -> SigerWrapper {
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

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<WitnessIdxSig> {
        from_jsval::<WitnessIdxSigWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for WitnessIdxSigWrapper {
    type Target = WitnessIdxSig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
