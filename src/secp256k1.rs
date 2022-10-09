use rug::{ops::Pow, Integer};

use crate::{field_element::FieldElement, point::Point};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    r: Integer,
    s: Integer,
}

pub fn create_field_element(num: Integer) -> FieldElement<Integer> {
    let p = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);
    FieldElement::new(num, p)
}

pub fn create_point(
    x: Option<FieldElement<Integer>>,
    y: Option<FieldElement<Integer>>,
) -> Point<FieldElement<Integer>, Integer> {
    let a = create_field_element(Integer::from(0));
    let b = create_field_element(Integer::from(7));
    Point::new(x, y, a, b)
}

pub fn scalar_multiplication(
    point: Point<FieldElement<Integer>, Integer>,
    mut coefficient: Integer,
) -> Point<FieldElement<Integer>, Integer> {
    coefficient %= get_n();
    point * coefficient
}

pub fn verify(point: Point<FieldElement<Integer>, Integer>, z: Integer, sig: Signature) -> bool {
    let n = get_n();
    let s_inv = sig.s.pow_mod(&(n.clone() - Integer::from(2)), &n).unwrap();
    let u = z * s_inv.clone() % &n;
    let v = sig.r.clone() * s_inv % &n;
    let total = scalar_multiplication(get_g(), u) + scalar_multiplication(point, v);
    if total.x.is_none() {
        panic!("Total is at infinity");
    }
    total.x.unwrap() == create_field_element(sig.r)
}

fn get_n() -> Integer {
    Integer::from_str_radix(
        "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap()
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

#[cfg(test)]
mod tests {
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

        assert!(verify(point.clone(), z1, Signature { r: r1, s: s1 }));
        assert!(verify(point, z2, Signature { r: r2, s: s2 }));
    }
}
