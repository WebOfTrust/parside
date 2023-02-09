pub mod attached_material_quadlets;
pub mod controller_idx_sigs;
pub mod first_seen_replay_couples;
pub mod non_trans_receipt_couples;
pub mod seal_source_couples;
pub mod trans_idx_sig_groups;
pub mod trans_last_idx_sig_groups;
pub mod trans_receipt_quadruples;
pub mod witness_idx_sigs;
pub mod sad_path_sig_group;
pub mod sad_path_sig;
pub mod pathed_material_quadlets;
pub mod parsers;

use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCodes;
use parsers::Parsers;
use cesride::Counter;
use cesride::counter::Codex;

use self::attached_material_quadlets::AttachedMaterialQuadlets;
use self::controller_idx_sigs::ControllerIdxSigs;
use self::first_seen_replay_couples::FirstSeenReplayCouples;
use self::non_trans_receipt_couples::NonTransReceiptCouples;
use self::seal_source_couples::SealSourceCouples;
use self::trans_idx_sig_groups::TransIdxSigGroups;
use self::trans_last_idx_sig_groups::TransLastIdxSigGroups;
use self::trans_receipt_quadruples::TransReceiptQuadruples;
use self::witness_idx_sigs::WitnessIdxSigs;
use self::sad_path_sig_group::SadPathSigGroup;
use self::sad_path_sig::SadPathSig;
use self::pathed_material_quadlets::PathedMaterialQuadlets;

#[derive(Debug)]
pub struct CesrGroup {
    pub counter: Counter,
    pub group: CesrGroupVariants,
}

#[derive(Debug)]
pub enum CesrGroupVariants {
    ControllerIdxSigsVariant { value: ControllerIdxSigs },
    WitnessIdxSigsVariant { value: WitnessIdxSigs },
    NonTransReceiptCouplesVariant { value: NonTransReceiptCouples },
    TransReceiptQuadruplesVariant { value: TransReceiptQuadruples },
    TransIdxSigGroupsVariant { value: TransIdxSigGroups },
    TransLastIdxSigGroupsVariant { value: TransLastIdxSigGroups },
    FirstSeenReplayCouplesVariant { value: FirstSeenReplayCouples },
    SealSourceCouplesVariant { value: SealSourceCouples },
    AttachedMaterialQuadletsVariant { value: AttachedMaterialQuadlets },
    SadPathSigGroupVariant { value: SadPathSigGroup },
    SadPathSigVariant { value: SadPathSig },
    PathedMaterialQuadletsVariant { value: PathedMaterialQuadlets },
}

