use js_sys::Error as JsError;
use parside_core::error::ParsideResult;
use wasm_bindgen::JsValue;

pub type Result<T> = core::result::Result<T, JsValue>;

// Allows conversion of ParsideError to javascript error
pub(crate) trait JsResult<T> {
    #[allow(clippy::wrong_self_convention)]
    fn as_js(self) -> core::result::Result<T, JsError>;
}

impl<T> JsResult<T> for ParsideResult<T> {
    fn as_js(self) -> core::result::Result<T, JsError> {
        self.map_err(|e| JsError::new(&e.to_string()))
    }
}
