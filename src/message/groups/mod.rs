pub mod attached_material_quadlets;
pub mod controller_idx_sigs;
pub mod first_seen_replay_couples;
pub mod group;
pub mod non_trans_receipt_couples;
pub mod pathed_material_quadlets;
pub mod sad_path_sig;
pub mod sad_path_sig_group;
pub mod seal_source_couples;
pub mod trans_idx_sig_groups;
pub mod trans_last_idx_sig_groups;
pub mod trans_receipt_quadruples;
pub mod witness_idx_sigs;

use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCode;
use crate::message::parsers::Parsers;

pub use self::attached_material_quadlets::AttachedMaterialQuadlets;
pub use self::controller_idx_sigs::{ControllerIdxSig, ControllerIdxSigs};
pub use self::first_seen_replay_couples::{FirstSeenReplayCouple, FirstSeenReplayCouples};
pub use self::group::{Group, GroupItem};
pub use self::non_trans_receipt_couples::{NonTransReceiptCouple, NonTransReceiptCouples};
pub use self::pathed_material_quadlets::{PathedMaterialQuadlet, PathedMaterialQuadlets};
pub use self::sad_path_sig::{SadPathSig, SadPathSigs};
pub use self::sad_path_sig_group::{SadPathSigGroup, SadPathSigGroups};
pub use self::seal_source_couples::{SealSourceCouple, SealSourceCouples};
pub use self::trans_idx_sig_groups::{TransIdxSigGroup, TransIdxSigGroups};
pub use self::trans_last_idx_sig_groups::{TransLastIdxSigGroup, TransLastIdxSigGroups};
pub use self::trans_receipt_quadruples::{TransReceiptQuadruple, TransReceiptQuadruples};
pub use self::witness_idx_sigs::{WitnessIdxSig, WitnessIdxSigs};

#[derive(Debug, Clone)]
pub enum CesrGroup {
    ControllerIdxSigsVariant { value: ControllerIdxSigs },
    WitnessIdxSigsVariant { value: WitnessIdxSigs },
    NonTransReceiptCouplesVariant { value: NonTransReceiptCouples },
    TransReceiptQuadruplesVariant { value: TransReceiptQuadruples },
    TransIdxSigGroupsVariant { value: TransIdxSigGroups },
    TransLastIdxSigGroupsVariant { value: TransLastIdxSigGroups },
    FirstSeenReplayCouplesVariant { value: FirstSeenReplayCouples },
    SealSourceCouplesVariant { value: SealSourceCouples },
    AttachedMaterialQuadletsVariant { value: AttachedMaterialQuadlets },
    SadPathSigGroupVariant { value: SadPathSigGroups },
    SadPathSigVariant { value: SadPathSigs },
    PathedMaterialQuadletsVariant { value: PathedMaterialQuadlets },
}

