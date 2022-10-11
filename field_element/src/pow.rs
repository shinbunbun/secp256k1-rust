pub trait Pow<T> {
    fn pow(&self, exp: T) -> Self;
}

pub trait PowMod<T> {
    fn pow_mod(&self, n: T, m: T) -> Self;
}

impl PowMod<u32> for i32 {
    fn pow_mod(&self, mut n: u32, m: u32) -> Self {
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
        assert_eq!(2.pow_mod(3_u32, 5_u32), 3);
    }
}
