use rug::Integer;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Debug, Clone)]
struct FieldElement {
    num: Integer,
    prime: Integer,
}

impl FieldElement {
    fn new(num: Integer, prime: Integer) -> Self {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Self { num, prime }
    }

    fn pow(&self, mut n: Integer, m: Integer) -> Self {
        // 負の指数に対応
        n %= m.clone() - 1;

        // 繰り返し二乗法
        let mut ret = Integer::from(1);
        let mut x = self.num.clone();
        while n > 0 {
            if n.clone() & Integer::from(1) == 1 {
                ret = ret * x.clone() % m.clone();
            }
            x = x.clone() * x.clone() % m.clone();
            n >>= 1;
        }

        Self {
            num: ret,
            prime: self.prime.clone(),
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        Self {
            num: (self.num + other.num) % self.prime.clone(),
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }

        let num = if self.num >= other.num {
            self.num - other.num
        } else {
            self.prime.clone() - other.num + self.num
        };

        Self {
            num,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        Self {
            num: (self.num * other.num) % self.prime.clone(),
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        self.clone() * other.pow(self.prime.clone() - Integer::from(2), self.prime)
    }
}

#[cfg(test)]
mod test {
    use super::FieldElement;
    use rug::Integer;

    #[test]
    fn test_field_element_derive() {
        let a = FieldElement::new(Integer::from(7), Integer::from(13));
        let b = FieldElement::new(Integer::from(6), Integer::from(13));

        // test PartialEq
        assert!(!(a == b));
        assert!(a == a);

        // test Debug
        assert_eq!(format!("{:?}", a), "FieldElement { num: 7, prime: 13 }");

        // test Clone
        let d = a.clone();
        assert_eq!(a, d);
    }

    #[test]
    fn test_field_element_add() {
        let a = FieldElement::new(Integer::from(7), Integer::from(13));
        let b = FieldElement::new(Integer::from(12), Integer::from(13));
        let c = FieldElement::new(Integer::from(6), Integer::from(13));

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_field_element_sub() {
        let a = FieldElement::new(Integer::from(7), Integer::from(13));
        let b = FieldElement::new(Integer::from(12), Integer::from(13));
        let c = FieldElement::new(Integer::from(6), Integer::from(13));
        let d = FieldElement::new(Integer::from(5), Integer::from(13));

        assert_eq!(c - b.clone(), a);
        assert_eq!(b - a, d);
    }

    #[test]
    fn test_field_element_mul() {
        let a = FieldElement::new(Integer::from(3), Integer::from(13));
        let b = FieldElement::new(Integer::from(12), Integer::from(13));
        let c = FieldElement::new(Integer::from(10), Integer::from(13));

        assert_eq!(a * b, c);
    }

    #[test]
    fn test_field_element_pow() {
        let a = FieldElement::new(Integer::from(3), Integer::from(13));
        let b = FieldElement::new(Integer::from(1), Integer::from(13));

        assert!(a.pow(Integer::from(3), a.prime.clone()) == b);
    }

    #[test]
    fn test_field_element_div() {
        let a = FieldElement::new(Integer::from(3), Integer::from(31));
        let b = FieldElement::new(Integer::from(24), Integer::from(31));
        let c = FieldElement::new(Integer::from(4), Integer::from(31));

        assert_eq!(a / b, c);
    }
}
