use crate::error::*;

use js_sys::Reflect;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::JsValue;

pub fn from_jsval<T: FromWasmAbi<Abi = u32>>(js: JsValue) -> Result<T> {
    let ptr = Reflect::get(&js, &JsValue::from_str("ptr"))?;
    let ptr_u32: u32 = ptr.as_f64().ok_or(JsValue::NULL)? as u32;
    let object = unsafe { T::from_abi(ptr_u32) };
    Ok(object)
}
