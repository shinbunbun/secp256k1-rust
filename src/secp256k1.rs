use rug::{ops::Pow, Integer};

use crate::{
    field_element::FieldElement,
    point::Point,
    private_key::PrivateKey,
    signature::{self, Signature},
};

pub struct Secp256k1 {
    private_key: Option<PrivateKey>,
    public_key: Option<Point<FieldElement<Integer>, Integer>>,
}

impl Secp256k1 {
    pub fn new(
        private_key: Option<PrivateKey>,
        public_key: Option<Point<FieldElement<Integer>, Integer>>,
    ) -> Self {
        Self {
            private_key,
            public_key,
        }
    }

    pub fn verify(&self, z: Integer, sig: Signature) -> bool {
        let public_key = if self.private_key.is_some() {
            self.private_key.clone().unwrap().point
        } else {
            if self.public_key.is_none() {
                panic!("Public key is not set");
            }
            self.public_key.clone().unwrap()
        };

        let n = get_n();
        let s_inv = sig.s.pow_mod(&(n.clone() - Integer::from(2)), &n).unwrap();
        let u = z * s_inv.clone() % &n;
        let v = sig.r.clone() * s_inv % &n;
        let total = scalar_multiplication(get_g(), u) + scalar_multiplication(public_key, v);
        if total.x.is_none() {
            panic!("Total is at infinity");
        }
        total.x.unwrap() == create_field_element(sig.r)
    }

    pub fn sign(&self, z: Integer, k: Integer) -> Signature {
        if self.private_key.is_none() {
            panic!("Private key is not set");
        }

        let n = get_n();
        let g = get_g();
        let r = (g * k.clone()).x.unwrap().num;
        let k_inv = k.pow_mod(&(n.clone() - 2), &n).unwrap();
        let mut s = (r.clone() * self.private_key.clone().unwrap().secret + z) * k_inv % n.clone();
        if s > n.clone() / 2 {
            s = n - s
        }
        Signature { r, s }
    }
}

fn create_field_element(num: Integer) -> FieldElement<Integer> {
    let p = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);
    FieldElement::new(num, p)
}

fn create_point(
    x: Option<FieldElement<Integer>>,
    y: Option<FieldElement<Integer>>,
) -> Point<FieldElement<Integer>, Integer> {
    let a = create_field_element(Integer::from(0));
    let b = create_field_element(Integer::from(7));
    Point::new(x, y, a, b)
}

fn scalar_multiplication(
    point: Point<FieldElement<Integer>, Integer>,
    mut coefficient: Integer,
) -> Point<FieldElement<Integer>, Integer> {
    coefficient %= get_n();
    point * coefficient
}

fn get_g() -> Point<FieldElement<Integer>, Integer> {
    create_point(
        Some(create_field_element(
            Integer::from_str_radix(
                "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                16,
            )
            .unwrap(),
        )),
        Some(create_field_element(
            Integer::from_str_radix(
                "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
                16,
            )
            .unwrap(),
        )),
    )
}

fn get_n() -> Integer {
    Integer::from_str_radix(
        "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use rug::integer::Order;

    use crate::{
        hash::create_sha256_from_string,
        random::{self, random},
    };

    use super::*;

    #[test]
    fn test_scalar_multiplication() {
        let x = create_field_element(
            Integer::from_str_radix(
                "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
                16,
            )
            .unwrap(),
        );
        let y = create_field_element(
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
        let point = create_point(Some(x), Some(y));
        let point2 = create_point(None, None);

        assert_eq!(scalar_multiplication(point, n), point2);
    }

    #[test]
    fn test_verify() {
        let px = create_field_element(
            Integer::from_str_radix(
                "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c",
                16,
            )
            .unwrap(),
        );
        let py = create_field_element(
            Integer::from_str_radix(
                "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34",
                16,
            )
            .unwrap(),
        );
        let point = create_point(Some(px), Some(py));

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

        let sec256 = Secp256k1::new(None, Some(point));

        assert!(sec256.verify(z1, Signature { r: r1, s: s1 }));
        assert!(sec256.verify(z2, Signature { r: r2, s: s2 }));
    }

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

        let point = get_g() * secret.clone();
        let private_key = PrivateKey::new(secret, point);
        let sec256 = Secp256k1::new(Some(private_key), None);
        let k = random::random(get_n());
        let signature = sec256.sign(message.clone(), k);

        assert!(sec256.verify(message, signature.clone()));
        assert!(!sec256.verify(message2, signature));
    }
}
