use rug::{integer::Order, Integer};

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

impl Signature<Integer> {
    pub fn der(&self) -> Vec<u8> {
        let mut r = vec![];
        if self.r.to_digits::<u8>(Order::MsfBe)[0] >= 0x80 {
            r.push(0x00);
        }
        r.extend_from_slice(&self.r.to_digits::<u8>(Order::MsfBe));

        let mut s = vec![];
        if self.s.to_digits::<u8>(Order::MsfBe)[0] >= 0x80 {
            s.push(0x00);
        }
        s.extend_from_slice(&self.s.to_digits::<u8>(Order::MsfBe));

        let mut der: Vec<u8> = Vec::new();
        der.push(0x02);
        der.push(r.len() as u8);
        der.extend_from_slice(&r);
        der.push(0x02);
        der.push(s.len() as u8);
        der.extend_from_slice(&s);
        der.insert(0, der.len() as u8);
        der.insert(0, 0x30);

        der
    }
}

#[cfg(test)]
mod test {
    use hex::ToHex;
    use rug::Integer;

    use crate::Signature;

    #[test]
    fn test_der() {
        let signature = Signature {
            r: Integer::from_str_radix(
                "37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
                16,
            )
            .unwrap(),
            s: Integer::from_str_radix(
                "8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
                16,
            )
            .unwrap(),
        };

        assert_eq!(signature.der().encode_hex::<String>(), "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
    }
}
