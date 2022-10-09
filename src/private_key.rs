use rug::Integer;

use crate::{field_element::FieldElement, point::Point, random, secp256k1};

/* #[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey {
    pub secret: Integer,
    pub point: Point<FieldElement<Integer>, Integer>,
} */

/* impl PrivateKey {
    pub fn new(secret: Integer) -> Self {
        Self {
            secret: secret.clone(),
            point: secp256k1::get_g() * secret,
        }
    }

    pub fn sign(&self, z: Integer) -> Signature {
        let n = secp256k1::get_n();
        let g = secp256k1::get_g();
        let k = random::random();
        let r = (g.clone() * k).x.unwrap();
        let k_inv = k.pow_mod(n.clone() - 2, &n).unwrap();
        let s = (z + r.clone() * self.secret.clone()) * k_inv % &n;
    }
} */
