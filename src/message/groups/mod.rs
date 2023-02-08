pub mod attached_material_quadlets;
pub mod controller_idx_sigs;
pub mod first_seen_replay_couples;
pub mod non_trans_receipt_couples;
pub mod seql_source_couples;
pub mod trans_idx_sig_groups;
pub mod trans_last_idx_sig_groups;
pub mod trans_receipt_quadruples;
pub mod witness_idx_sigs;

use crate::error::ParsideResult;
use crate::message::cold_code::ColdCodes;
use crate::utils::parsers::Parsers;
use cesride::Counter;

use self::attached_material_quadlets::AttachedMaterialQuadlets;
use self::controller_idx_sigs::ControllerIdxSigs;
use self::first_seen_replay_couples::FirstSeenReplayCouples;
use self::non_trans_receipt_couples::NonTransReceiptCouples;
use self::seql_source_couples::SealSourceCouples;
use self::trans_idx_sig_groups::TransIdxSigGroups;
use self::trans_last_idx_sig_groups::TransLastIdxSigGroups;
use self::trans_receipt_quadruples::TransReceiptQuadruples;
use self::witness_idx_sigs::WitnessIdxSigs;

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
}

impl CesrGroup {
    pub fn from_stream_bytes<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], CesrGroup)> {
        let cold_code = ColdCodes::try_from(bytes[0])?;
        let (rest, counter) = Parsers::counter_parser(&cold_code)?(bytes)?;

        if counter.code() == AttachedMaterialQuadlets::code() {
            let (rest, group) =
                AttachedMaterialQuadlets::from_stream_bytes(rest, &counter, &cold_code)?;
            return Ok((
                rest,
                CesrGroup {
                    counter,
                    group: CesrGroupVariants::AttachedMaterialQuadletsVariant { value: group },
                },
            ));
        }

        if counter.code() == ControllerIdxSigs::code() {
            let (rest, group) = ControllerIdxSigs::from_stream_bytes(rest, &counter, &cold_code)?;
            return Ok((
                rest,
                CesrGroup {
                    counter,
                    group: CesrGroupVariants::ControllerIdxSigsVariant { value: group },
                },
            ));
        }

        if counter.code() == WitnessIdxSigs::code() {
            let (rest, group) = WitnessIdxSigs::from_stream_bytes(rest, &counter, &cold_code)?;
            return Ok((
                rest,
                CesrGroup {
                    counter,
                    group: CesrGroupVariants::WitnessIdxSigsVariant { value: group },
                },
            ));
        }

        if counter.code() == NonTransReceiptCouples::code() {
            let (rest, group) =
                NonTransReceiptCouples::from_stream_bytes(rest, &counter, &cold_code)?;
            return Ok((
                rest,
                CesrGroup {
                    counter,
                    group: CesrGroupVariants::NonTransReceiptCouplesVariant { value: group },
                },
            ));
        }

        if counter.code() == TransReceiptQuadruples::code() {
            let (rest, group) =
                TransReceiptQuadruples::from_stream_bytes(rest, &counter, &cold_code)?;
            return Ok((
                rest,
                CesrGroup {
                    counter,
                    group: CesrGroupVariants::TransReceiptQuadruplesVariant { value: group },
                },
            ));
        }

        if counter.code() == TransIdxSigGroups::code() {
            let (rest, group) = TransIdxSigGroups::from_stream_bytes(rest, &counter, &cold_code)?;
            return Ok((
                rest,
                CesrGroup {
                    counter,
                    group: CesrGroupVariants::TransIdxSigGroupsVariant { value: group },
                },
            ));
        }

        if counter.code() == TransLastIdxSigGroups::code() {
            let (rest, group) =
                TransLastIdxSigGroups::from_stream_bytes(rest, &counter, &cold_code)?;
            return Ok((
                rest,
                CesrGroup {
                    counter,
                    group: CesrGroupVariants::TransLastIdxSigGroupsVariant { value: group },
                },
            ));
        }

        if counter.code() == FirstSeenReplayCouples::code() {
            let (rest, group) =
                FirstSeenReplayCouples::from_stream_bytes(rest, &counter, &cold_code)?;
            return Ok((
                rest,
                CesrGroup {
                    counter,
                    group: CesrGroupVariants::FirstSeenReplayCouplesVariant { value: group },
                },
            ));
        }

        if counter.code() == SealSourceCouples::code() {
            let (rest, group) = SealSourceCouples::from_stream_bytes(rest, &counter, &cold_code)?;
            return Ok((
                rest,
                CesrGroup {
                    counter,
                    group: CesrGroupVariants::SealSourceCouplesVariant { value: group },
                },
            ));
        }

        unimplemented!();
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

        let expected_counter = Counter::new(TransIdxSigGroups::code(), 1);
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

        let expected_counter = Counter::new(ControllerIdxSigs::code(), 1);
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

        let expected_counter = Counter::new(NonTransReceiptCouples::code(), 1);
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

        let expected_counter = Counter::new(AttachedMaterialQuadlets::code(), 62);
        assert_eq!(expected_counter, message.counter);
    }

    #[test]
    pub fn test_parse_trans_last_idx_sig_groups() {
        let stream = br#"-HABEB1f36VmoizOIpBIBv3X4ZiWJQWjtKJ7TMmsZltT0B32-AABAAAKB9u6wyLS9kl_iGVGCqrs-3XqFbyGeOKuiOEA9JZpxI9GMv0GJv2wbY1-sOD_HOJcvXO7LSO8g8MSeRXjtL4I"#;

        let (_rest, message) = CesrGroup::from_stream_bytes(stream).unwrap();

        let expected_counter = Counter::new(TransLastIdxSigGroups::code(), 1);
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