impl CesrGroup {
    pub fn from_stream_bytes(bytes: &[u8]) -> ParsideResult<(&[u8], CesrGroup)> {
        if bytes.is_empty() {
            return Err(ParsideError::EmptyBytesStream);
        }

        let cold_code = ColdCode::try_from(bytes[0])?;
        let (rest, counter) = Parsers::counter_parser(&cold_code)?(bytes)?;
        let code = counter.code();

        match code.as_str() {
            AttachedMaterialQuadlets::CODE => {
                let (rest, group) =
                    AttachedMaterialQuadlets::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::AttachedMaterialQuadletsVariant { value: group }))
            }
            ControllerIdxSigs::CODE => {
                let (rest, group) =
                    ControllerIdxSigs::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::ControllerIdxSigsVariant { value: group }))
            }
            WitnessIdxSigs::CODE => {
                let (rest, group) = WitnessIdxSigs::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::WitnessIdxSigsVariant { value: group }))
            }
            NonTransReceiptCouples::CODE => {
                let (rest, group) =
                    NonTransReceiptCouples::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::NonTransReceiptCouplesVariant { value: group }))
            }
            TransReceiptQuadruples::CODE => {
                let (rest, group) =
                    TransReceiptQuadruples::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::TransReceiptQuadruplesVariant { value: group }))
            }
            TransIdxSigGroups::CODE => {
                let (rest, group) =
                    TransIdxSigGroups::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::TransIdxSigGroupsVariant { value: group }))
            }
            TransLastIdxSigGroups::CODE => {
                let (rest, group) =
                    TransLastIdxSigGroups::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::TransLastIdxSigGroupsVariant { value: group }))
            }
            FirstSeenReplayCouples::CODE => {
                let (rest, group) =
                    FirstSeenReplayCouples::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::FirstSeenReplayCouplesVariant { value: group }))
            }
            SealSourceCouples::CODE => {
                let (rest, group) =
                    SealSourceCouples::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::SealSourceCouplesVariant { value: group }))
            }
            SadPathSigGroups::CODE => {
                let (rest, group) =
                    SadPathSigGroups::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::SadPathSigGroupVariant { value: group }))
            }
            SadPathSigs::CODE => {
                let (rest, group) = SadPathSigs::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::SadPathSigVariant { value: group }))
            }
            PathedMaterialQuadlets::CODE => {
                let (rest, group) =
                    PathedMaterialQuadlets::from_stream_bytes(rest, &counter, &cold_code)?;
                Ok((rest, CesrGroup::PathedMaterialQuadletsVariant { value: group }))
            }
            _ => Err(ParsideError::Unexpected(format!(
                "Unexpected counter code {:?}",
                counter.code()
            ))),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use cesride::matter::Codex as MatterCodex;
    use cesride::{Indexer, Matter};

    #[test]
    pub fn test_parse_trans_idx_sig_groups() {
        let stream = br#"-FABEFhg5my9DuMU6gw1CVk6QgkmZKBttWSXDzVzWVmxh0_K0AAAAAAAAAAAAAAAAAAAAAAAEFhg5my9DuMU6gw1CVk6QgkmZKBttWSXDzVzWVmxh0_K-AABAADghKct9eYTuSgSd5wdPSYG06tGX7ZRp_BDnrgbSxJpsJtrA-fP7Pa1W602gHeMrO6HZsD1z3tWV5jGlApFmVIB"#;

        let (_rest, group) = CesrGroup::from_stream_bytes(stream).unwrap();

        match group {
            CesrGroup::TransIdxSigGroupsVariant { value: group } => {
                assert_eq!(1, group.value.len());
                assert_eq!(MatterCodex::Blake3_256.to_string(), group.value[0].prefixer.code());
                assert_eq!(MatterCodex::Salt_128.to_string(), group.value[0].seqner.code());
                assert_eq!(MatterCodex::Blake3_256.to_string(), group.value[0].saider.code());
            }
            _ => panic!("Unexpected case"),
        }
    }

    #[test]
    pub fn test_parse_controller_idx_sigs() {
        let stream = br#"-AABAABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O"#;
        let (rest, group) = CesrGroup::from_stream_bytes(stream).unwrap();

        assert!(rest.is_empty());
        match group {
            CesrGroup::ControllerIdxSigsVariant { value: group } => {
                assert_eq!(1, group.value.len());
                assert_eq!(MatterCodex::Ed25519_Seed.to_string(), group.value[0].siger.code());
            }
            _ => panic!("Unexpected case"),
        }
    }

    #[test]
    pub fn test_parse_non_trans_receipt_couples() {
        let stream = br#"-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG"#;

        let (_rest, group) = CesrGroup::from_stream_bytes(stream).unwrap();

        match group {
            CesrGroup::NonTransReceiptCouplesVariant { value: group } => {
                assert_eq!(1, group.value.len());
                assert_eq!(MatterCodex::Ed25519_Sig.to_string(), group.value[0].cigar.code());
            }
            _ => panic!("Unexpected case"),
        }
    }

    #[test]
    pub fn test_parse_attached_material_quadlets() {
        let stream = br#"-VA--AABAAAEmCc25ETG2m1Ya-tPGuEqsPywOtusQwXKy076ve56IHXzX2bs0xsdQ4dk0XsanstpThg71ynIy-yUDSue6jMD-BABAABfvC7zCIVOVMol9C4AlSALS9JhL8PCdfgRnJgkXG4U11gFyZbsI_J828POrtwtoOmFhs20hoH1pYw4NZr2cdwN-EAB0AAAAAAAAAAAAAAAAAAAAAAE1AAG2023-02-07T15c00c00d025640p00c00"#;

        let (_rest, _message) = CesrGroup::from_stream_bytes(stream).unwrap();
    }

    #[test]
    pub fn test_parse_trans_last_idx_sig_groups() {
        let stream = br#"-HABEB1f36VmoizOIpBIBv3X4ZiWJQWjtKJ7TMmsZltT0B32-AABAAAKB9u6wyLS9kl_iGVGCqrs-3XqFbyGeOKuiOEA9JZpxI9GMv0GJv2wbY1-sOD_HOJcvXO7LSO8g8MSeRXjtL4I"#;

        let (_rest, group) = CesrGroup::from_stream_bytes(stream).unwrap();

        match group {
            CesrGroup::TransLastIdxSigGroupsVariant { value: group } => {
                assert_eq!(1, group.value.len());
                assert_eq!(MatterCodex::Blake3_256.to_string(), group.value[0].prefixer.code());
            }
            _ => panic!("Unexpected case"),
        }
    }
}
