use std::{
    fmt::Debug,
    ops::{
        Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, ShrAssign, Sub,
        SubAssign,
    },
};

use crate::pow::Pow;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FieldElement<T>
where
    T: PartialEq
        + PartialOrd
        + Debug
        + Sub<Output = T>
        + From<i32>
        + Clone
        + DivAssign
        + MulAssign
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + RemAssign
        + BitAnd<Output = T>
        + Rem<Output = T>
        + ShrAssign<i32>
        + Add<Output = T>,
{
    num: T,
    prime: T,
}

impl<T> FieldElement<T>
where
    T: PartialEq
        + PartialOrd
        + Debug
        + Sub<Output = T>
        + From<i32>
        + Clone
        + DivAssign
        + MulAssign
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + RemAssign
        + BitAnd<Output = T>
        + Rem<Output = T>
        + ShrAssign<i32>
        + Add<Output = T>,
{
    pub fn new(num: T, prime: T) -> Self {
        if num >= prime {
            panic!(
                "Num {:?} not in field range 0 to {:?}",
                num,
                prime - 1.into()
            );
        }
        Self { num, prime }
    }
}

impl<T> Pow<T> for FieldElement<T>
where
    T: PartialEq
        + PartialOrd
        + Debug
        + Sub<Output = T>
        + From<i32>
        + Clone
        + DivAssign
        + MulAssign
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + RemAssign
        + BitAnd<Output = T>
        + Rem<Output = T>
        + ShrAssign<i32>
        + Add<Output = T>,
{
    fn pow(&self, mut n: T, m: Option<T>) -> Self {
        // 繰り返し二乗法を使わない場合
        /* if m.is_none() {
            let mut num = self.num.clone();
            if n < 0.into() {
                while n <= 0.into() {
                    n += 1.into();
                    num /= num.clone();
                }
            } else {
                while n > 1.into() {
                    n -= 1.into();
                    num *= num.clone();
                }
            }
            return Self::new(num, self.prime.clone());
        } */

        let m = m.unwrap();

        // 負の指数に対応
        if n < 0.into() {
            n = (m.clone() - 2.into()) * (n.clone() * (-1).into());
        }

        // 繰り返し二乗法
        let mut ret: T = 1.into();
        let mut x = self.num.clone();
        while n > 0.into() {
            if n.clone() & 1.into() == 1.into() {
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

impl<T> Add for FieldElement<T>
where
    T: PartialEq
        + PartialOrd
        + Debug
        + Sub<Output = T>
        + From<i32>
        + Clone
        + DivAssign
        + MulAssign
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + RemAssign
        + BitAnd<Output = T>
        + Rem<Output = T>
        + ShrAssign<i32>
        + Add<Output = T>,
{
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

impl<T> Sub for FieldElement<T>
where
    T: PartialEq
        + PartialOrd
        + Debug
        + Sub<Output = T>
        + From<i32>
        + Clone
        + DivAssign
        + MulAssign
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + RemAssign
        + BitAnd<Output = T>
        + Rem<Output = T>
        + ShrAssign<i32>
        + Add<Output = T>,
{
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

impl<T> Mul for FieldElement<T>
where
    T: PartialEq
        + PartialOrd
        + Debug
        + Sub<Output = T>
        + From<i32>
        + Clone
        + DivAssign
        + MulAssign
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + RemAssign
        + BitAnd<Output = T>
        + Rem<Output = T>
        + ShrAssign<i32>
        + Add<Output = T>,
{
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

impl<T> Div for FieldElement<T>
where
    T: PartialEq
        + PartialOrd
        + Debug
        + Sub<Output = T>
        + From<i32>
        + Clone
        + DivAssign
        + MulAssign
        + Mul<Output = T>
        + AddAssign
        + SubAssign
        + RemAssign
        + BitAnd<Output = T>
        + Rem<Output = T>
        + ShrAssign<i32>
        + Add<Output = T>,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        self.clone() * other.pow(self.prime.clone() - 2.into(), Some(self.prime))
    }
}

#[cfg(test)]
mod test {
    use crate::pow::Pow;

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
    fn test_field_element_pow() {
        let a = FieldElement::new(Integer::from(4), Integer::from(13));
        let b = FieldElement::new(Integer::from(12), Integer::from(13));

        assert_eq!(a.pow(Integer::from(3), Some(a.prime.clone())), b);
        assert_eq!(a.pow(Integer::from(-3), Some(a.prime.clone())), b);
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
    fn test_field_element_div() {
        let a = FieldElement::new(Integer::from(3), Integer::from(31));
        let b = FieldElement::new(Integer::from(24), Integer::from(31));
        let c = FieldElement::new(Integer::from(4), Integer::from(31));

        assert_eq!(a / b, c);
    }
}
