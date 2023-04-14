use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub enum ColdCode {
    // not taken
    Free = 0b000,
    // CountCode Base64
    CtB64 = 0b001,
    // OpCode Base64
    OpB64 = 0b010,
    // JSON Map Event Start
    Json = 0b011,
    // MGPK Fixed Map Event Start
    MGPK1 = 0b100,
    // CBOR Map Event Start
    Cbor = 0b101,
    // MGPK Big 16 or 32 Map Event Start
    MGPK2 = 0b110,
    // CountCode or OpCode Base2
    CtOpB2 = 0b111,
}

// impl From<parside_core::message::cold_code::ColdCode> for ColdCode {
//     fn from(value: parside_core::message::cold_code::ColdCode) -> Self {
//         todo!()
//     }
// }
