use crate::error::JsResult;
use crate::error::*;
use crate::utils::from_jsval;
use cesride_wasm::SigerWrapper;
use cesride_wasm::Wrap;
use js_sys::Array;
use parside_core::message::{GroupItem, SadPathSig, SadPathSigs};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = SadPathSigs)]
pub struct SadPathSigsWrapper(pub(crate) SadPathSigs);

#[wasm_bindgen(js_class = SadPathSigs)]
impl SadPathSigsWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<SadPathSigsWrapper> {
        let values = value
            .iter()
            .map(SadPathSigWrapper::from_jsvalue)
            .collect::<Result<Vec<SadPathSig>>>()?;
        Ok(SadPathSigsWrapper(SadPathSigs::new(values)))
    }

    pub fn size(&self) -> usize {
        self.0.value.len()
    }

    pub fn values(&self) -> Array {
        self.0
            .value
            .iter()
            .map(|value| JsValue::from(SadPathSigWrapper(value.clone())))
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<SadPathSigWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(SadPathSigWrapper(value.clone())),
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

#[wasm_bindgen(js_name = SadPathSig)]
pub struct SadPathSigWrapper(pub(crate) SadPathSig);

#[wasm_bindgen(js_class = SadPathSig)]
impl SadPathSigWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(siger: SigerWrapper) -> SadPathSigWrapper {
        SadPathSigWrapper(SadPathSig::new((*siger).clone()))
    }

    #[wasm_bindgen(getter)]
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

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<SadPathSig> {
        from_jsval::<SadPathSigWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for SadPathSigWrapper {
    type Target = SadPathSig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
