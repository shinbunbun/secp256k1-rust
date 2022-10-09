use chrono::Local;
use rug::{rand::RandState, Integer};

pub(crate) fn random(n: Integer) -> Integer {
    let mut rand = RandState::new();
    rand.seed(&Integer::from(Local::now().timestamp_nanos()));
    n.random_below(&mut rand)
}
