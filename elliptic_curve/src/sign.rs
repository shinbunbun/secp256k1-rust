use crate::{signature::Signature, Point};

pub trait Sign<T, U> {
    fn new_with_public_key(public_key: Point<T, U>) -> Self;
    fn new_with_secret(secret: &str) -> Self;
    fn verify(&self, z: U, sig: Signature) -> bool;
    fn sign(&self, z: U, k: U) -> Signature;
    fn get_n() -> U;
    fn get_g() -> Point<T, U>;
    fn deterministic_k(&self, z: U) -> U;
}
