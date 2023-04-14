use crate::error::JsResult;
use crate::error::*;
use crate::utils::from_jsval;
use cesride_wasm::Wrap;
use cesride_wasm::{PrefixerWrapper, SaiderWrapper, SeqnerWrapper, SigerWrapper};
use js_sys::Array;
use parside_core::message::{GroupItem, TransReceiptQuadruple, TransReceiptQuadruples};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = TransReceiptQuadruples)]
pub struct TransReceiptQuadruplesWrapper(pub(crate) TransReceiptQuadruples);

#[wasm_bindgen(js_class = TransReceiptQuadruples)]
impl TransReceiptQuadruplesWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<TransReceiptQuadruplesWrapper> {
        let values = value
            .iter()
            .map(TransReceiptQuadrupleWrapper::from_jsvalue)
            .collect::<Result<Vec<TransReceiptQuadruple>>>()?;
        Ok(TransReceiptQuadruplesWrapper(TransReceiptQuadruples::new(
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
            .map(|value| TransReceiptQuadrupleWrapper(value.clone()))
            .map(JsValue::from)
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<TransReceiptQuadrupleWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(TransReceiptQuadrupleWrapper(value.clone())),
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

#[wasm_bindgen(js_name = TransReceiptQuadruple)]
pub struct TransReceiptQuadrupleWrapper(pub(crate) TransReceiptQuadruple);

#[wasm_bindgen(js_class = TransReceiptQuadruple)]
impl TransReceiptQuadrupleWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(
        prefixer: PrefixerWrapper,
        seqner: SeqnerWrapper,
        saider: SaiderWrapper,
        siger: SigerWrapper,
    ) -> TransReceiptQuadrupleWrapper {
        TransReceiptQuadrupleWrapper(TransReceiptQuadruple::new(
            (*prefixer).clone(),
            (*seqner).clone(),
            (*saider).clone(),
            (*siger).clone(),
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

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<TransReceiptQuadruple> {
        from_jsval::<TransReceiptQuadrupleWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for TransReceiptQuadrupleWrapper {
    type Target = TransReceiptQuadruple;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
