use rug::{ops::Pow, Integer};

use crate::{field_element::FieldElement, point::Point};

pub fn create_field_element(num: Integer) -> FieldElement<Integer> {
    let p = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);
    FieldElement::new(num, p)
}

pub fn scalar_multiplication(
    point: Point<FieldElement<Integer>, Integer>,
    mut coefficient: Integer,
) -> Point<FieldElement<Integer>, Integer> {
    coefficient %= Integer::from_str_radix(
        "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap();
    point * coefficient
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_multiplication() {
        let a = create_field_element(Integer::from(0));
        let b = create_field_element(Integer::from(7));
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
        let point: Point<FieldElement<Integer>, Integer> =
            Point::new(Some(x), Some(y), a.clone(), b.clone());
        let point2 = Point::new(None, None, a, b);

        assert_eq!(scalar_multiplication(point, n), point2);
    }
}
