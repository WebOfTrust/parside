use crate::error::ParsideResult;
use crate::message::cold_code::ColdCode;
use crate::message::{Group, GroupItem};
use crate::{nomify, CesrGroup};
use cesride::counter::Codex;
use cesride::Counter;
use nom::multi::many0;

#[derive(Debug, Clone, Default)]
pub struct AttachedMaterialQuadlets {
    pub value: Vec<CesrGroup>,
}

impl Group<CesrGroup> for AttachedMaterialQuadlets {
    const CODE: &'static str = Codex::AttachedMaterialQuadlets;

    fn new(value: Vec<CesrGroup>) -> Self {
        Self { value }
    }

    fn value(&self) -> &Vec<CesrGroup> {
        &self.value
    }
}

impl AttachedMaterialQuadlets {
    pub(crate) fn from_stream_bytes<'a>(
        bytes: &'a [u8],
        _counter: &Counter,
        _cold_code: &ColdCode,
    ) -> ParsideResult<(&'a [u8], AttachedMaterialQuadlets)> {
        let (rest, body) = many0(nomify!(CesrGroup::from_stream_bytes))(bytes)?;
        return Ok((rest, AttachedMaterialQuadlets { value: body }));
    }
}

impl GroupItem for CesrGroup {
    fn qb64(&self) -> ParsideResult<String> {
        match self {
            CesrGroup::ControllerIdxSigsVariant { value } => value.qb64(),
            CesrGroup::WitnessIdxSigsVariant { value } => value.qb64(),
            CesrGroup::NonTransReceiptCouplesVariant { value } => value.qb64(),
            CesrGroup::TransReceiptQuadruplesVariant { value } => value.qb64(),
            CesrGroup::TransIdxSigGroupsVariant { value } => value.qb64(),
            CesrGroup::TransLastIdxSigGroupsVariant { value } => value.qb64(),
            CesrGroup::FirstSeenReplayCouplesVariant { value } => value.qb64(),
            CesrGroup::SealSourceCouplesVariant { value } => value.qb64(),
            CesrGroup::AttachedMaterialQuadletsVariant { value } => value.qb64(),
            CesrGroup::SadPathSigGroupVariant { value } => value.qb64(),
            CesrGroup::SadPathSigVariant { value } => value.qb64(),
            CesrGroup::PathedMaterialQuadletsVariant { value } => value.qb64(),
        }
    }

    fn qb64b(&self) -> ParsideResult<Vec<u8>> {
        match self {
            CesrGroup::ControllerIdxSigsVariant { value } => value.qb64b(),
            CesrGroup::WitnessIdxSigsVariant { value } => value.qb64b(),
            CesrGroup::NonTransReceiptCouplesVariant { value } => value.qb64b(),
            CesrGroup::TransReceiptQuadruplesVariant { value } => value.qb64b(),
            CesrGroup::TransIdxSigGroupsVariant { value } => value.qb64b(),
            CesrGroup::TransLastIdxSigGroupsVariant { value } => value.qb64b(),
            CesrGroup::FirstSeenReplayCouplesVariant { value } => value.qb64b(),
            CesrGroup::SealSourceCouplesVariant { value } => value.qb64b(),
            CesrGroup::AttachedMaterialQuadletsVariant { value } => value.qb64b(),
            CesrGroup::SadPathSigGroupVariant { value } => value.qb64b(),
            CesrGroup::SadPathSigVariant { value } => value.qb64b(),
            CesrGroup::PathedMaterialQuadletsVariant { value } => value.qb64b(),
        }
    }

    fn qb2(&self) -> ParsideResult<Vec<u8>> {
        match self {
            CesrGroup::ControllerIdxSigsVariant { value } => value.qb2(),
            CesrGroup::WitnessIdxSigsVariant { value } => value.qb2(),
            CesrGroup::NonTransReceiptCouplesVariant { value } => value.qb2(),
            CesrGroup::TransReceiptQuadruplesVariant { value } => value.qb2(),
            CesrGroup::TransIdxSigGroupsVariant { value } => value.qb2(),
            CesrGroup::TransLastIdxSigGroupsVariant { value } => value.qb2(),
            CesrGroup::FirstSeenReplayCouplesVariant { value } => value.qb2(),
            CesrGroup::SealSourceCouplesVariant { value } => value.qb2(),
            CesrGroup::AttachedMaterialQuadletsVariant { value } => value.qb2(),
            CesrGroup::SadPathSigGroupVariant { value } => value.qb2(),
            CesrGroup::SadPathSigVariant { value } => value.qb2(),
            CesrGroup::PathedMaterialQuadletsVariant { value } => value.qb2(),
        }
    }
}
