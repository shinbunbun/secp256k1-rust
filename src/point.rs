use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::pow::Pow;

#[derive(PartialEq, Debug, Clone)]
struct Point<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + PartialEq
        + Clone
        + Debug,
{
    x: Option<T>,
    y: Option<T>,
    a: T,
    b: T,
}

impl<T> Point<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Pow<u32>
        + PartialEq
        + Clone
        + Debug
        + From<i32>,
{
    fn new(x: Option<T>, y: Option<T>, a: T, b: T) -> Self {
        if x.is_none() && y.is_none() {
            return Self {
                x: None,
                y: None,
                a,
                b,
            };
        }

        if x.is_none()
            || y.is_none()
            || y.clone().unwrap().pow(2 as u32, None)
                != x.clone().unwrap().pow(3 as u32, None)
                    + a.clone() * x.clone().unwrap()
                    + b.clone()
        {
            panic!("({:?}, {:?}) is not on the curve", x, y);
        }

        Self { x, y, a, b }
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Pow<u32>
        + PartialEq
        + Clone
        + Debug
        + From<i32>,
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
            };
        }

        // 異なる点の加算
        if self != other {
            let s = (y2 - y1.clone()) / (x2.clone() - x1.clone());
            let x3 = s.clone().pow(2 as u32, None) - x1.clone() - x2;
            let y3 = s * (x1 - x3.clone()) - y1;
            return Self::new(Some(x3), Some(y3), self.a, self.b);
        }

        // 同じ点の加算
        let s = (x1.clone().pow(2 as u32, None) * 3.into() + self.a.clone())
            / (T::from(2) * y1.clone());
        let x3 = s.clone().pow(2 as u32, None) - T::from(2) * x1.clone();
        let y3 = s * (x1 - x3.clone()) - y1;
        Self::new(Some(x3), Some(y3), self.a, self.b)
    }
}

mod test {
    use rug::Integer;

    use super::Point;
    use crate::field_element::FieldElement;

    #[test]
    fn test_point_derive() {
        let p1: Point<i32> = Point::new(Some(-1), Some(-1), 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = Point::new(Some(-1), Some(1), 5, 7);

        // test PartialEq
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);

        // test Debug
        assert_eq!(
            format!("{:?}", p1),
            "Point { x: Some(-1), y: Some(-1), a: 5, b: 7 }"
        );

        // test Clone
        let _p4 = p1.clone();
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(None, None, 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = Point::new(Some(-1), Some(1), 5, 7);
        let p4 = Point::new(Some(2), Some(5), 5, 7);
        let p5 = Point::new(Some(3), Some(-7), 5, 7);
        let p6 = Point::new(Some(18), Some(77), 5, 7);

        assert_eq!(p1.clone() + p2.clone(), p2);
        assert_eq!(p2.clone() + p3, p1);
        assert_eq!(p2.clone() + p4, p5);
        assert_eq!(p2.clone() + p2, p6)
    }

    /* #[test]
    fn field_element_point_unit() {
        let a = FieldElement::new(Integer::from(0), Integer::from(223));
        let b = FieldElement::new(Integer::from(7), Integer::from(223));
        let x = FieldElement::new(Integer::from(192), Integer::from(223));
        let y = FieldElement::new(Integer::from(105), Integer::from(223));

        let p1 = Point::new(Some(x), Some(y), a, b);
        println!("{:?}", p1);
    } */
}
