// n <= 4,759,123,141
pub fn is_prime(n: i32) -> bool {
    let n = n as i64;
    let a = [2i64, 7, 61];
    match n {
        _ if n <= 1 => return false,
        _ if a.contains(&n) => return true,
        _ if n%2 == 0 => return false,
        _ => {},
    }
    let mut d = n-1;
    while d%2 == 0 {
        d /= 2;
    }
    for &ai in &a {
        let mut t = d;
        let mut y = mod_pow(ai, t, n as i32);
        while t != n-1 && y != 1 && y != n-1 {
            y = y*y%n;
            t <<= 1;
        }
        if y != n-1 && t%2 == 0 {
            return false;
        }
    } 
    true 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(6));
        assert!(is_prime(7));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(57));
        assert!(!is_prime(58));
        assert!(is_prime(59));
        assert!(!is_prime(60));
        assert!(is_prime(61));
        assert!(!is_prime(62));
        assert!(!is_prime(701928443));
        assert!(is_prime(998244353));
        assert!(!is_prime(1_000_000_000));
        assert!(is_prime(1_000_000_007));
        assert!(is_prime(i32::max_value()));
    }
}

pub fn safe_mod(mut x: i64, m: i64) -> i64 {
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}

pub fn mod_pow(x: i64, mut n: i64, m: i32) -> i64 {
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