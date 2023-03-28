use crate::error::*;
use crate::{error::JsResult, message::cold_code::ColdCode};
use cesride::Counter;
use cesride_wasm::SigerWrapper;
use cesride_wasm::Wrap;
use parside_core::message::{ControllerIdxSig, ControllerIdxSigs, GroupItem};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = ControllerIdxSigs)]
pub struct ControllerIdxSigsWrapper(pub(crate) ControllerIdxSigs);

#[wasm_bindgen(js_class = ControllerIdxSigs)]
impl ControllerIdxSigsWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &ControllerIdxSigWrapper) -> ControllerIdxSigsWrapper {
        todo!()
        // let vec = value.map(|x| x.0);
        // ControllerIdxSigsWrapper(ControllerIdxSigs::new(value))
    }

    pub fn value(&self) -> ControllerIdxSigWrapper {
        todo!()
        // let v = self.0.value();
        // &v
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

    pub fn value(&self) -> SigerWrapper {
        SigerWrapper.wrap(self.0.siger)
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
