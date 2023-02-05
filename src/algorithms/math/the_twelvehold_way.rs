const MOD: usize = 1_000_000_007;
//const MOD: usize = 998_244_353;

//写像12相 : https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/5
pub struct TwelvefoldWay {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
}

impl TwelvefoldWay {
    pub fn new(n: usize) -> Self {
        assert!(n <= 1e9 as usize);
        let mut fact = vec![1; n+1];
        let mut inv_fact = vec![1; n+1];
        fact[0] = 1;
        for i in 1..=n {
            fact[i] = i * fact[i-1] % MOD;
        }
        inv_fact[n] = TwelvefoldWay::modpow(fact[n], MOD-2);
        for i in (1..n).rev() {
            inv_fact[i] = inv_fact[i+1] * (i+1) % MOD; 
        }
        TwelvefoldWay {
            fact,
            inv_fact,
        }
    }
    pub fn perm(&self, n: usize, k: usize) -> usize {
        assert!(n >= k);
        self.fact[n] % MOD * self.inv_fact[n-k] % MOD
    }
    pub fn comb(&self, n: usize, k: usize) -> usize {
        assert!(n >= k);
        self.fact[n] * self.inv_fact[k] % MOD * self.inv_fact[n-k] % MOD
    }
    pub fn modpow(mut n: usize, mut r: usize) -> usize {
        let mut ret = 1;
        while r > 0 {
            if r & 1 == 1 {
                ret = ret * n % MOD;
            }
            n = n * n % MOD;
            r /= 2;
        }
        ret
    }
    pub fn modinv(x: usize) -> usize {
        TwelvefoldWay::modpow(x, MOD-2)
    } 
    // 1. n個の玉を区別, x個の箱を区別, 配り方に制限なし
    pub fn bab1(n: usize, x: usize) -> usize {
        TwelvefoldWay::modpow(x, n)
    }
    // 2. n個の玉を区別, x個の箱を区別, 箱には1個以内(単射)
    pub fn bab2(&self, n: usize, x: usize) -> usize {
        if n > x { 0 } else { self.perm(x, n) }
    }
    // 3. n個の玉を区別, x個の箱を区別, 箱には1個以上(全射) O(xlogn)
    pub fn bab3(&self, n: usize, x: usize) -> usize {
        let mut ret = 0;
        for i in 0..=x {
            if (x-i) % 2 == 0 {
                ret += self.comb(x, i) * TwelvefoldWay::modpow(i, n) % MOD;
            } else {
                ret += MOD - self.comb(x, i) * TwelvefoldWay::modpow(i, n) % MOD;
            }
            ret %= MOD;
        }
        ret
    }
    // 4. n個の玉を区別しない, x個の箱を区別, 配り方に制限なし
    pub fn bab4(&self, n: usize, x: usize) -> usize {
        self.comb(n+x-1, n)
    }
    // 5. n個の玉を区別しない, x個の箱を区別, 箱には1個以内(単射)
    pub fn bab5(&self, n: usize, x: usize) -> usize {
        if n > x { 0 } else { self.comb(x, n) }
    }
    // 6. n個の玉を区別しない, x個の箱を区別, 箱には1個以上(全射)
    pub fn bab6(&self, n: usize, x: usize) -> usize {
        if n < x { 0 } else { self.comb(n-1, n-x) }
    }
    // 7. n個の玉を区別, x個の箱を区別しない, 配り方に制限なし O(x^2logn)
    pub fn bab7(&self, n: usize, x: usize) -> usize {
        let mut ret = 0; 
        for i in 0..=x {
            ret += self.bab3(n, i) * self.inv_fact[i] % MOD;
            ret %= MOD;
        }
        ret
    }
    // 8. n個の玉を区別, x個の箱を区別しない, 箱には1個以内(単射)
    pub fn bab8(n: usize, x: usize) -> usize {
        (n <= x) as usize
    }
    // 9. n個の玉を区別, x個の箱を区別しない, 箱には1個以上(全射)
    pub fn bab9(&self, n: usize, x: usize) -> usize {
        self.bab3(n, x) * self.inv_fact[x] % MOD
    }
    // 10. n個の玉を区別しない, x個の箱を区別しない, 配り方に制限なし O(nx)
    pub fn bab10(n: usize, x: usize) -> usize {
        let mut dp = vec![vec![0; x+1]; n+1];
        dp[0][0] = 1;
        for i in 0..=n {
            for j in 1..=x {
                if i as isize - j as isize >= 0 {
                    dp[i][j] = (dp[i][j-1] + dp[i-j][j]) % MOD;
                } else {
                    dp[i][j] = dp[i][j-1];
                }
            }
        }
        dp[n][x]
    }
    // 11. n個の玉を区別しない, x個の箱を区別しない, 箱には1個以内(単射)
    pub fn bab11(n: usize, x: usize) -> usize {
        TwelvefoldWay::bab8(n, x)
    }
    // 12. n個の玉を区別しない, x個の箱を区別しない, 箱には1個以上(全射)
    pub fn bab12(n: usize, x: usize) -> usize {
        if n < x { 0 } else { TwelvefoldWay::bab10(n-x, x) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_the_twelvehold_way() {
        // 1.
        assert_eq!(TwelvefoldWay::bab1(2, 3), 9);
        assert_eq!(TwelvefoldWay::bab1(10, 5), 9765625);
        assert_eq!(TwelvefoldWay::bab1(100, 100), 424090053);
        // 2.
        assert_eq!(TwelvefoldWay::new(3).bab2(2, 3), 6);
        assert_eq!(TwelvefoldWay::new(2).bab2(3, 2), 0);
        assert_eq!(TwelvefoldWay::new(100).bab2(100, 100), 437918130);
        // 3.
        assert_eq!(TwelvefoldWay::new(3).bab3(4, 3), 36);
        assert_eq!(TwelvefoldWay::new(3).bab3(10, 3), 55980);
        assert_eq!(TwelvefoldWay::new(100).bab3(100, 100), 437918130);
        // 4.
        assert_eq!(TwelvefoldWay::new(5+3).bab4(5, 3), 21);
        assert_eq!(TwelvefoldWay::new(10+5).bab4(10, 5), 1001);
        assert_eq!(TwelvefoldWay::new(100+100).bab4(100, 100), 703668401);
        // 5.
        assert_eq!(TwelvefoldWay::new(5).bab5(3, 5), 10);
        assert_eq!(TwelvefoldWay::new(10).bab5(5, 10), 252);
        assert_eq!(TwelvefoldWay::new(200).bab5(100, 200), 407336795);
        // 6.
        assert_eq!(TwelvefoldWay::new(5).bab6(5, 3), 6);
        assert_eq!(TwelvefoldWay::new(10).bab6(10, 5), 126);
        assert_eq!(TwelvefoldWay::new(100).bab6(100, 30), 253579538);
        // 7.
        assert_eq!(TwelvefoldWay::new(5).bab7(3, 5), 5);
        assert_eq!(TwelvefoldWay::new(3).bab7(5, 3), 41);
        assert_eq!(TwelvefoldWay::new(100).bab7(100, 100), 193120002);
        // 8.
        assert_eq!(TwelvefoldWay::bab8(5, 10), 1);
        assert_eq!(TwelvefoldWay::bab8(200, 100), 0);
        // 9.
        assert_eq!(TwelvefoldWay::new(3).bab9(4, 3), 6);
        assert_eq!(TwelvefoldWay::new(5).bab9(10, 5), 42525);
        assert_eq!(TwelvefoldWay::new(30).bab9(100, 30), 203169470);
        // 10.
        assert_eq!(TwelvefoldWay::bab10(5, 3), 5);
        assert_eq!(TwelvefoldWay::bab10(10, 5), 30);
        assert_eq!(TwelvefoldWay::bab10(100, 100), 190569292);
        // 11.
        assert_eq!(TwelvefoldWay::bab11(5, 10), 1);
        assert_eq!(TwelvefoldWay::bab11(200, 100), 0);
        // 12.
        assert_eq!(TwelvefoldWay::bab12(10, 5), 7);
        assert_eq!(TwelvefoldWay::bab12(30, 15), 176);
        assert_eq!(TwelvefoldWay::bab12(100, 30), 3910071);
    }
}