//Fenwick Tree(Binary Index Tree)
pub struct FenwickTree<T> {
    n: usize,
    array: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> FenwickTree<T> {
    pub fn new(n: usize, e: T) -> Self {
        FenwickTree {
            n,
            array: vec![e.clone(); n],
            e,
        }
    }
    //[0,idx)
    pub fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.array[idx-1].clone();
            idx &= idx - 1;
        }
        sum
    }
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
    where
        T: std::ops::AddAssign<U>,
    {
        let n = self.n;
        idx += 1;
        while idx <= n {
            self.array[idx-1] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    // [l,r)
    pub fn sum(&self, l: usize, r: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        self.accum(r) - self.accum(l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fenwick_tree_works() {
        let mut bit = FenwickTree::new(5, 0i64);
        // [1, 2, 3, 4, 5]
        for i in 0..5 {
            bit.add(i, i as i64 + 1);
        }
        assert_eq!(bit.sum(0, 5), 15);
        assert_eq!(bit.sum(0, 4), 10);
        assert_eq!(bit.sum(1, 3), 5);
    }
}