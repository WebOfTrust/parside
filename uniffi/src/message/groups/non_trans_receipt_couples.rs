use parside::error::ParsideResult;
pub use parside::message::groups::{NonTransReceiptCouple, NonTransReceiptCouples};
use parside::Cigar;
use parside::Group;

pub fn non_trans_receipt_couple_create(cigar: Cigar) -> NonTransReceiptCouple {
    NonTransReceiptCouple::new(cigar)
}

pub fn non_trans_receipt_couples_create(
    value: Vec<NonTransReceiptCouple>,
) -> NonTransReceiptCouples {
    NonTransReceiptCouples::new(value)
}

pub fn non_trans_receipt_couples_qb64(
    non_trans_receipt_couples: &NonTransReceiptCouples,
) -> ParsideResult<String> {
    non_trans_receipt_couples.qb64()
}

pub fn non_trans_receipt_couples_qb64b(
    non_trans_receipt_couples: &NonTransReceiptCouples,
) -> ParsideResult<Vec<u8>> {
    non_trans_receipt_couples.qb64b()
}

pub fn non_trans_receipt_couples_qb2(
    non_trans_receipt_couples: &NonTransReceiptCouples,
) -> ParsideResult<Vec<u8>> {
    non_trans_receipt_couples.qb2()
}
