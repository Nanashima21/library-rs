pub fn z_algorithm(s: &str) -> Vec<usize> {
    let s: &[u8] = s.as_bytes();
    z_algorithm_arbitary(s)
}

fn z_algorithm_arbitary<T: Ord>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return vec![];
    }
    let mut ret = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        let mut k = if j+ret[i] <= i { 0 } else { std::cmp::min(j+ret[j]-i, ret[i-j]) };
        while i+k < n && s[k] == s[i+k] {
            k += 1;
        }
        ret[i] = k;
        if j+ret[j] < i+ret[i] {
            j = i;
        }
    }
    ret[0] = n;
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_z_algorithm() {
        assert_eq!(z_algorithm("abracadabra"), &[11, 0, 0, 1, 0, 1, 0, 4, 0, 0, 1]);
        assert_eq!(z_algorithm("ababababa"), &[9, 0, 7, 0, 5, 0, 3, 0, 1]);
    }
}