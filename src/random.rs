use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use rug::{integer::Order, Integer};

fn csprng() -> Integer {
    let mut csp_rng = ChaCha20Rng::from_entropy();
    let mut data = [0u8; 32];
    csp_rng.fill_bytes(&mut data);
    Integer::from_digits(&data, Order::Lsf)
}
