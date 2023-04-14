use crate::error::JsResult;
use crate::error::*;
use crate::utils::from_jsval;
use cesride_wasm::CigarWrapper;
use cesride_wasm::Wrap;
use js_sys::Array;
use parside_core::message::{GroupItem, NonTransReceiptCouple, NonTransReceiptCouples};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = NonTransReceiptCouples)]
pub struct NonTransReceiptCouplesWrapper(pub(crate) NonTransReceiptCouples);

#[wasm_bindgen(js_class = NonTransReceiptCouples)]
impl NonTransReceiptCouplesWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<NonTransReceiptCouplesWrapper> {
        let values = value
            .iter()
            .map(NonTransReceiptCoupleWrapper::from_jsvalue)
            .collect::<Result<Vec<NonTransReceiptCouple>>>()?;
        Ok(NonTransReceiptCouplesWrapper(NonTransReceiptCouples::new(
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
            .map(|value| JsValue::from(NonTransReceiptCoupleWrapper(value.clone())))
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<NonTransReceiptCoupleWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(NonTransReceiptCoupleWrapper(value.clone())),
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

#[wasm_bindgen(js_name = NonTransReceiptCouple)]
pub struct NonTransReceiptCoupleWrapper(pub(crate) NonTransReceiptCouple);

#[wasm_bindgen(js_class = NonTransReceiptCouple)]
impl NonTransReceiptCoupleWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(cigar: CigarWrapper) -> NonTransReceiptCoupleWrapper {
        NonTransReceiptCoupleWrapper(NonTransReceiptCouple::new((*cigar).clone()))
    }

    pub fn cigar(&self) -> CigarWrapper {
        CigarWrapper::wrap(&self.0.cigar)
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

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<NonTransReceiptCouple> {
        from_jsval::<NonTransReceiptCoupleWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for NonTransReceiptCoupleWrapper {
    type Target = NonTransReceiptCouple;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
