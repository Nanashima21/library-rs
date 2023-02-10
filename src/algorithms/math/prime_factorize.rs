pub fn prime_factorize(n: usize) -> Vec<(usize, usize)> {
    let mut ret = vec![];
    let mut m = n;
    for i in (2..).take_while(|&x| x*x <= n) {
        if m%i != 0 {
            continue;
        }
        let mut ex: usize = 0;
        while m%i == 0 {
            m /= i;
            ex += 1;
        }
        ret.push((i, ex));
    }
    if m != 1 {
        ret.push((m, 1));
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prime_factorize() {
        assert_eq!(prime_factorize(6), vec![(2usize, 1usize), (3, 1)]);
        assert_eq!(prime_factorize(12), vec![(2usize, 2usize), (3, 1)]);
        assert_eq!(prime_factorize(36), vec![(2usize, 2usize), (3, 2)]);
        assert_eq!(prime_factorize(1000000007), vec![(1000000007usize, 1usize)]);
        assert_eq!(prime_factorize(u32::max_value() as usize), vec![(3usize, 1usize), (5, 1), (17, 1), (257, 1), (65537, 1)])
    }
}