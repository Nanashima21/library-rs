//Segment Tree
pub struct Segtree<F, T> {
    n: usize,
    array: Vec<T>,
    e: T,
    func: F,
}

impl< F: Fn(T, T) -> T, T: Copy + Eq + std::fmt::Debug> Segtree<F, T> {
    pub fn new(_n: usize, e: T, func: F) -> Self {
        let n = _n.next_power_of_two();
        Segtree {
            n,
            array: vec![e; n * 2],
            e,
            func,
        }
    }
    pub fn set(&mut self, mut idx: usize, val: T) {
        idx += self.n - 1;
        self.array[idx] = val;
        while idx > 0 {
            idx = (idx-1)/2;
            self.array[idx] = (self.func)(self.array[idx*2+1], self.array[idx*2+2]);
        }
    }
    //[a,b)
    pub fn query(&self, a: usize, b: usize) -> T {
        self._query(a, b, 0, 0, self.n)
    }
    fn _query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            self.e
        } else if a <= l && r <= b {
            self.array[k]
        } else {
            let vl: T = self._query(a, b, k*2+1, l, (l+r)/2);
            let vr: T = self._query(a, b, k*2+2, (l+r)/2, r);
            (self.func)(vl, vr)
        }
    }
    pub fn get(&self, idx: usize) -> T {
        self.array[idx+self.n-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_segment_tree() {
        //RSQ(Range Sum Queries)
        let mut seg = Segtree::new(5, 0, |a,b| a+b);
        // [1, 2, 3, 4, 5]
        for i in 0..5 {
            seg.set(i, i as i64 + 1);
        }
        assert_eq!(seg.query(0, 5), 15);
        assert_eq!(seg.query(0, 4), 10);
        assert_eq!(seg.query(1, 3), 5);

        //RMQ(Range Maximum Queries)
        let mut seg = Segtree::new(5, 0, |a,b| a.max(b));
        // [1, 5, 4, 2, 3]
        seg.set(0, 1); 
        seg.set(1, 5);
        seg.set(2, 4);
        seg.set(3, 2);
        seg.set(4, 3);
        assert_eq!(seg.query(0, 5), 5);
        assert_eq!(seg.query(3, 5), 3);
        assert_eq!(seg.query(2, 4), 4);
    }
}
