use crate::error::JsResult;
use crate::error::*;
use crate::utils::from_jsval;
use cesride_wasm::Wrap;
use cesride_wasm::{DaterWrapper, SeqnerWrapper};
use js_sys::Array;
use parside_core::message::{FirstSeenReplayCouple, FirstSeenReplayCouples, GroupItem};
use parside_core::Group;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = FirstSeenReplayCouples)]
pub struct FirstSeenReplayCouplesWrapper(pub(crate) FirstSeenReplayCouples);

#[wasm_bindgen(js_class = FirstSeenReplayCouples)]
impl FirstSeenReplayCouplesWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &Array) -> Result<FirstSeenReplayCouplesWrapper> {
        let values = value
            .iter()
            .map(FirstSeenReplayCoupleWrapper::from_jsvalue)
            .collect::<Result<Vec<FirstSeenReplayCouple>>>()?;
        Ok(FirstSeenReplayCouplesWrapper(FirstSeenReplayCouples::new(
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
            .map(|value| FirstSeenReplayCoupleWrapper(value.clone()))
            .map(JsValue::from)
            .collect()
    }

    pub fn value(&self, index: usize) -> Option<FirstSeenReplayCoupleWrapper> {
        match self.0.value.get(index) {
            Some(value) => Some(FirstSeenReplayCoupleWrapper(value.clone())),
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

#[wasm_bindgen(js_name = FirstSeenReplayCouple)]
pub struct FirstSeenReplayCoupleWrapper(pub(crate) FirstSeenReplayCouple);

#[wasm_bindgen(js_class = FirstSeenReplayCouple)]
impl FirstSeenReplayCoupleWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(firner: SeqnerWrapper, dater: DaterWrapper) -> FirstSeenReplayCoupleWrapper {
        FirstSeenReplayCoupleWrapper(FirstSeenReplayCouple::new(
            (*firner).clone(),
            (*dater).clone(),
        ))
    }

    pub fn firner(&self) -> SeqnerWrapper {
        SeqnerWrapper::wrap(&self.0.firner)
    }

    pub fn dater(&self) -> DaterWrapper {
        DaterWrapper::wrap(&self.0.dater)
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

    pub(crate) fn from_jsvalue(value: JsValue) -> Result<FirstSeenReplayCouple> {
        from_jsval::<FirstSeenReplayCoupleWrapper>(value).map(|item| (*item).clone())
    }
}

impl Deref for FirstSeenReplayCoupleWrapper {
    type Target = FirstSeenReplayCouple;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
