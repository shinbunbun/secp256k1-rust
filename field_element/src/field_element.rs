use std::{
    fmt::Debug,
    ops::{Add, BitAnd, Div, Mul, Rem, ShrAssign, Sub},
};

use num_traits::Pow;

use crate::pow::PowMod;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FieldElement<T> {
    pub num: T,
    pub prime: T,
}

impl<T> FieldElement<T>
where
    T: PartialOrd + Debug + Sub<Output = T> + From<i32>,
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

impl<T, U> Pow<U> for FieldElement<T>
where
    T: PartialEq
        + PartialOrd
        + Sub<Output = T>
        + From<i32>
        + Clone
        + Mul<Output = T>
        + BitAnd<Output = T>
        + Rem<Output = T>
        + ShrAssign<i32>
        + Add<Output = T>,
    U: Into<T>,
{
    type Output = Self;

    fn pow(self, exp: U) -> Self::Output {
        self.pow_mod(exp.into(), self.prime.clone())
    }
}

impl<T> PowMod<T> for FieldElement<T>
where
    T: PartialEq
        + PartialOrd
        + Sub<Output = T>
        + From<i32>
        + Clone
        + Mul<Output = T>
        + BitAnd<Output = T>
        + Rem<Output = T>
        + ShrAssign<i32>
        + Add<Output = T>,
{
    fn pow_mod(&self, mut n: T, m: T) -> Self {
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
    T: Add<Output = T> + Clone + PartialEq + Rem<Output = T>,
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
    T: PartialEq + PartialOrd + Sub<Output = T> + Clone + Add<Output = T>,
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
    T: PartialEq + Mul<Output = T> + Clone + Rem<Output = T>,
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
        + Sub<Output = T>
        + From<i32>
        + Clone
        + Mul<Output = T>
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
        self.clone() * other.pow_mod(self.prime.clone() - 2.into(), self.prime)
    }
}

impl<T> Mul<i32> for FieldElement<T>
where
    T: PartialOrd + Debug + Sub<Output = T> + From<i32> + Clone + Mul<Output = T> + Rem<Output = T>,
{
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        self.clone() * Self::new(other.into(), self.prime)
    }
}

#[cfg(test)]
mod test {

    use crate::pow::PowMod;

    use super::FieldElement;
    use rug::Integer;

    #[test]
    fn test_field_element_derive() {
        let a = FieldElement::new(Integer::from(7), Integer::from(13));
        let b = FieldElement::new(Integer::from(6), Integer::from(13));
        let c = FieldElement::new(7, 13);
        let d = FieldElement::new(6, 13);

        // test PartialEq
        assert_ne!(a, b);
        assert_ne!(c, d);
        assert!(a == a);
        assert!(c == c);

        // test Debug
        assert_eq!(format!("{:?}", a), "FieldElement { num: 7, prime: 13 }");

        // test Clone
        let e = a.clone();
        let f = c.clone();

        assert_eq!(a, e);
        assert_eq!(c, f);
    }

    #[test]
    fn test_field_element_pow() {
        let a = FieldElement::new(Integer::from(4), Integer::from(13));
        let b = FieldElement::new(Integer::from(12), Integer::from(13));
        let c = FieldElement::new(4, 13);
        let d = FieldElement::new(12, 13);

        assert_eq!(a.pow_mod(Integer::from(3), a.prime.clone()), b);
        assert_eq!(a.pow_mod(Integer::from(-3), a.prime.clone()), b);
        assert_eq!(c.pow_mod(3, c.prime), d);
        assert_eq!(c.pow_mod(-3, c.prime), d);
    }

    #[test]
    fn test_field_element_add() {
        let a = FieldElement::new(Integer::from(7), Integer::from(13));
        let b = FieldElement::new(Integer::from(12), Integer::from(13));
        let c = FieldElement::new(Integer::from(6), Integer::from(13));
        let d = FieldElement::new(7, 13);
        let e = FieldElement::new(12, 13);
        let f = FieldElement::new(6, 13);

        assert_eq!(a + b, c);
        assert_eq!(d + e, f);
    }

    #[test]
    fn test_field_element_sub() {
        let a = FieldElement::new(Integer::from(7), Integer::from(13));
        let b = FieldElement::new(Integer::from(12), Integer::from(13));
        let c = FieldElement::new(Integer::from(6), Integer::from(13));
        let d = FieldElement::new(Integer::from(5), Integer::from(13));
        let e = FieldElement::new(7, 13);
        let f = FieldElement::new(12, 13);
        let g = FieldElement::new(6, 13);
        let h = FieldElement::new(5, 13);

        assert_eq!(c - b.clone(), a);
        assert_eq!(b - a, d);
        assert_eq!(g - f.clone(), e);
        assert_eq!(f - e, h);
    }

    #[test]
    fn test_field_element_mul() {
        let a = FieldElement::new(Integer::from(3), Integer::from(13));
        let b = FieldElement::new(Integer::from(12), Integer::from(13));
        let c = FieldElement::new(Integer::from(10), Integer::from(13));
        let d = FieldElement::new(3, 13);
        let e = FieldElement::new(12, 13);
        let f = FieldElement::new(10, 13);

        assert_eq!(a * b, c);
        assert_eq!(d * e, f);
    }

    #[test]
    fn test_field_element_div() {
        let a = FieldElement::new(Integer::from(3), Integer::from(31));
        let b = FieldElement::new(Integer::from(24), Integer::from(31));
        let c = FieldElement::new(Integer::from(4), Integer::from(31));
        let d = FieldElement::new(3, 31);
        let e = FieldElement::new(24, 31);
        let f = FieldElement::new(4, 31);

        assert_eq!(a / b, c);
        assert_eq!(d / e, f);
    }
}
