use crate::error::{ParsideError, ParsideResult};
use crate::message::cold_code::ColdCodes;
use crate::message::parsers::Parsers;
use cesride::{Counter, Matter};
use cesride::counter::Codex;
use nom::multi::count;
use nom::sequence::tuple;

#[derive(Debug)]
pub struct CesrGroup {
    pub counter: Counter,
    pub group: GroupData,
}

type Single = Vec<Matter>;
type Couple = Vec<(Matter, Matter)>;
type CoupleWithList = Vec<(Matter, Vec<Matter>)>;
type Quadruple = Vec<(Matter, Matter, Matter, Matter)>;
type QuadrupleWithList = Vec<(Matter, Matter, Matter, Vec<Matter>)>;

#[derive(Debug)]
pub enum GroupData {
    Single { value: Single },
    Couple { value: Couple },
    CoupleWithList { value: CoupleWithList },
    Quadruple { value: Quadruple },
    QuadrupleWithList { value: QuadrupleWithList },
}

impl GroupData {
    fn parse_single<'a>(bytes: &'a [u8], cold_code: &ColdCodes, amount: usize) -> ParsideResult<(&'a [u8], GroupData)> {
        count(
            Parsers::matter_parser(&cold_code)?,
            amount,
        )(bytes)
            .map(|(rest, value)| (rest, GroupData::Single { value }))
            .map_err(ParsideError::from)
    }

    fn parse_couple<'a>(bytes: &'a [u8], cold_code: &ColdCodes, amount: usize) -> ParsideResult<(&'a [u8], GroupData)> {
        count(
            tuple((
                Parsers::matter_parser(&cold_code)?,
                Parsers::matter_parser(&cold_code)?,
            )),
            amount,
        )(bytes)
            .map(|(rest, value)| (rest, GroupData::Couple { value }))
            .map_err(ParsideError::from)
    }

    fn parse_couple_with_list<'a>(bytes: &'a [u8], cold_code: &ColdCodes, amount: usize) -> ParsideResult<(&'a [u8], GroupData)> {
        count(
            tuple((
                Parsers::matter_parser(&cold_code)?,
                Parsers::matter_list_parser(&cold_code)?,
            )),
            amount,
        )(bytes)
            .map(|(rest, value)| (rest, GroupData::CoupleWithList { value }))
            .map_err(ParsideError::from)
    }

    fn parse_quadruple<'a>(bytes: &'a [u8], cold_code: &ColdCodes, amount: usize) -> ParsideResult<(&'a [u8], GroupData)> {
        count(
            tuple((
                Parsers::matter_parser(&cold_code)?,
                Parsers::matter_parser(&cold_code)?,
                Parsers::matter_parser(&cold_code)?,
                Parsers::matter_parser(&cold_code)?,
            )),
            amount,
        )(bytes)
            .map(|(rest, value)| (rest, GroupData::Quadruple { value }))
            .map_err(ParsideError::from)
    }

    fn parse_quadruple_with_list<'a>(bytes: &'a [u8], cold_code: &ColdCodes, amount: usize) -> ParsideResult<(&'a [u8], GroupData)> {
        count(
            tuple((
                Parsers::matter_parser(&cold_code)?,
                Parsers::matter_parser(&cold_code)?,
                Parsers::matter_parser(&cold_code)?,
                Parsers::matter_list_parser(&cold_code)?,
            )),
            amount,
        )(bytes)
            .map(|(rest, value)| (rest, GroupData::QuadrupleWithList { value }))
            .map_err(ParsideError::from)
    }

    pub fn single(self) -> ParsideResult<Single> {
        match self {
            GroupData::Single { value } => Ok(value),
            _ => Err(ParsideError::NotExist)

        }
    }

    pub fn couple(self) -> ParsideResult<Couple> {
        match self {
            GroupData::Couple { value } => Ok(value),
            _ => Err(ParsideError::NotExist)

        }
    }

    pub fn couple_with_list(self) -> ParsideResult<CoupleWithList> {
        match self {
            GroupData::CoupleWithList { value } => Ok(value),
            _ => Err(ParsideError::NotExist)

        }
    }

    pub fn quadruple(self) -> ParsideResult<Quadruple> {
        match self {
            GroupData::Quadruple { value } => Ok(value),
            _ => Err(ParsideError::NotExist)

        }
    }

    pub fn quadruple_with_list(self) -> ParsideResult<QuadrupleWithList> {
        match self {
            GroupData::QuadrupleWithList { value } => Ok(value),
            _ => Err(ParsideError::NotExist)

        }
    }
}

