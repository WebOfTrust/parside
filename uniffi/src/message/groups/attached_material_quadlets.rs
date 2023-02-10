use parside::error::ParsideResult;
use parside::{Matter, Group};
pub use parside::message::groups::AttachedMaterialQuadlets;

pub fn attached_material_quadlets_create(value: Vec<Matter>) -> AttachedMaterialQuadlets {
    AttachedMaterialQuadlets::new(value)
}

pub fn attached_material_quadlets_qb64(attached_material_quadlets: &AttachedMaterialQuadlets) -> ParsideResult<String> {
    attached_material_quadlets.qb64()
}

pub fn attached_material_quadlets_qb64b(attached_material_quadlets: &AttachedMaterialQuadlets) -> ParsideResult<Vec<u8>> {
    attached_material_quadlets.qb64b()
}

pub fn attached_material_quadlets_qb2(attached_material_quadlets: &AttachedMaterialQuadlets) -> ParsideResult<Vec<u8>> {
    attached_material_quadlets.qb2()
}