use parside::error::ParsideResult;
use parside::{Matter, Group};
pub use parside::message::groups::{
    WitnessIdxSigs,
};

pub fn witness_ids_sigs_create(value: Vec<Matter>) -> WitnessIdxSigs {
    WitnessIdxSigs::new(value)
}

pub fn witness_ids_sigs_qb64(witness_ids_sigs: &WitnessIdxSigs) -> ParsideResult<String> {
    witness_ids_sigs.qb64()
}

pub fn witness_ids_sigs_qb64b(witness_ids_sigs: &WitnessIdxSigs) -> ParsideResult<Vec<u8>> {
    witness_ids_sigs.qb64b()
}

pub fn witness_ids_sigs_qb2(witness_ids_sigs: &WitnessIdxSigs) -> ParsideResult<Vec<u8>> {
    witness_ids_sigs.qb2()
}
