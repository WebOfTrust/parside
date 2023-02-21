use parside::error::ParsideResult;
pub use parside::message::groups::{WitnessIdxSig, WitnessIdxSigs};
use parside::Group;
use parside::Siger;

pub fn witness_ids_sig_create(siger: Siger) -> WitnessIdxSig {
    WitnessIdxSig::new(siger)
}

pub fn witness_ids_sigs_create(value: Vec<WitnessIdxSig>) -> WitnessIdxSigs {
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
