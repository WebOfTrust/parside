use parside::error::ParsideResult;

pub use parside::{
    CustomPayload,
    Message,
    MessageList,
};

/*
    MessageList
*/
pub struct MessageListFromStreamResult {
    pub rest: Vec<u8>,
    pub messages: MessageList
}

pub fn message_list_from_stream_bytes(bytes: &[u8]) -> ParsideResult<MessageListFromStreamResult> {
    let (rest, messages) = MessageList::from_stream_bytes(bytes)?;
    Ok(MessageListFromStreamResult {
        rest: rest.to_vec(),
        messages
    })
}
