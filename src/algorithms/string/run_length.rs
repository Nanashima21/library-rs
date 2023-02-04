pub fn run_length(s: &str) -> Vec<(char, usize)> {
    let str = s.chars().collect::<Vec<_>>();
    let mut pre = str[0];
    let mut cnt = 1;
    let mut ret = vec![];
    for c in str.iter().skip(1) {
        if *c != pre {
            ret.push((pre, cnt));
            cnt = 1;
            pre = *c;
        } else {
            cnt += 1;
        }
    }
    ret.push((pre, cnt));
    ret
} 

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run_length() {
        assert_eq!(run_length("abbccc"), vec![('a', 1usize), ('b', 2), ('c', 3)]);
        assert_eq!(run_length("aaabbbbbc"), vec![('a', 3usize), ('b', 5), ('c', 1)]);
        assert_eq!(run_length("001100"), vec![('0', 2usize), ('1', 2), ('0', 2)]);
    }
}