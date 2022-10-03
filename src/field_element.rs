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

#[cfg(test)]
mod test {
    #[test]
    fn test_field_element() {
        use super::FieldElement;

        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);

        // test PartialEq
        assert!(!(a == b));
        assert!(a == a);

        // test Debug
        assert_eq!(format!("{:?}", a), "FieldElement { num: 7, prime: 13 }");
    }
}
