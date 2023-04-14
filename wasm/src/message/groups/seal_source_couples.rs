use crate::error::JsResult;
use crate::error::*;
use crate::utils::from_jsval;
use cesride_wasm::Wrap;
use cesride_wasm::{SaiderWrapper, SeqnerWrapper};
use js_sys::Array;
use parside_core::message::{GroupItem, SealSourceCouple, SealSourceCouples};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = SealSourceCouples)]
pub struct SealSourceCouplesWrapper(pub(crate) SealSourceCouples);

#[wasm_bindgen(js_class = SealSourceCouples)]
impl SealSourceCouplesWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<SealSourceCouplesWrapper> {
        let values = value
            .iter()
            .map(SealSourceCoupleWrapper::from_jsvalue)
            .collect::<Result<Vec<SealSourceCouple>>>()?;
        Ok(SealSourceCouplesWrapper(SealSourceCouples::new(values)))
    }

    pub fn size(&self) -> usize {
        self.0.value.len()
    }

    pub fn values(&self) -> Array {
        self.0
            .value
            .iter()
            .map(|value| JsValue::from(SealSourceCoupleWrapper(value.clone())))
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<SealSourceCoupleWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(SealSourceCoupleWrapper(value.clone())),
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

#[wasm_bindgen(js_name = SealSourceCouple)]
pub struct SealSourceCoupleWrapper(pub(crate) SealSourceCouple);

#[wasm_bindgen(js_class = SealSourceCouple)]
impl SealSourceCoupleWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(seqner: SeqnerWrapper, saider: SaiderWrapper) -> SealSourceCoupleWrapper {
        SealSourceCoupleWrapper(SealSourceCouple::new((*seqner).clone(), (*saider).clone()))
    }

    pub fn seqner(&self) -> SeqnerWrapper {
        SeqnerWrapper::wrap(&self.0.seqner)
    }
    pub fn saider(&self) -> SaiderWrapper {
        SaiderWrapper::wrap(&self.0.saider)
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

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<SealSourceCouple> {
        from_jsval::<SealSourceCoupleWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for SealSourceCoupleWrapper {
    type Target = SealSourceCouple;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
