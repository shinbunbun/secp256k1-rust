use rug::Integer;

use crate::{
    field_element::FieldElement,
    point::Point,
    secp256k1::{self},
    signature::Signature,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey {
    pub secret: Integer,
    pub point: Point<FieldElement<Integer>, Integer>,
}

impl PrivateKey {
    pub fn new(secret: Integer) -> Self {
        Self {
            secret: secret.clone(),
            point: secp256k1::get_g() * secret,
        }
    }

    pub fn sign(&self, z: Integer, k: Integer) -> Signature {
        let n = secp256k1::get_n();
        let g = secp256k1::get_g();
        let r = (g * k.clone()).x.unwrap().num;
        let k_inv = k.pow_mod(&(n.clone() - 2), &n).unwrap();
        let mut s = (r.clone() * self.secret.clone() + z) * k_inv % n.clone();
        if s > n.clone() / 2 {
            s = n - s
        }
        Signature { r, s }
    }
}

#[cfg(test)]
mod test {
    use rug::integer::Order;

    use crate::{hash::create_sha256_from_string, random, secp256k1::verify};

    use super::*;

    #[test]
    fn test_sign() {
        let secret = Integer::from_digits(
            create_sha256_from_string("my secret").as_slice(),
            Order::LsfBe,
        );
        let message = Integer::from_digits(
            create_sha256_from_string("my message").as_slice(),
            Order::LsfBe,
        );
        let message2 = Integer::from_digits(
            create_sha256_from_string("my message2").as_slice(),
            Order::LsfBe,
        );

        let private_key = PrivateKey::new(secret);
        let k = random::random();
        let signature = private_key.sign(message.clone(), k);

        assert!(verify(
            private_key.point.clone(),
            message,
            signature.clone()
        ));
        assert!(!verify(private_key.point, message2, signature));
    }
}
