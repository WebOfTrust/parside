use crate::error::*;
use parside_core::CustomPayload;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = CustomPayload)]
#[derive(Clone)]
pub struct CustomPayloadWrapper(pub(crate) CustomPayload);

#[wasm_bindgen(js_class = CustomPayload)]
impl CustomPayloadWrapper {
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> Result<JsValue> {
        serde_wasm_bindgen::to_value(&self.0.value).map_err(|e| JsValue::from(&e.to_string()))
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.value.to_string().clone()
    }
}
