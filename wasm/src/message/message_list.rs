use crate::error::*;
use crate::message::message::MessageWrapper;
use js_sys::Array;
use parside_core::MessageList as ParsideMessageList;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = MessageListParsingResult)]
pub struct MessageListParsingResult {
    pub(crate) rest: Vec<u8>,
    pub(crate) messages: Vec<MessageWrapper>,
}

#[wasm_bindgen(js_name = MessageList)]
pub struct MessageListWrapper(pub(crate) ParsideMessageList);

#[wasm_bindgen(js_class = MessageList)]
impl MessageListWrapper {
    pub fn from_stream_bytes(value: Vec<u8>) -> Result<MessageListParsingResult> {
        let (rest, parsed_message) = ParsideMessageList::from_stream_bytes(&value)
            .as_js()
            .map_err(JsValue::from)?;
        let parsed_result = MessageListParsingResult {
            rest: rest.to_vec(),
            messages: parsed_message
                .messages
                .into_iter()
                .map(MessageWrapper::from)
                .collect(),
        };
        Ok(parsed_result)
    }
}

#[wasm_bindgen(js_class = MessageListParsingResult)]
impl MessageListParsingResult {
    pub fn size(&self) -> usize {
        self.messages.len()
    }

    #[wasm_bindgen(getter)]
    pub fn messages(&self) -> Array {
        self.messages
            .iter()
            .map(|message| JsValue::from(message.clone()))
            .collect()
    }

    pub fn message(&self, index: usize) -> Option<MessageWrapper> {
        match self.messages.get(index) {
            Some(value) => Some(value.clone()),
            _ => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn rest(&self) -> Vec<u8> {
        self.rest.clone()
    }
}
