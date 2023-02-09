#[allow(clippy::stable_sort_primitive)]
pub fn enum_divisors(n: usize) -> Vec<usize> {
    let mut ret = vec![];
    for i in (1..).take_while(|&x| x*x <= n) {
        if n%i == 0 {
            ret.push(i);
            if i*i < n {
                ret.push(n/i);
            }
        } 
    }
    ret.sort();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_enum_divisors() {
        assert_eq!(enum_divisors(6), vec![1usize, 2, 3, 6]);
        assert_eq!(enum_divisors(12), vec![1usize, 2, 3, 4, 6, 12]);
        assert_eq!(enum_divisors(36), vec![1usize, 2, 3, 4, 6, 9, 12, 18, 36]);
        assert_eq!(enum_divisors(1000000007), vec![1usize, 1000000007]);
        assert_eq!(enum_divisors(u32::max_value() as usize), vec![1usize, 3, 5, 15, 17, 51, 85, 255, 257, 771, 1285, 3855, 4369, 13107, 21845, 65535, 65537, 196611, 327685, 983055, 1114129, 3342387, 5570645, 16711935, 16843009, 50529027, 84215045, 252645135, 286331153, 858993459, 1431655765, 4294967295])
    }
}