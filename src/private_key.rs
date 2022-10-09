use rug::Integer;

use crate::{field_element::FieldElement, point::Point, secp256k1::get_g};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey {
    pub secret: Integer,
    pub point: Point<FieldElement<Integer>, Integer>,
}

impl PrivateKey {
    pub fn new(secret: Integer) -> Self {
        Self {
            secret: secret.clone(),
            point: get_g() * secret,
        }
    }

    /* pub fn sign(&self, z: Integer) -> Signature {
        let k =
    } */
}
