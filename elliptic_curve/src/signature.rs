#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature<T> {
    pub r: T,
    pub s: T,
}

impl<T> Signature<T> {
    pub fn new(r: T, s: T) -> Self {
        Self { r, s }
    }
}
