use elliptic_curve::{Ecdsa, Point, Signature};
use num_traits::Pow;
use rug::{integer::Order, ops::Pow as OtherPow, Integer};

use field_element::FieldElement;

use crate::hash::{create_hmac256, create_sha256_from_string};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Secp256k1 {
    pub private_key: Option<Integer>,
    pub public_key: Point<FieldElement<Integer>, Integer>,
}

impl Ecdsa<FieldElement<Integer>, Integer> for Secp256k1 {
    fn new(
        private_key: Option<Integer>,
        public_key: Point<FieldElement<Integer>, Integer>,
    ) -> Self {
        Self {
            private_key,
            public_key,
        }
    }

    fn generate_key_pair_from_secret(secret: &str) -> Self {
        let private_key =
            Integer::from_digits(create_sha256_from_string(secret).as_slice(), Order::MsfBe);
        let public_key = Secp256k1::get_g() * private_key.clone();
        Self {
            private_key: Some(private_key),
            public_key,
        }
    }

    fn generate_public_key_from_coord(
        x: Integer,
        y: Integer,
    ) -> Point<FieldElement<Integer>, Integer> {
        let x = Secp256k1::create_field_element(x);
        let y = Secp256k1::create_field_element(y);
        Secp256k1::create_point(Some(x), Some(y))
    }

