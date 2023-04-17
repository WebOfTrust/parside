use nom::multi::many0;

use crate::error::{ParsideError, ParsideResult};
use crate::{nomify, Message};

/// Datastructures representing list of parsed messages
#[derive(Debug)]
pub struct MessageList {
    pub messages: Vec<Message>,
}

impl MessageList {
    /// Parse multiple messages from provided bytes
    pub fn from_stream_bytes<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], MessageList)> {
        if bytes.is_empty() {
            return Err(ParsideError::EmptyBytesStream);
        }
        let (rest, messages) = many0(nomify!(Message::from_stream_bytes))(bytes)?;
        Ok((rest, MessageList { messages }))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::CesrGroup;
    use crate::error::ParsideError;

    const PAYLOAD_1: &'static str = r#"{"v":"1","t":"foo"}"#;
    const PAYLOAD_2:&'static str = r#"{"v":"2","t":"bla"}"#;
    const NON_TRANS_RECEIPT_COUPLES: &'static str = r#"-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG"#;
    const CONTROLLER_IDX_SIGS: &'static str = r#"-AABAABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O"#;
    const REST: &'static str = "rest";

    #[test]
    pub fn test_parse_message_list_with_empty_bytes() {
        let err = MessageList::from_stream_bytes(&[]).unwrap_err();
        assert_eq!(err, ParsideError::EmptyBytesStream);
    }

    #[test]
    pub fn test_parse_message_list_does_not_containing_messages() {
        let stream = format!("{}", REST);
        let (rest, message_list) = MessageList::from_stream_bytes(stream.as_bytes()).unwrap();
        assert!(!rest.is_empty());
        assert_eq!(REST.as_bytes(), rest);
        assert_eq!(0, message_list.messages.len());
    }

    #[test]
    pub fn test_parse_message_list_containing_single_generic_payload() {
        let stream = format!("{}", PAYLOAD_1);
        let (rest, message_list) = MessageList::from_stream_bytes(stream.as_bytes()).unwrap();
        assert!(rest.is_empty());
        assert_eq!(1, message_list.messages.len());
        assert!(matches!(message_list.messages[0], Message::Custom { .. }));
    }

    #[test]
    pub fn test_parse_message_list_containing_single_cesr_group() {
        let stream = format!("{}", NON_TRANS_RECEIPT_COUPLES);
        let (rest, message_list) = MessageList::from_stream_bytes(stream.as_bytes()).unwrap();
        assert!(rest.is_empty());
        assert_eq!(1, message_list.messages.len());
        assert!(matches!(message_list.messages[0], Message::Group { value: CesrGroup::NonTransReceiptCouplesVariant { .. } }));
    }

    #[test]
    pub fn test_parse_message_list_containing_multiple_message() {
        let stream = format!("{}{}{}{}", PAYLOAD_1, NON_TRANS_RECEIPT_COUPLES, PAYLOAD_2, CONTROLLER_IDX_SIGS);
        let (rest, message_list) = MessageList::from_stream_bytes(stream.as_bytes()).unwrap();
        assert!(rest.is_empty());
        assert_eq!(4, message_list.messages.len());
        assert!(matches!(message_list.messages[0], Message::Custom { .. }));
        assert!(matches!(message_list.messages[1], Message::Group { value: CesrGroup::NonTransReceiptCouplesVariant { .. } }));
        assert!(matches!(message_list.messages[2], Message::Custom { .. }));
        assert!(matches!(message_list.messages[3], Message::Group { value: CesrGroup::ControllerIdxSigsVariant { .. } }));
    }

    #[test]
    pub fn test_parse_message_list_containing_multiple_message_with_rest_bytes() {
        let stream = format!("{}{}{}", PAYLOAD_1, NON_TRANS_RECEIPT_COUPLES, REST);
        let (rest, message_list) = MessageList::from_stream_bytes(stream.as_bytes()).unwrap();
        assert!(!rest.is_empty());
        assert_eq!(2, message_list.messages.len());
        assert!(matches!(message_list.messages[0], Message::Custom { .. }));
        assert!(matches!(message_list.messages[1], Message::Group { value: CesrGroup::NonTransReceiptCouplesVariant { .. } }));
    }
}
