use std::fmt::Debug;

use rug::{ops::Pow, Integer};

use crate::field_element::FieldElement;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct S256Field {
    field: FieldElement<Integer>,
}

impl S256Field {
    pub fn new(num: Integer) -> Self {
        Self {
            field: FieldElement::new(
                num,
                Integer::from(2).pow(256) - Integer::from(2).pow(32) - 977,
            ),
        }
    }
}