    fn verify(&self, z: Integer, sig: Signature<Integer>) -> bool {
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

    fn sign(&self, z: Integer, k: Integer) -> Signature<Integer> {
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

    fn get_n() -> Integer {
        Integer::from_str_radix(
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16,
        )
        .unwrap()
    }

    fn get_g() -> Point<FieldElement<Integer>, Integer> {
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

    fn deterministic_k(&self, mut z: Integer) -> Integer {
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

    fn sec(&self, compress: bool) -> Vec<u8> {
        if compress {
            let mut sec = vec![0x02];
            if self.public_key.y.clone().unwrap().num.is_odd() {
                sec[0] = 0x03;
            }
            sec.extend(
                self.public_key
                    .x
                    .clone()
                    .unwrap()
                    .num
                    .to_digits::<u8>(Order::MsfBe),
            );
            sec
        } else {
            let mut sec = vec![0x04];
            sec.extend(
                self.public_key
                    .x
                    .clone()
                    .unwrap()
                    .num
                    .to_digits::<u8>(Order::MsfBe),
            );
            sec.extend(
                self.public_key
                    .y
                    .clone()
                    .unwrap()
                    .num
                    .to_digits::<u8>(Order::MsfBe),
            );
            sec
        }
    }

    fn parse_sec(sec: &[u8]) -> Self {
        if sec[0] == 0x04 {
            let x = Integer::from_digits(&sec[1..33], Order::MsfBe);
            let y = Integer::from_digits(&sec[33..65], Order::MsfBe);
            let public_key = Secp256k1::generate_public_key_from_coord(x, y);
            return Self {
                private_key: None,
                public_key,
            };
        }

        let is_even = sec[0] == 0x02;
        let x = Secp256k1::create_field_element(Integer::from_digits(&sec[1..], Order::MsfBe));
        let alpha =
            x.clone().pow(Integer::from(3)) + Secp256k1::create_field_element(Integer::from(7));
        let beta = Secp256k1::sqrt(alpha);

        let even_beta: FieldElement<Integer>;
        let odd_beta: FieldElement<Integer>;
        if beta.num.is_even() {
            even_beta = beta.clone();
            odd_beta = Secp256k1::create_field_element(x.prime.clone() - beta.num);
        } else {
            even_beta = Secp256k1::create_field_element(x.prime.clone() - beta.clone().num);
            odd_beta = beta;
        }
        if is_even {
            Self {
                private_key: None,
                public_key: Secp256k1::create_point(Some(x), Some(even_beta)),
            }
        } else {
            Self {
                private_key: None,
                public_key: Secp256k1::create_point(Some(x), Some(odd_beta)),
            }
        }
    }
}

impl Secp256k1 {
    fn create_field_element(num: Integer) -> FieldElement<Integer> {
        let p = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);
        FieldElement::new(num, p)
    }

    fn create_point(
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

    fn sqrt(num: FieldElement<Integer>) -> FieldElement<Integer> {
        num.clone().pow((num.prime + 1) / 4)
    }
}

#[cfg(test)]
mod tests {
    use hex::ToHex;
    use rug::integer::Order;

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

        let sec256 = Secp256k1::new(None, public_key);

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

        let sec256 = Secp256k1::generate_key_pair_from_secret("my secret");
        let k = sec256.deterministic_k(message.clone());
        let signature = sec256.sign(message.clone(), k);

        assert!(sec256.verify(message, signature.clone()));
        assert!(!sec256.verify(message2, signature));
    }

    #[test]
    fn test_sec() {
        let private_key_1 = Integer::from(5000);
        let private_key_2 = Integer::from(2018).pow(5);
        let private_key_3 = Integer::from_str_radix("deadbeef12345", 16).unwrap();
        let private_key_4 = Integer::from(5001);
        let private_key_5 = Integer::from(2019).pow(5);
        let private_key_6 = Integer::from_str_radix("deadbeef54321", 16).unwrap();
        let public_key_1 = Secp256k1::get_g() * private_key_1.clone();
        let public_key_2 = Secp256k1::get_g() * private_key_2.clone();
        let public_key_3 = Secp256k1::get_g() * private_key_3.clone();
        let public_key_4 = Secp256k1::get_g() * private_key_4.clone();
        let public_key_5 = Secp256k1::get_g() * private_key_5.clone();
        let public_key_6 = Secp256k1::get_g() * private_key_6.clone();

        let sec256_1 = Secp256k1::new(Some(private_key_1), public_key_1);
        let sec256_2 = Secp256k1::new(Some(private_key_2), public_key_2);
        let sec256_3 = Secp256k1::new(Some(private_key_3), public_key_3);
        let sec256_4 = Secp256k1::new(Some(private_key_4), public_key_4);
        let sec256_5 = Secp256k1::new(Some(private_key_5), public_key_5);
        let sec256_6 = Secp256k1::new(Some(private_key_6), public_key_6);

        assert_eq!(sec256_1.sec(false).encode_hex::<String>(), "04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10");
        assert_eq!(sec256_2.sec(false).encode_hex::<String>(), "04027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06");
        assert_eq!(sec256_3.sec(false).encode_hex::<String>(), "04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd59961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121");
        assert_eq!(
            sec256_4.sec(true).encode_hex::<String>(),
            "0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1"
        );
        assert_eq!(
            sec256_5.sec(true).encode_hex::<String>(),
            "02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701"
        );
        assert_eq!(
            sec256_6.sec(true).encode_hex::<String>(),
            "0296be5b1292f6c856b3c5654e886fc13511462059089cdf9c479623bfcbe77690"
        );

        assert_eq!(
            Secp256k1::parse_sec(&sec256_1.sec(false)).public_key,
            sec256_1.public_key
        );
        assert_eq!(
            Secp256k1::parse_sec(&sec256_2.sec(false)).public_key,
            sec256_2.public_key
        );
        assert_eq!(
            Secp256k1::parse_sec(&sec256_3.sec(false)).public_key,
            sec256_3.public_key
        );
        assert_eq!(
            Secp256k1::parse_sec(&sec256_4.sec(true)).public_key,
            sec256_4.public_key
        );
        assert_eq!(
            Secp256k1::parse_sec(&sec256_5.sec(true)).public_key,
            sec256_5.public_key
        );
        assert_eq!(
            Secp256k1::parse_sec(&sec256_6.sec(true)).public_key,
            sec256_6.public_key
        );
    }
}
