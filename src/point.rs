use rug::ops::Pow;

#[derive(PartialEq, Debug, Clone)]
struct Point {
    a: i128,
    b: i128,
    x: i128,
    y: i128,
}

impl Point {
    fn new(x: i128, y: i128, a: i128, b: i128) -> Self {
        if y.pow(2) != x.pow(3) + a * x + b {
            panic!("({}, {}) is not on the curve", x, y);
        }
        Self { a, b, x, y }
    }
}

mod test {
    use super::Point;

    #[test]
    fn test_derive() {
        let p1 = Point::new(-1, -1, 5, 7);
        let p2 = Point::new(-1, -1, 5, 7);
        let p3 = Point::new(-1, 1, 5, 7);

        // test PartialEq
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);

        // test Debug
        assert_eq!(format!("{:?}", p1), "Point { a: 5, b: 7, x: -1, y: -1 }");

        // test Clone
        let _p4 = p1.clone();
    }
}
