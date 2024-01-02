#[derive(Debug, Copy, Clone)]
pub struct Series {
    u_0: u32,
    u_1: u32,
    n_0: u32,
    q: u32,
}

impl Series {
    pub fn new(u_0: u32, u_1: u32, q: u32) -> Series {
        Series {
            u_0: u_0,
            u_1: u_1,
            // n_0 = 2 since we have two terms u_0 and u_1
            n_0: 2,
            q: q,
        }
    }

    pub fn u_n(self, n: u32) -> u32 {
        match n {
            0 => self.u_0,
            1 => self.u_1,
            i => (self.u_0 + self.u_1) * self.q.pow(i - self.n_0),
        }
    }

    // reciprocal of u_n
    pub fn n(self, u_n: u32) -> u32 {
        match u_n {
            u_0 if u_0 == self.u_0 => 0,
            u_1 if u_1 == self.u_1 => 1,
            // operation is safe if u_n is generated with the `u_n` function above
            i => ((u_n / (self.u_0 + self.u_1)) as f64).log(self.q as f64) as u32 + self.n_0,
        }
    }

    pub fn u_n_rec(self, n: u32) -> u32 {
        match n {
            0 => self.u_0,
            1 => self.u_1,
            2 => self.u_0 + self.u_1,
            i => self.u_n(i - 1) * self.q,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u_10() -> () {
        assert_same(0, 1);
        assert_same(1, 2);
        assert_same(2, 3);
        assert_same(3, 6);
        assert_same(4, 12);
        assert_same(5, 24);
        assert_same(6, 48);
        assert_same(7, 96);
        assert_same(8, 192);
        assert_same(9, 384);
        assert_same(10, 768);
    }

    fn assert_same(n: u32, expected: u32) -> () {
        let s = Series::new(1, 2, 2);
        assert_eq!(s.u_n(n), expected);
        assert_eq!(s.u_n_rec(n), expected);
        assert_eq!(s.n(expected), n)
    }
}