impl CesrGroup {
    pub fn from_stream_bytes<'a>(bytes: &'a [u8]) -> ParsideResult<(&'a [u8], CesrGroup)> {
        let cold_code = ColdCodes::try_from(bytes[0])?;
        let (bytes, counter) = Parsers::counter_parser(&cold_code)?(bytes)?;
        let amount = counter.count() as usize;
        let code = Codex::from_code(&counter.code())?;

        let (rest, group) =
            match code {
                Codex::AttachedMaterialQuadlets => {
                    unimplemented!()
                }
                Codex::ControllerIdxSigs => {
                    GroupData::parse_single(bytes, &cold_code, amount)?
                }
                Codex::WitnessIdxSigs => {
                    GroupData::parse_single(bytes, &cold_code, amount)?
                }
                Codex::NonTransReceiptCouples => {
                    GroupData::parse_couple(bytes, &cold_code, amount)?
                }
                Codex::TransReceiptQuadruples => {
                    GroupData::parse_quadruple(bytes, &cold_code, amount)?
                }
                Codex::TransIdxSigGroups => {
                    GroupData::parse_quadruple_with_list(bytes, &cold_code, amount)?
                }
                Codex::TransLastIdxSigGroups => {
                    GroupData::parse_couple_with_list(bytes, &cold_code, amount)?
                }
                Codex::FirstSeenReplayCouples => {
                    GroupData::parse_couple(bytes, &cold_code, amount)?
                }
                Codex::SealSourceCouples => {
                    GroupData::parse_couple(bytes, &cold_code, amount)?
                }
                Codex::SadPathSigGroup => {
                    unimplemented!()
                }
                Codex::SadPathSig => {
                    unimplemented!()
                }
                Codex::PathedMaterialQuadlets => {
                    unimplemented!()
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

        let expected_counter = Counter::new(Codex::TransIdxSigGroups.code(), 1);
        assert_eq!(expected_counter, message.counter);

        match message.group {
            GroupData::QuadrupleWithList { value: group } => {
                assert_eq!(1, group.len());
                assert_eq!(
                    MatterCodex::Blake3_256.code().to_string(),
                    group[0].0.code()
                );
                assert_eq!(
                    MatterCodex::Salt_128.code().to_string(),
                    group[0].1.code()
                );
                assert_eq!(
                    MatterCodex::Blake3_256.code().to_string(),
                    group[0].2.code()
                );
            }
            _ => assert!(false, "Unexpected case"),
        }
    }

    #[test]
    pub fn test_parse_controller_idx_sigs() {
        let stream = br#"-AABAABg3q8uNg1A2jhEAdbKGf-QupQhNnmZQx3zIyPLWBe6qqLT5ynytivf9EwJhxyhy87a0x2cezDdil4SsM2xxs0O"#;

        let (_rest, message) = CesrGroup::from_stream_bytes(stream).unwrap();

        let expected_counter = Counter::new(Codex::ControllerIdxSigs.code(), 1);
        assert_eq!(expected_counter, message.counter);

        match message.group {
            GroupData::Single { value: group } => {
                assert_eq!(1, group.len());
                assert_eq!(
                    MatterCodex::Ed25519_Seed.code().to_string(),
                    group[0].code()
                );
            }
            _ => assert!(false, "Unexpected case"),
        }
    }

    #[test]
    pub fn test_parse_non_trans_receipt_couples() {
        let stream = br#"-CABBD8-gMSJ6K1PQ7_gG5ZJn2NkHQJgdkiNrTBz_FWWS_cC0BDc1i44ZX0jaIHh5oNDx-TITbPnI6VEn2nKlqPwkkTF452X7XxYh80tolDpReYwZpnD8TF4Or2v3CpSCikyt6EG"#;

        let (_rest, message) = CesrGroup::from_stream_bytes(stream).unwrap();

        let expected_counter = Counter::new(Codex::NonTransReceiptCouples.code(), 1);
        assert_eq!(expected_counter, message.counter);

        match message.group {
            GroupData::Couple { value: group } => {
                assert_eq!(1, group.len());
                assert_eq!(
                    MatterCodex::Ed25519N.code().to_string(),
                    group[0].0.code()
                );
                assert_eq!(
                    MatterCodex::Ed25519_Sig.code().to_string(),
                    group[0].1.code()
                );
            }
            _ => assert!(false, "Unexpected case"),
        }
    }

    #[test]
    pub fn test_parse_attached_material_quadlets() {
        let stream = br#"-VA--AABAAAEmCc25ETG2m1Ya-tPGuEqsPywOtusQwXKy076ve56IHXzX2bs0xsdQ4dk0XsanstpThg71ynIy-yUDSue6jMD-BABAABfvC7zCIVOVMol9C4AlSALS9JhL8PCdfgRnJgkXG4U11gFyZbsI_J828POrtwtoOmFhs20hoH1pYw4NZr2cdwN-EAB0AAAAAAAAAAAAAAAAAAAAAAE1AAG2023-02-07T15c00c00d025640p00c00"#;

        let (_rest, message) = CesrGroup::from_stream_bytes(stream).unwrap();

        let expected_counter = Counter::new(Codex::AttachedMaterialQuadlets.code(), 62);
        assert_eq!(expected_counter, message.counter);
    }

    #[test]
    pub fn test_parse_trans_last_idx_sig_groups() {
        let stream = br#"-HABEB1f36VmoizOIpBIBv3X4ZiWJQWjtKJ7TMmsZltT0B32-AABAAAKB9u6wyLS9kl_iGVGCqrs-3XqFbyGeOKuiOEA9JZpxI9GMv0GJv2wbY1-sOD_HOJcvXO7LSO8g8MSeRXjtL4I"#;

        let (_rest, message) = CesrGroup::from_stream_bytes(stream).unwrap();

        let expected_counter = Counter::new(Codex::TransLastIdxSigGroups.code(), 1);
        assert_eq!(expected_counter, message.counter);

        match message.group {
            GroupData::CoupleWithList { value: group } => {
                assert_eq!(1, group.len());
                assert_eq!(
                    MatterCodex::Blake3_256.code().to_string(),
                    group[0].0.code()
                );
            }
            _ => assert!(false, "Unexpected case"),
        }
    }
}
