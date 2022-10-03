use std::ops::Add;

#[derive(PartialEq, Debug)]
struct FieldElement {
    num: u64,
    prime: u64,
}

impl FieldElement {
    fn new(num: u64, prime: u64) -> Self {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Self { num, prime }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        Self {
            num: (self.num + other.num) % self.prime,
            prime: self.prime,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_field_element_derive() {
        use super::FieldElement;

        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);

        // test PartialEq
        assert!(!(a == b));
        assert!(a == a);

        // test Debug
        assert_eq!(format!("{:?}", a), "FieldElement { num: 7, prime: 13 }");
    }

    #[test]
    fn test_field_element_add() {
        use super::FieldElement;

        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(6, 13);

        assert_eq!(a + b, c);
    }
}
