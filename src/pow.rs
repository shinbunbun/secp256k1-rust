pub trait Pow<T> {
    fn pow(&self, n: T, m: Option<T>) -> Self;
}

impl Pow<u32> for i32 {
    fn pow(&self, mut n: u32, m: Option<u32>) -> Self {
        if m.is_none() {
            let mut num = *self;
            while n > 1 {
                n -= 1;
                num *= self;
            }
            return num;
        }

        let m = m.unwrap();

        // 負の指数に対応
        n %= m - 1;

        // 繰り返し二乗法
        let mut ret = 1;
        let mut x = *self;
        while n > 0 {
            if n & 1 == 1 {
                ret = ret * x % m as i32;
            }
            x = x * x % m as i32;
            n >>= 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_u32() {
        assert_eq!(2.pow(3, None), 8);
        assert_eq!(2.pow(3, Some(5)), 3);
    }
}
