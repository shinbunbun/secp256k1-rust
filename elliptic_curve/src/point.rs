use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, BitAnd, Div, Mul, ShrAssign, Sub},
};

use num_traits::Pow;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Point<T, U> {
    pub x: Option<T>,
    pub y: Option<T>,
    pub a: T,
    pub b: T,
    _maker: PhantomData<fn() -> U>,
}

impl<T, U> Point<T, U>
where
    T: Add<Output = T> + Mul<Output = T> + Pow<u32, Output = T> + PartialEq + Clone + Debug,
{
    pub fn new(x: Option<T>, y: Option<T>, a: T, b: T) -> Self {
        if x.is_none() && y.is_none() {
            return Self {
                x: None,
                y: None,
                a,
                b,
                _maker: PhantomData,
            };
        }

        if x.is_none()
            || y.is_none()
            || y.clone().unwrap().pow(2)
                != x.clone().unwrap().pow(3) + a.clone() * x.clone().unwrap() + b.clone()
        {
            panic!("({:?}, {:?}) is not on the curve", x, y);
        }

        Self {
            x,
            y,
            a,
            b,
            _maker: PhantomData,
        }
    }
}

impl<T, U> Add for Point<T, U>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Pow<u32, Output = T>
        + PartialEq
        + Clone
        + Debug
        + Mul<i32, Output = T>,
    U: Debug + PartialEq,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        // 加法単位元との加算
        if self.x.is_none() {
            return other;
        }
        if other.x.is_none() {
            return self;
        }

        let x1 = self.x.clone().unwrap();
        let y1 = self.y.clone().unwrap();
        let x2 = other.x.clone().unwrap();
        let y2 = other.y.clone().unwrap();

        // 加法逆元との加算
        if x1 == x2 && y1 != y2 {
            return Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
                _maker: PhantomData,
            };
        }

        // 異なる点の加算
        if self != other {
            let s = (y2 - y1.clone()) / (x2.clone() - x1.clone());
            let x3 = s.clone().pow(2) - x1.clone() - x2;
            let y3 = s * (x1 - x3.clone()) - y1;
            return Self::new(Some(x3), Some(y3), self.a, self.b);
        }

        // 同じ点の加算
        let s = (x1.clone().pow(2) * 3 + self.a.clone()) / (y1.clone() * 2);
        let x3 = s.clone().pow(2) - x1.clone() * 2;
        let y3 = s * (x1 - x3.clone()) - y1;
        Self::new(Some(x3), Some(y3), self.a, self.b)
    }
}

impl<T, U> Mul<U> for Point<T, U>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Pow<u32, Output = T>
        + PartialEq
        + Clone
        + Debug
        + Mul<i32, Output = T>,
    U: Debug + Clone + PartialEq + PartialOrd + From<i32> + BitAnd<Output = U> + ShrAssign<i32>,
{
    type Output = Self;

    fn mul(self, rhs: U) -> Self::Output {
        let mut coef = rhs;
        let mut current = self.clone();
        let mut result = Point::new(None, None, self.a, self.b);
        while coef > 0.into() {
            if coef.clone() & 1.into() == 1.into() {
                result = result + current.clone();
            }
            current = current.clone() + current;
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use rug::Integer;

    use super::Point;
    use field_element::FieldElement;

    #[test]
    fn test_point_derive() {
        let p1: Point<i32, i32> = Point::new(Some(-1), Some(-1), 5, 7);
        let p2: Point<i32, i32> = Point::new(Some(-1), Some(-1), 5, 7);
        let p3: Point<i32, i32> = Point::new(Some(-1), Some(1), 5, 7);

        // test PartialEq
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);

        // test Debug
        assert_eq!(
            format!("{:?}", p1),
            "Point { x: Some(-1), y: Some(-1), a: 5, b: 7, _maker: PhantomData }"
        );

        // test Clone
        let _p4 = p1;
    }

    #[test]
    fn test_point_add() {
        let p1: Point<i32, i32> = Point::new(None, None, 5, 7);
        let p2: Point<i32, i32> = Point::new(Some(-1), Some(-1), 5, 7);
        let p3: Point<i32, i32> = Point::new(Some(-1), Some(1), 5, 7);
        let p4: Point<i32, i32> = Point::new(Some(2), Some(5), 5, 7);
        let p5: Point<i32, i32> = Point::new(Some(3), Some(-7), 5, 7);
        let p6: Point<i32, i32> = Point::new(Some(18), Some(77), 5, 7);

        assert_eq!(p1.clone() + p2.clone(), p2);
        assert_eq!(p2.clone() + p3, p1);
        assert_eq!(p2.clone() + p4, p5);
        assert_eq!(p2.clone() + p2, p6)
    }

    #[test]
    fn field_element_point_unit() {
        let a = FieldElement::new(Integer::from(0), Integer::from(223));
        let b = FieldElement::new(Integer::from(7), Integer::from(223));
        let x1 = FieldElement::new(Integer::from(170), Integer::from(223));
        let y1 = FieldElement::new(Integer::from(142), Integer::from(223));
        let x2 = FieldElement::new(Integer::from(60), Integer::from(223));
        let y2 = FieldElement::new(Integer::from(139), Integer::from(223));
        let x3 = FieldElement::new(Integer::from(220), Integer::from(223));
        let y3 = FieldElement::new(Integer::from(181), Integer::from(223));

        let p1: Point<FieldElement<Integer>, Integer> =
            Point::new(Some(x1), Some(y1), a.clone(), b.clone());
        let p2: Point<FieldElement<Integer>, Integer> =
            Point::new(Some(x2), Some(y2), a.clone(), b.clone());
        let p3: Point<FieldElement<Integer>, Integer> = Point::new(Some(x3), Some(y3), a, b);

        // add
        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn scalar_multiplication() {
        let a = FieldElement::new(Integer::from(0), Integer::from(223));
        let b = FieldElement::new(Integer::from(7), Integer::from(223));
        let x1 = FieldElement::new(Integer::from(47), Integer::from(223));
        let y1 = FieldElement::new(Integer::from(71), Integer::from(223));
        let y2 = FieldElement::new(Integer::from(152), Integer::from(223));
        let p1 = Point::new(Some(x1.clone()), Some(y1.clone()), a.clone(), b.clone());
        let p2 = Point::new(Some(x1.clone()), Some(y2.clone()), a.clone(), b.clone());
        let p3 = Point::new(Some(x1.clone()), Some(y1), a.clone(), b.clone());
        let p4 = Point::new(Some(x1), Some(y2), a, b);

        assert_eq!(p1 * 20, p2);
        assert_eq!(p3 * Integer::from(20), p4);
    }
}
