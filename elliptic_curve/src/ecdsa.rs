use crate::{signature::Signature, Point};

pub trait Ecdsa<T, U> {
    fn new(private_key: Option<U>, public_key: Point<T, U>) -> Self;
    fn generate_key_pair_from_secret(secret: &str) -> Self;
    fn generate_public_key_from_coord(x: U, y: U) -> Point<T, U>;
    fn verify(&self, z: U, sig: Signature<U>) -> bool;
    fn sign(&self, z: U, k: U) -> Signature<U>;
    fn get_n() -> U;
    fn get_g() -> Point<T, U>;
    fn deterministic_k(&self, z: U) -> U;
    fn sec(&self) -> Vec<u8>;
}
