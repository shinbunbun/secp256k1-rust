use rug::{ops::Pow, Integer};

#[derive(PartialEq, Debug, Clone)]
struct Point {
    x: Integer,
    y: Integer,
    a: Integer,
    b: Integer,
}

impl Point {
    fn new(x: Integer, y: Integer, a: Integer, b: Integer) -> Self {
        if y.clone().pow(2) != x.clone().pow(3) + a.clone() * x.clone() + b.clone() {
            panic!("({}, {}) is not on the curve", x, y);
        }
        Self { a, b, x, y }
    }
}

mod test {
    use rug::Integer;

    use super::Point;

    #[test]
    fn test_derive() {
        let p1 = Point::new(
            Integer::from(-1),
            Integer::from(-1),
            Integer::from(5),
            Integer::from(7),
        );
        let p2 = Point::new(
            Integer::from(-1),
            Integer::from(-1),
            Integer::from(5),
            Integer::from(7),
        );
        let p3 = Point::new(
            Integer::from(-1),
            Integer::from(1),
            Integer::from(5),
            Integer::from(7),
        );

        // test PartialEq
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);

        // test Debug
        assert_eq!(format!("{:?}", p1), "Point { a: 5, b: 7, x: -1, y: -1 }");

        // test Clone
        let _p4 = p1.clone();
    }
}
