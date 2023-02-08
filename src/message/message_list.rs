use nom::multi::many0;

use crate::error::ParsideResult;
use crate::{nomify, Message};

pub struct MessageList {
    pub messages: Vec<Message>,
}

impl MessageList {
    pub fn from_stream_bytes<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], MessageList)> {
        let (rest, messages) = many0(
            nomify!(Message::from_stream_bytes)
        )(bytes)?;
        return Ok((rest, MessageList { messages }));
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_message_list() {
        let stream = br#"{"v":"KERI10JSON00012b_","t":"icp"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG"#;
        let (rest, message_list) = MessageList::from_stream_bytes(stream).unwrap();
        assert_eq!(2, message_list.messages.len());
        assert!(rest.is_empty());
        assert!(matches!(message_list.messages[0], Message::Custom { .. }));
        assert!(matches!(message_list.messages[1], Message::Group { .. }));

        let stream = br#"{"v":"KERI10JSON00012b_","t":"icp"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG{"v":"KERI10JSON00012b_","t":"icp"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG"#;
        let (rest, message_list) = MessageList::from_stream_bytes(stream).unwrap();
        assert!(rest.is_empty());
        assert_eq!(4, message_list.messages.len());
        assert!(matches!(message_list.messages[0], Message::Custom { .. }));
        assert!(matches!(message_list.messages[1], Message::Group { .. }));
        assert!(matches!(message_list.messages[2], Message::Custom { .. }));
        assert!(matches!(message_list.messages[3], Message::Group { .. }));
    }
}
