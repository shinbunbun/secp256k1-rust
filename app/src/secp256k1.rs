use rug::{integer::Order, ops::Pow, Integer};

use crate::{
    hash::{self, create_hmac256},
    point::Point,
    signature::Signature,
};

use field_element::FieldElement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Secp256k1 {
    pub private_key: Option<Integer>,
    pub public_key: Point<FieldElement<Integer>, Integer>,
}

impl Secp256k1 {
    pub fn new_with_public_key(public_key: Point<FieldElement<Integer>, Integer>) -> Self {
        Self {
            private_key: None,
            public_key,
        }
    }

    pub fn new_with_secret(secret: &str) -> Self {
        let private_key = Integer::from_digits(
            hash::create_sha256_from_string(secret).as_slice(),
            Order::MsfBe,
        );
        let public_key = Secp256k1::get_g() * private_key.clone();

        Self {
            private_key: Some(private_key),
            public_key,
        }
    }

    pub fn verify(&self, z: Integer, sig: Signature) -> bool {
        let n = Secp256k1::get_n();
        let g = Secp256k1::get_g();

        let s_inv = sig.s.pow_mod(&(n.clone() - Integer::from(2)), &n).unwrap();
        let u = z * s_inv.clone() % &n;
        let v = sig.r.clone() * s_inv % &n;
        let total = Secp256k1::scalar_multiplication(g, u)
            + Secp256k1::scalar_multiplication(self.public_key.clone(), v);
        if total.x.is_none() {
            panic!("Total is at infinity");
        }
        total.x.unwrap() == Secp256k1::create_field_element(sig.r)
    }

    pub fn sign(&self, z: Integer, k: Integer) -> Signature {
        if self.private_key.is_none() {
            panic!("Private key is not set");
        }

        let n = Secp256k1::get_n();
        let g = Secp256k1::get_g();

        let r = (g * k.clone()).x.unwrap().num;
        let k_inv = k.pow_mod(&(n.clone() - 2), &n).unwrap();
        let mut s = (r.clone() * self.private_key.clone().unwrap() + z) * k_inv % n.clone();
        if s > n.clone() / 2 {
            s = n - s
        }
        Signature { r, s }
    }

    pub fn create_field_element(num: Integer) -> FieldElement<Integer> {
        let p = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);
        FieldElement::new(num, p)
    }

    pub fn create_point(
        x: Option<FieldElement<Integer>>,
        y: Option<FieldElement<Integer>>,
    ) -> Point<FieldElement<Integer>, Integer> {
        let a = Secp256k1::create_field_element(Integer::from(0));
        let b = Secp256k1::create_field_element(Integer::from(7));
        Point::new(x, y, a, b)
    }

    fn scalar_multiplication(
        point: Point<FieldElement<Integer>, Integer>,
        mut coefficient: Integer,
    ) -> Point<FieldElement<Integer>, Integer> {
        coefficient %= Secp256k1::get_n();
        point * coefficient
    }

    fn get_n() -> Integer {
        Integer::from_str_radix(
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16,
        )
        .unwrap()
    }

    pub fn get_g() -> Point<FieldElement<Integer>, Integer> {
        Secp256k1::create_point(
            Some(Secp256k1::create_field_element(
                Integer::from_str_radix(
                    "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                    16,
                )
                .unwrap(),
            )),
            Some(Secp256k1::create_field_element(
                Integer::from_str_radix(
                    "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
                    16,
                )
                .unwrap(),
            )),
        )
    }

    pub fn deterministic_k(&self, mut z: Integer) -> Integer {
        let n = Secp256k1::get_n();
        if z > n {
            z -= n.clone();
        }
        let mut k = [b'\x00'; 32].to_vec();
        let mut v = [b'\x01'; 32].to_vec();
        let z_bytes = z.to_digits::<u8>(Order::MsfBe);
        let secret_bytes = self
            .private_key
            .clone()
            .unwrap()
            .to_digits::<u8>(Order::MsfBe);

        v.push(b'\x00');
        k = create_hmac256(
            &k,
            &[v.clone(), secret_bytes.clone(), z_bytes.clone()].concat(),
        );

        v = create_hmac256(&k, &v);

        v.push(b'\x01');
        k = create_hmac256(&k, ([v.clone(), secret_bytes, z_bytes].concat()).as_slice());

        v = create_hmac256(&k, &v);

        loop {
            v = create_hmac256(&k, &v);

            let candidate = Integer::from_digits(v.as_slice(), Order::MsfBe);
            if candidate >= 1 && candidate < n {
                return candidate;
            }

            v.push(b'\x00');
            k = create_hmac256(&k, &v);

            v = create_hmac256(&k, &v);
        }
    }
}

#[cfg(test)]
mod tests {
    use rug::integer::Order;

    use crate::hash::create_sha256_from_string;

    use super::*;

    #[test]
    fn test_scalar_multiplication() {
        let x = Secp256k1::create_field_element(
            Integer::from_str_radix(
                "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                16,
            )
            .unwrap(),
        );
        let y = Secp256k1::create_field_element(
            Integer::from_str_radix(
                "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
                16,
            )
            .unwrap(),
        );
        let n = Integer::from_str_radix(
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16,
        )
        .unwrap();
        let point = Secp256k1::create_point(Some(x), Some(y));
        let point2 = Secp256k1::create_point(None, None);

        assert_eq!(Secp256k1::scalar_multiplication(point, n), point2);
    }

    #[test]
    fn test_verify() {
        let px = Secp256k1::create_field_element(
            Integer::from_str_radix(
                "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
                16,
            )
            .unwrap(),
        );
        let py = Secp256k1::create_field_element(
            Integer::from_str_radix(
                "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
                16,
            )
            .unwrap(),
        );
        let public_key = Secp256k1::create_point(Some(px), Some(py));

        // signature 1
        let z1 = Integer::from_str_radix(
            "ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60",
            16,
        )
        .unwrap();
        let r1 = Integer::from_str_radix(
            "ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395",
            16,
        )
        .unwrap();
        let s1 = Integer::from_str_radix(
            "68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4",
            16,
        )
        .unwrap();

        // signature 2
        let z2 = Integer::from_str_radix(
            "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
            16,
        )
        .unwrap();
        let r2 = Integer::from_str_radix(
            "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
            16,
        )
        .unwrap();
        let s2 = Integer::from_str_radix(
            "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
            16,
        )
        .unwrap();

        let sec256 = Secp256k1::new_with_public_key(public_key);

        assert!(sec256.verify(z1, Signature { r: r1, s: s1 }));
        assert!(sec256.verify(z2, Signature { r: r2, s: s2 }));
    }

    #[test]
    fn test_sign() {
        let message = Integer::from_digits(
            create_sha256_from_string("my message").as_slice(),
            Order::MsfBe,
        );
        let message2 = Integer::from_digits(
            create_sha256_from_string("my message2").as_slice(),
            Order::MsfBe,
        );

        let sec256 = Secp256k1::new_with_secret("my secret");
        let k = sec256.deterministic_k(message.clone());
        let signature = sec256.sign(message.clone(), k);

        assert!(sec256.verify(message, signature.clone()));
        assert!(!sec256.verify(message2, signature));
    }
}
