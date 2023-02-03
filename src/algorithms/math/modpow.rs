#[allow(clippy::many_single_char_names)]
pub fn safe_mod(mut x: i64, m: i64) -> i64 {
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

pub fn modpow(x: i64, mut n: i64, m: i32) -> i64 {
    if m == 1 {
        return 0;
    }
    let _m = m as u32;
    let mut r: u64 = 1;
    let mut y: u64 = safe_mod(x, m as i64) as u64;
    while n!=0 {
        if (n & 1) > 0 {
            r = (r * y) % (_m as u64);
        }
        y = (y * y) % (_m as u64);
        n >>= 1;
    }
    r as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_safe_mod() {
        assert_eq!(safe_mod(0, 3), 0);
        assert_eq!(safe_mod(1, 3), 1);
        assert_eq!(safe_mod(2, 3), 2);
        assert_eq!(safe_mod(3, 3), 0);
        assert_eq!(safe_mod(4, 3), 1);
        assert_eq!(safe_mod(5, 3), 2);
        assert_eq!(safe_mod(73, 11), 7);
        assert_eq!(safe_mod(2306249155046129918, 6620319213327), 1374210749525);

        assert_eq!(safe_mod(-1, 3), 2);
        assert_eq!(safe_mod(-2, 3), 1);
        assert_eq!(safe_mod(-3, 3), 0);
        assert_eq!(safe_mod(-4, 3), 2);
        assert_eq!(safe_mod(-5, 3), 1);
        assert_eq!(safe_mod(-7170500492396019511, 777567337), 333221848);
    }
    #[test]
    fn test_modpow() {
        assert_eq!(modpow(0, 0, 1), 0);
        assert_eq!(modpow(0, 0, 3), 1);
        assert_eq!(modpow(0, 0, 723), 1);
        assert_eq!(modpow(0, 0, 998244353), 1);
        assert_eq!(modpow(0, 0, i32::max_value()), 1);

        assert_eq!(modpow(0, 1, 1), 0);
        assert_eq!(modpow(0, 1, 3), 0);
        assert_eq!(modpow(0, 1, 723), 0);
        assert_eq!(modpow(0, 1, 998244353), 0);
        assert_eq!(modpow(0, 1, i32::max_value()), 0);

        assert_eq!(modpow(0, i64::max_value(), 1), 0);
        assert_eq!(modpow(0, i64::max_value(), 3), 0);
        assert_eq!(modpow(0, i64::max_value(), 723), 0);
        assert_eq!(modpow(0, i64::max_value(), 998244353), 0);
        assert_eq!(modpow(0, i64::max_value(), i32::max_value()), 0);

        assert_eq!(modpow(1, 0, 1), 0);
        assert_eq!(modpow(1, 0, 3), 1);
        assert_eq!(modpow(1, 0, 723), 1);
        assert_eq!(modpow(1, 0, 998244353), 1);
        assert_eq!(modpow(1, 0, i32::max_value()), 1);

        assert_eq!(modpow(1, 1, 1), 0);
        assert_eq!(modpow(1, 1, 3), 1);
        assert_eq!(modpow(1, 1, 723), 1);
        assert_eq!(modpow(1, 1, 998244353), 1);
        assert_eq!(modpow(1, 1, i32::max_value()), 1);

        assert_eq!(modpow(1, i64::max_value(), 1), 0);
        assert_eq!(modpow(1, i64::max_value(), 3), 1);
        assert_eq!(modpow(1, i64::max_value(), 723), 1);
        assert_eq!(modpow(1, i64::max_value(), 998244353), 1);
        assert_eq!(modpow(1, i64::max_value(), i32::max_value()), 1);

        assert_eq!(modpow(i64::max_value(), 0, 1), 0);
        assert_eq!(modpow(i64::max_value(), 0, 3), 1);
        assert_eq!(modpow(i64::max_value(), 0, 723), 1);
        assert_eq!(modpow(i64::max_value(), 0, 998244353), 1);
        assert_eq!(modpow(i64::max_value(), 0, i32::max_value()), 1);

        assert_eq!(modpow(i64::max_value(), i64::max_value(), 1), 0);
        assert_eq!(modpow(i64::max_value(), i64::max_value(), 3), 1);
        assert_eq!(modpow(i64::max_value(), i64::max_value(), 723), 640);
        assert_eq!(
            modpow(i64::max_value(), i64::max_value(), 998244353),
            683296792
        );
        assert_eq!(
            modpow(i64::max_value(), i64::max_value(), i32::max_value()),
            1
        );

        assert_eq!(modpow(2, 3, 1_000_000_007), 8);
        assert_eq!(modpow(5, 7, 1_000_000_007), 78125);
        assert_eq!(modpow(123, 456, 1_000_000_007), 565291922);
    }
}

