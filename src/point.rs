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
            || y.clone().unwrap().pow(2)
                != x.clone().unwrap().pow(3) + a.clone() * x.clone().unwrap() + b.clone()
        {
            panic!("({:?}, {:?}) is not on the curve", x, y);
        }

        Self { x, y, a, b }
    }
}

impl Add for Point {
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
            let x3 = s.clone().pow(2) - x1.clone() - x2;
            let y3 = s * (x1 - x3.clone()) - y1;
            return Self::new(Some(x3), Some(y3), self.a, self.b);
        }

        Self {
            x: None,
            y: None,
            a: self.a,
            b: self.b,
        }
    }
}

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

    #[test]
    fn test_point_add() {
        let p1 = Point::new(None, None, Integer::from(5), Integer::from(7));
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
        let p4 = Point::new(
            Some(Integer::from(2)),
            Some(Integer::from(5)),
            Integer::from(5),
            Integer::from(7),
        );
        let p5 = Point::new(
            Some(Integer::from(3)),
            Some(Integer::from(-7)),
            Integer::from(5),
            Integer::from(7),
        );

        assert_eq!(p1.clone() + p2.clone(), p2);
        assert_eq!(p2.clone() + p3, p1);
        assert_eq!(p2 + p4, p5);
    }
}
