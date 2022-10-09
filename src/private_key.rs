use rug::Integer;

use crate::{field_element::FieldElement, point::Point};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateKey {
    pub secret: Integer,
    pub point: Point<FieldElement<Integer>, Integer>,
}

impl PrivateKey {
    pub fn new(secret: Integer, point: Point<FieldElement<Integer>, Integer>) -> Self {
        Self { secret, point }
    }
}
