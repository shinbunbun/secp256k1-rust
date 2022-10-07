use std::fmt::Debug;

use rug::{ops::Pow, Integer};

use crate::field_element::FieldElement;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct S256Field {
    field: FieldElement<Integer>,
    prime: Integer,
}

impl S256Field {
    pub fn new(num: Integer) -> Self {
        let p = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);
        Self {
            field: FieldElement::new(num, p.clone()),
            prime: p,
        }
    }
}
