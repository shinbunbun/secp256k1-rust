use rug::{rand::RandState, Integer};

use crate::secp256k1::get_n;

pub(crate) fn random() -> Integer {
    let mut rand = RandState::new();
    let n = get_n();
    n.random_below(&mut rand)
}
