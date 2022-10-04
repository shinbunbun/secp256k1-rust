use std::ops::Add;

use rug::{ops::Pow, Integer};

#[derive(PartialEq, Debug, Clone)]
struct Point {
    x: Option<Integer>,
    y: Option<Integer>,
    a: Integer,
    b: Integer,
}

impl Point {
    fn new(x: Option<Integer>, y: Option<Integer>, a: Integer, b: Integer) -> Self {
        if x.is_none() || y.is_none() {
            return Self {
                x: None,
                y: None,
                a,
                b,
            };
        }

        if y.clone().unwrap().pow(2)
            != x.clone().unwrap().pow(3) + a.clone() * x.clone().unwrap() + b.clone()
        {
            panic!("({:?}, {:?}) is not on the curve", x, y);
        }

        Self { x, y, a, b }
    }
}

/* impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        if self.x.is_none() {
            return other;
        }
        if other.x.is_none() {
            return self;
        }

    }
} */

mod test {
    use rug::Integer;

    use super::Point;

    #[test]
    fn test_point_derive() {
        let p1 = Point::new(
            Some(Integer::from(-1)),
            Some(Integer::from(-1)),
            Integer::from(5),
            Integer::from(7),
        );
        let p2 = Point::new(
            Some(Integer::from(-1)),
            Some(Integer::from(-1)),
            Integer::from(5),
            Integer::from(7),
        );
        let p3 = Point::new(
            Some(Integer::from(-1)),
            Some(Integer::from(1)),
            Integer::from(5),
            Integer::from(7),
        );

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
}
