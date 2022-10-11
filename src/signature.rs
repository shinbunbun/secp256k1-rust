use rug::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub r: Integer,
    pub s: Integer,
}

impl Signature {
    pub fn new(r: Integer, s: Integer) -> Self {
        Self { r, s }
    }
}
