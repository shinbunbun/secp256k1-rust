use rug::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub r: Integer,
    pub s: Integer,
}
