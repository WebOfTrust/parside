use crate::error::*;
use parside_core::CesrGroup;
use wasm_bindgen::prelude::wasm_bindgen;

// pub mod attached_material_quadlets;
pub mod controller_idx_sigs;

#[wasm_bindgen(js_name = CesrGroup)]
#[derive(Debug, Clone)]
pub struct CesrGroupWrapper(pub(crate) CesrGroup);

#[wasm_bindgen(getter_with_clone)]
pub struct ParserRet {
    pub rest: Vec<u8>,
    pub group: CesrGroupWrapper,
}

#[wasm_bindgen(js_class = CesrGroup)]
impl CesrGroupWrapper {
    pub fn from_stream_bytes(bytes: &[u8]) -> Result<ParserRet> {
        let ret = CesrGroup::from_stream_bytes(bytes).as_js()?;
        Ok(ParserRet { rest: ret.0.to_vec(), group: CesrGroupWrapper(ret.1) })
    }
}
