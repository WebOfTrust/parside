use crate::error::*;
use crate::message::custom_payload::CustomPayloadWrapper;
use crate::message::groups::CesrGroupWrapper;
use parside_core::Message as ParsideMessage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = MessageParsingResult)]
pub struct MessageParsingResult {
    pub(crate) rest: Vec<u8>,
    pub(crate) message: MessageWrapper,
}

#[wasm_bindgen(js_name = Message)]
#[derive(Clone)]
pub struct MessageWrapper {
    pub(crate) group: Option<CesrGroupWrapper>,
    pub(crate) payload: Option<CustomPayloadWrapper>,
}

#[wasm_bindgen(js_class = Message)]
impl MessageWrapper {
    pub fn from_stream_bytes(value: Vec<u8>) -> Result<MessageParsingResult> {
        let (rest, parsed_message) = ParsideMessage::from_stream_bytes(&value)
            .as_js()
            .map_err(JsValue::from)?;
        let parsed_result = MessageParsingResult {
            rest: rest.to_vec(),
            message: MessageWrapper::from(parsed_message),
        };
        Ok(parsed_result)
    }

    #[wasm_bindgen(getter)]
    pub fn group(&self) -> Option<CesrGroupWrapper> {
        self.group.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn payload(&self) -> Option<CustomPayloadWrapper> {
        self.payload.clone()
    }
}

#[wasm_bindgen(js_class = MessageParsingResult)]
impl MessageParsingResult {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> MessageWrapper {
        self.message.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn rest(&self) -> Vec<u8> {
        self.rest.clone()
    }
}

impl From<ParsideMessage> for MessageWrapper {
    fn from(message: ParsideMessage) -> Self {
        MessageWrapper {
            group: message
                .cesr_group()
                .ok()
                .cloned()
                .map(|group| CesrGroupWrapper(group)),
            payload: message
                .payload()
                .ok()
                .cloned()
                .map(|group| CustomPayloadWrapper(group)),
        }
    }
}
