use crate::error::JsResult;
use crate::error::*;
use crate::utils::from_jsval;
use cesride_wasm::SigerWrapper;
use cesride_wasm::Wrap;
use js_sys::Array;
use parside_core::message::{GroupItem, PathedMaterialQuadlet, PathedMaterialQuadlets};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = PathedMaterialQuadlets)]
pub struct PathedMaterialQuadletsWrapper(pub(crate) PathedMaterialQuadlets);

#[wasm_bindgen(js_class = PathedMaterialQuadlets)]
impl PathedMaterialQuadletsWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<PathedMaterialQuadletsWrapper> {
        let values = value
            .iter()
            .map(PathedMaterialQuadletWrapper::from_jsvalue)
            .collect::<Result<Vec<PathedMaterialQuadlet>>>()?;
        Ok(PathedMaterialQuadletsWrapper(PathedMaterialQuadlets::new(
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
            .map(|value| JsValue::from(PathedMaterialQuadletWrapper(value.clone())))
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<PathedMaterialQuadletWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(PathedMaterialQuadletWrapper(value.clone())),
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

#[wasm_bindgen(js_name = PathedMaterialQuadlet)]
pub struct PathedMaterialQuadletWrapper(pub(crate) PathedMaterialQuadlet);

#[wasm_bindgen(js_class = PathedMaterialQuadlet)]
impl PathedMaterialQuadletWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(siger: SigerWrapper) -> PathedMaterialQuadletWrapper {
        PathedMaterialQuadletWrapper(PathedMaterialQuadlet::new((*siger).clone()))
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

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<PathedMaterialQuadlet> {
        from_jsval::<PathedMaterialQuadletWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for PathedMaterialQuadletWrapper {
    type Target = PathedMaterialQuadlet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
