use crate::error::JsResult;
use crate::error::*;
use crate::utils::from_jsval;
use cesride_wasm::SigerWrapper;
use cesride_wasm::Wrap;
use js_sys::Array;
use parside_core::message::{GroupItem, SadPathSigGroup, SadPathSigGroups};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = SadPathSigGroups)]
pub struct SadPathSigGroupsWrapper(pub(crate) SadPathSigGroups);

#[wasm_bindgen(js_class = SadPathSigGroups)]
impl SadPathSigGroupsWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<SadPathSigGroupsWrapper> {
        let values = value
            .iter()
            .map(SadPathSigGroupWrapper::from_jsvalue)
            .collect::<Result<Vec<SadPathSigGroup>>>()?;
        Ok(SadPathSigGroupsWrapper(SadPathSigGroups::new(values)))
    }

    pub fn size(&self) -> usize {
        self.0.value.len()
    }

    pub fn values(&self) -> Array {
        self.0
            .value
            .iter()
            .map(|value| JsValue::from(SadPathSigGroupWrapper(value.clone())))
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<SadPathSigGroupWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(SadPathSigGroupWrapper(value.clone())),
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

#[wasm_bindgen(js_name = SadPathSigGroup)]
pub struct SadPathSigGroupWrapper(pub(crate) SadPathSigGroup);

#[wasm_bindgen(js_class = SadPathSigGroup)]
impl SadPathSigGroupWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(siger: SigerWrapper) -> SadPathSigGroupWrapper {
        SadPathSigGroupWrapper(SadPathSigGroup::new((*siger).clone()))
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

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<SadPathSigGroup> {
        from_jsval::<SadPathSigGroupWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for SadPathSigGroupWrapper {
    type Target = SadPathSigGroup;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