impl CesrGroup {
    pub fn from_stream_bytes<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], CesrGroup)> {
        let cold_code = ColdCodes::try_from(bytes[0])?;
        let (rest, counter) = Parsers::counter_parser(&cold_code)?(bytes)?;
        let code = Codex::from_code(&counter.code())?;

        let (rest, group) =
            match code {
                AttachedMaterialQuadlets::CODE => {
                    let (rest, group) =
                        AttachedMaterialQuadlets::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::AttachedMaterialQuadletsVariant { value: group })
                }
                ControllerIdxSigs::CODE => {
                    let (rest, group) =
                        ControllerIdxSigs::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::ControllerIdxSigsVariant { value: group })
                }
                WitnessIdxSigs::CODE => {
                    let (rest, group) =
                        WitnessIdxSigs::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::WitnessIdxSigsVariant { value: group })
                }
                NonTransReceiptCouples::CODE => {
                    let (rest, group) =
                        NonTransReceiptCouples::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::NonTransReceiptCouplesVariant { value: group })
                }
                TransReceiptQuadruples::CODE => {
                    let (rest, group) =
                        TransReceiptQuadruples::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::TransReceiptQuadruplesVariant { value: group })
                }
                TransIdxSigGroups::CODE => {
                    let (rest, group) =
                        TransIdxSigGroups::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::TransIdxSigGroupsVariant { value: group })
                }
                TransLastIdxSigGroups::CODE => {
                    let (rest, group) =
                        TransLastIdxSigGroups::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::TransLastIdxSigGroupsVariant { value: group })
                }
                FirstSeenReplayCouples::CODE => {
                    let (rest, group) =
                        FirstSeenReplayCouples::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::FirstSeenReplayCouplesVariant { value: group })
                }
                SealSourceCouples::CODE => {
                    let (rest, group) =
                        SealSourceCouples::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::SealSourceCouplesVariant { value: group })
                }
                SadPathSigGroup::CODE => {
                    let (rest, group) =
                        SadPathSigGroup::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::SadPathSigGroupVariant { value: group })
                }
                SadPathSig::CODE => {
                    let (rest, group) =
                        SadPathSig::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::SadPathSigVariant { value: group })
                }
                PathedMaterialQuadlets::CODE => {
                    let (rest, group) =
                        PathedMaterialQuadlets::from_stream_bytes(rest, &counter, &cold_code)?;
                    (rest, CesrGroupVariants::PathedMaterialQuadletsVariant { value: group })
                }
                _ => {
                    return Err(ParsideError::Unexpected(format!("Unexpected counter code {:?}", counter.code())));
                }
            };

        return Ok((
            rest,
            CesrGroup { counter, group },
        ));
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    pub use cesride::matter::Codex as MatterCodex;

    #[test]
    pub fn test_parse_trans_idx_sig_groups() {
        let stream = br#"-FABEFhg5my9DuMU6gw1CVk6QgkmZKBttWSXDzVzWVmxh0_K0AAAAAAAAAAAAAAAAAAAAAAAEFhg5my9DuMU6gw1CVk6QgkmZKBttWSXDzVzWVmxh0_K-AABAADghKct9eYTuSgSd5wdPSYG06tGX7ZRp_BDnrgbSxJpsJtrA-fP7Pa1W602gHeMrO6HZsD1z3tWV5jGlApFmVIB"#;

        let (_rest, message) = CesrGroup::from_stream_bytes(stream).unwrap();

        let expected_counter = Counter::new(TransIdxSigGroups::CODE.code(), 1);
        assert_eq!(expected_counter, message.counter);

        match message.group {
            CesrGroupVariants::TransIdxSigGroupsVariant { value: group } => {
                assert_eq!(1, group.value.len());
                assert_eq!(
                    MatterCodex::Blake3_256.code().to_string(),
                    group.value[0].prefixer.code()
                );
                assert_eq!(
                    MatterCodex::Salt_128.code().to_string(),
                    group.value[0].seqner.code()
                );
                assert_eq!(
                    MatterCodex::Blake3_256.code().to_string(),
                    group.value[0].saider.code()
                );
            }
            _ => assert!(false, "Unexpected case"),
        }
    }

    #[test]
    pub fn test_parse_controller_idx_sigs() {
        let stream = br#"-AABAABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O"#;

        let (_rest, message) = CesrGroup::from_stream_bytes(stream).unwrap();

        let expected_counter = Counter::new(ControllerIdxSigs::CODE.code(), 1);
        assert_eq!(expected_counter, message.counter);

        match message.group {
            CesrGroupVariants::ControllerIdxSigsVariant { value: group } => {
                assert_eq!(1, group.value.len());
                assert_eq!(
                    MatterCodex::Ed25519_Seed.code().to_string(),
                    group.value[0].code()
                );
            }
            _ => assert!(false, "Unexpected case"),
        }
    }

    #[test]
    pub fn test_parse_non_trans_receipt_couples() {
        let stream = br#"-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG"#;

        let (_rest, message) = CesrGroup::from_stream_bytes(stream).unwrap();

        let expected_counter = Counter::new(NonTransReceiptCouples::CODE.code(), 1);
        assert_eq!(expected_counter, message.counter);

        match message.group {
            CesrGroupVariants::NonTransReceiptCouplesVariant { value: group } => {
                assert_eq!(1, group.value.len());
                assert_eq!(
                    MatterCodex::Ed25519N.code().to_string(),
                    group.value[0].verfer.code()
                );
                assert_eq!(
                    MatterCodex::Ed25519_Sig.code().to_string(),
                    group.value[0].cigar.code()
                );
            }
            _ => assert!(false, "Unexpected case"),
        }
    }

    #[test]
    pub fn test_parse_attached_material_quadlets() {
        let stream = br#"-VA--AABAAAEmCc25ETG2m1Ya-tPGuEqsPywOtusQwXKy076ve56IHXzX2bs0xsdQ4dk0XsanstpThg71ynIy-yUDSue6jMD-BABAABfvC7zCIVOVMol9C4AlSALS9JhL8PCdfgRnJgkXG4U11gFyZbsI_J828POrtwtoOmFhs20hoH1pYw4NZr2cdwN-EAB0AAAAAAAAAAAAAAAAAAAAAAE1AAG2023-02-07T15c00c00d025640p00c00"#;

        let (_rest, message) = CesrGroup::from_stream_bytes(stream).unwrap();

        let expected_counter = Counter::new(AttachedMaterialQuadlets::CODE.code(), 62);
        assert_eq!(expected_counter, message.counter);
    }

    #[test]
    pub fn test_parse_trans_last_idx_sig_groups() {
        let stream = br#"-HABEB1f36VmoizOIpBIBv3X4ZiWJQWjtKJ7TMmsZltT0B32-AABAAAKB9u6wyLS9kl_iGVGCqrs-3XqFbyGeOKuiOEA9JZpxI9GMv0GJv2wbY1-sOD_HOJcvXO7LSO8g8MSeRXjtL4I"#;

        let (_rest, message) = CesrGroup::from_stream_bytes(stream).unwrap();

        let expected_counter = Counter::new(TransLastIdxSigGroups::CODE.code(), 1);
        assert_eq!(expected_counter, message.counter);

        match message.group {
            CesrGroupVariants::TransLastIdxSigGroupsVariant { value: group } => {
                assert_eq!(1, group.value.len());
                assert_eq!(
                    MatterCodex::Blake3_256.code().to_string(),
                    group.value[0].prefixer.code()
                );
            }
            _ => assert!(false, "Unexpected case"),
        }
    }
}
