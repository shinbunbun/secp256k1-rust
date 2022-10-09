use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{
        Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, ShrAssign, Sub,
        SubAssign,
    },
};

use rug::ops::Pow;

use crate::pow::PowMod;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FieldElement<T, U>
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
    _marker: PhantomData<fn() -> U>,
}

impl<T, U> FieldElement<T, U>
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
        Self {
            num,
            prime,
            _marker: PhantomData,
        }
    }
}

impl<T, U, V> Pow<V> for FieldElement<T, U>
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
    V: Into<T>,
{
    type Output = Self;

    fn pow(self, exp: V) -> Self::Output {
        self.pow_mod(exp.into(), self.prime.clone())
    }
}

impl<T, U> PowMod<T> for FieldElement<T, U>
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
            _marker: PhantomData,
        }
    }
}

impl<T, U> Add for FieldElement<T, U>
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
            _marker: PhantomData,
        }
    }
}

impl<T, U> Sub for FieldElement<T, U>
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
            _marker: PhantomData,
        }
    }
}

impl<T, U> Mul for FieldElement<T, U>
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
            _marker: PhantomData,
        }
    }
}

impl<T, U> Div for FieldElement<T, U>
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
    U: Clone,
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        self.clone() * other.pow_mod(self.prime.clone() - 2.into(), self.prime)
    }
}

impl<T, U> Mul<U> for FieldElement<T, U>
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
    U: Clone + Into<T>,
{
    type Output = Self;

    fn mul(self, other: U) -> Self::Output {
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
        let a: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(7), Integer::from(13));
        let b: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(6), Integer::from(13));
        let c: FieldElement<i32, i32> = FieldElement::new(7, 13);
        let d: FieldElement<i32, i32> = FieldElement::new(6, 13);

        // test PartialEq
        assert_ne!(a, b);
        assert_ne!(c, d);
        assert!(a == a);
        assert!(c == c);

        // test Debug
        assert_eq!(
            format!("{:?}", a),
            "FieldElement { num: 7, prime: 13, _marker: PhantomData }"
        );

        // test Clone
        let e = a.clone();
        let f = c.clone();

        assert_eq!(a, e);
        assert_eq!(c, f);
    }

    #[test]
    fn test_field_element_pow() {
        let a: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(4), Integer::from(13));
        let b: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(12), Integer::from(13));
        let c: FieldElement<i32, i32> = FieldElement::new(4, 13);
        let d: FieldElement<i32, i32> = FieldElement::new(12, 13);

        assert_eq!(a.pow_mod(Integer::from(3), a.prime.clone()), b);
        assert_eq!(a.pow_mod(Integer::from(-3), a.prime.clone()), b);
        assert_eq!(c.pow_mod(3, c.prime.clone()), d);
        assert_eq!(c.pow_mod(-3, c.prime.clone()), d);
    }

    #[test]
    fn test_field_element_add() {
        let a: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(7), Integer::from(13));
        let b: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(12), Integer::from(13));
        let c: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(6), Integer::from(13));
        let d: FieldElement<i32, i32> = FieldElement::new(7, 13);
        let e: FieldElement<i32, i32> = FieldElement::new(12, 13);
        let f: FieldElement<i32, i32> = FieldElement::new(6, 13);

        assert_eq!(a + b, c);
        assert_eq!(d + e, f);
    }

    #[test]
    fn test_field_element_sub() {
        let a: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(7), Integer::from(13));
        let b: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(12), Integer::from(13));
        let c: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(6), Integer::from(13));
        let d: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(5), Integer::from(13));
        let e: FieldElement<i32, i32> = FieldElement::new(7, 13);
        let f: FieldElement<i32, i32> = FieldElement::new(12, 13);
        let g: FieldElement<i32, i32> = FieldElement::new(6, 13);
        let h: FieldElement<i32, i32> = FieldElement::new(5, 13);

        assert_eq!(c - b.clone(), a);
        assert_eq!(b - a, d);
        assert_eq!(g - f.clone(), e);
        assert_eq!(f - e, h);
    }

    #[test]
    fn test_field_element_mul() {
        let a: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(3), Integer::from(13));
        let b: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(12), Integer::from(13));
        let c: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(10), Integer::from(13));
        let d: FieldElement<i32, i32> = FieldElement::new(3, 13);
        let e: FieldElement<i32, i32> = FieldElement::new(12, 13);
        let f: FieldElement<i32, i32> = FieldElement::new(10, 13);

        assert_eq!(a * b, c);
        assert_eq!(d * e, f);
    }

    #[test]
    fn test_field_element_div() {
        let a: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(3), Integer::from(31));
        let b: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(24), Integer::from(31));
        let c: FieldElement<Integer, Integer> =
            FieldElement::new(Integer::from(4), Integer::from(31));
        let d: FieldElement<i32, i32> = FieldElement::new(3, 31);
        let e: FieldElement<i32, i32> = FieldElement::new(24, 31);
        let f: FieldElement<i32, i32> = FieldElement::new(4, 31);

        assert_eq!(a / b, c);
        assert_eq!(d / e, f);
    }
}
