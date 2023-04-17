# parside

[![parside](https://github.com/WebOfTrust/parside/actions/workflows/test.yml/badge.svg)](https://github.com/WebOfTrust/parside/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/WebOfTrust/parside/branch/main/graph/badge.svg?token=L8K7H1XXQS)](https://codecov.io/gh/WebOfTrust/parside)

Parser library for Composable Event Streaming Representation (CESR).

This library is **currently under construction**.

## Example

Parse stream containing multiple messages:

```
use cesride::{CesrGroup, Message, MessageList};

let stream = br#"{"v":"12b_","t":"icp"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG{"v":"81b_","t":"tcp"}-AABAABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0Orest"#;
let (rest, message_list) = MessageList::from_stream_bytes(stream)?;
println!("Rest bytes: {:?}", rest);

for message in message_list.messages {
    match message {
        Message::Custom { value } => {
            println!("Handle custom payload {:?}", value)
        },
        Message::Group { value } => {
            match value {
                CesrGroup::ControllerIdxSigsVariant { value } => println!("Handle ControllerIdxSigsVariant {:?}", value),
                CesrGroup::WitnessIdxSigsVariant { value } => println!("Handle WitnessIdxSigsVariant {:?}", value),
                CesrGroup::NonTransReceiptCouplesVariant { value } => println!("Handle NonTransReceiptCouplesVariant {:?}", value),
                CesrGroup::TransReceiptQuadruplesVariant { value } => println!("Handle TransReceiptQuadruplesVariant {:?}", value),
                CesrGroup::TransIdxSigGroupsVariant { value } => println!("Handle TransIdxSigGroupsVariant {:?}", value),
                CesrGroup::TransLastIdxSigGroupsVariant { value } => println!("Handle TransLastIdxSigGroupsVariant {:?}", value),
                CesrGroup::FirstSeenReplayCouplesVariant { value } => println!("Handle FirstSeenReplayCouplesVariant {:?}", value),
                CesrGroup::SealSourceCouplesVariant { value } => println!("Handle SealSourceCouplesVariant {:?}", value),
                CesrGroup::AttachedMaterialQuadletsVariant { value } => println!("Handle AttachedMaterialQuadletsVariant {:?}", value),
                CesrGroup::SadPathSigGroupVariant { value } => println!("Handle SadPathSigGroupVariant {:?}", value),
                CesrGroup::SadPathSigVariant { value } => println!("Handle SadPathSigVariant {:?}", value),
                CesrGroup::PathedMaterialQuadletsVariant { value } => println!("Handle PathedMaterialQuadletsVariant {:?}", value),
            }
        }
    }
}
```

Parse single message from stream:
```
use cesride::{CesrGroup, Message, MessageList};

let stream = br#"{"v":"12b_","t":"icp"}-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG{"v":"81b_","t":"tcp"}-AABAABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0Orest"#;
let (rest, message) = Message::from_stream_bytes(stream).unwrap();
println!("Rest bytes: {:?}", rest);

match message {
    Message::Custom { value } => {
        println!("Handle custom payload {:?}", value)
    },
    Message::Group { value } => {
        match value {
            CesrGroup::ControllerIdxSigsVariant { value } => println!("Handle ControllerIdxSigsVariant {:?}", value),
            _ => println!("Handle Group"),
        }
    }
}
```

## Community

Parside work currently resides alongside the [cesride](https://github.com/WebOfTrust/cesride) work.

## Bi-weekly Meeting
- [Zoom Link](https://us06web.zoom.us/j/88102305873?pwd=Wm01TEJKUWc0aE51a0QzZ2hNbTV2Zz09)
- [HackMD Link](https://hackmd.io/UQaEI0w8Thy_xRF7oYX03Q?view) Bi-Weekly Meeting Agenda and Minutes
- Slack https://join.slack.com/t/keriworld/shared_invite/zt-14326yxue-p7P~GEmAZ65luGSZvbgFAQ
    - `#cesr` channel.

# Important Reference Material
- WebOfTrust/[ietf-cesr](https://github.com/WebOfTrust/ietf-cesr) repository - IETF draft specification for CESR
- Design Assumptions, Use Cases, and ToDo list - [HackMD link](https://hackmd.io/W2Z39cuSSTmD2TovVLvAPg?view)
- Introductory articles:
    - [#1 CESR Proof Signatures](https://medium.com/happy-blockchains/cesr-proof-signatures-are-the-segwit-of-authentic-data-in-keri-e891c83e070a)
    - [#2 CESR Overview](https://medium.com/happy-blockchains/cesr-one-of-sam-smiths-inventions-is-as-controversial-as-genius-d757f36b88f8)