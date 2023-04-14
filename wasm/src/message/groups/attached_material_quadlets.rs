use crate::error::JsResult;
use crate::error::*;
use crate::message::message::CesrGroupWrapper;
use js_sys::Array;
use parside_core::message::{AttachedMaterialQuadlets, CesrGroup};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = AttachedMaterialQuadlets)]
pub struct AttachedMaterialQuadletsWrapper(pub(crate) AttachedMaterialQuadlets);

#[wasm_bindgen(js_class = AttachedMaterialQuadlets)]
impl AttachedMaterialQuadletsWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<AttachedMaterialQuadletsWrapper> {
        let values = value
            .iter()
            .map(CesrGroupWrapper::from_jsvalue)
            .collect::<Result<Vec<CesrGroup>>>()?;
        Ok(AttachedMaterialQuadletsWrapper(
            AttachedMaterialQuadlets::new(values),
        ))
    }

    pub fn size(&self) -> usize {
        self.0.value.len()
    }

    pub fn values(&self) -> Array {
        self.0
            .value
            .iter()
            .map(|value| JsValue::from(CesrGroupWrapper(value.clone())))
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<CesrGroupWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(CesrGroupWrapper(value.clone())),
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

impl Deref for AttachedMaterialQuadletsWrapper {
    type Target = AttachedMaterialQuadlets;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
