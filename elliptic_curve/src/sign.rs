use crate::{signature::Signature, Point};

pub trait Sign<T, U> {
    fn new(private_key: Option<U>, public_key: Point<T, U>) -> Self;
    fn generate_key_pair_from_secret(secret: &str) -> Self;
    fn verify(&self, z: U, sig: Signature) -> bool;
    fn sign(&self, z: U, k: U) -> Signature;
    fn get_n() -> U;
    fn get_g() -> Point<T, U>;
    fn deterministic_k(&self, z: U) -> U;
}
