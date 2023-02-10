use parside::error::ParsideResult;
use parside::{Matter, Group};
pub use parside::message::groups::{
    FirstSeenReplayCouple,
    FirstSeenReplayCouples,
};

pub fn first_seen_replay_couple_create(firner: Matter, dater: Matter) -> FirstSeenReplayCouple {
    FirstSeenReplayCouple::new(firner, dater)
}

pub fn first_seen_replay_couples_create(value: Vec<FirstSeenReplayCouple>) -> FirstSeenReplayCouples {
    FirstSeenReplayCouples::new(value)
}

pub fn first_seen_replay_couples_qb64(first_seen_replay_couples: &FirstSeenReplayCouples) -> ParsideResult<String> {
    first_seen_replay_couples.qb64()
}

pub fn first_seen_replay_couples_qb64b(first_seen_replay_couples: &FirstSeenReplayCouples) -> ParsideResult<Vec<u8>> {
    first_seen_replay_couples.qb64b()
}

pub fn first_seen_replay_couples_qb2(first_seen_replay_couples: &FirstSeenReplayCouples) -> ParsideResult<Vec<u8>> {
    first_seen_replay_couples.qb2()
}
