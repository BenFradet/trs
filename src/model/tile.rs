use rand::{distributions::Uniform, Rng};

use crate::math::{distribution::Distribution, series::Series};

pub struct Tile {
    value: u32,
    series: Series,
    distribution: Distribution,
}

impl Tile {
    pub fn new<R: Rng + ?Sized>(r: &mut R) -> Tile {
        Tile {
            value: r.sample(Uniform::new(1, 3)),
            series: Series::new(1, 2, 2),
            distribution: Distribution::new(0.5),
        }
    }

    pub fn current(&self) -> u32 {
        self.value
    }

    pub fn next<R: Rng + ?Sized>(&mut self, r: &mut R, max: u32) -> u32 {
        let rank = self.rank(r, max);
        self.value = self.series.u_n(rank);
        self.value
    }

    fn rank<R: Rng + ?Sized>(&mut self, r: &mut R, max: u32) -> u32 {
        let max_rank = self.series.n(max);
        match max_rank {
            // high excluding
            0 | 1 => r.sample(Uniform::new(0, 2)),
            _ => {
                let sampled_rank = self.distribution.sample(r);
                // ranks are 0-based, the distribution is 1 based
                (sampled_rank - 1).min(max_rank)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;

    use super::*;

    #[test]
    fn next_is_less_than_or_equal_to_max() -> () {
        let mut r = OsRng;
        let mut s = Tile::new(&mut r);
        let mut vec = Vec::new();
        let max = 12;
        for _ in 0..=1000 {
            let res = s.next(&mut r, max);
            vec.push(res);
        }
        assert!(vec.into_iter().all(|r| r <= 12));
    }

    #[test]
    fn rank_0_or_1_if_max_1() -> () {
        let mut r = OsRng;
        let mut s = Tile::new(&mut r);
        let mut vec = Vec::new();
        for _ in 0..=10 {
            let res = s.rank(&mut r, 1);
            vec.push(res);
        }
        assert!(vec.contains(&0));
        assert!(vec.contains(&1));
    }

    #[test]
    fn rank_0_or_1_if_max_2() -> () {
        let mut r = OsRng;
        let mut s = Tile::new(&mut r);
        let mut vec = Vec::new();
        for _ in 0..=10 {
            let res = s.rank(&mut r, 2);
            vec.push(res);
        }
        assert!(vec.contains(&0));
        assert!(vec.contains(&1));
    }

    #[test]
    fn rank_possibly_2_if_max_3() -> () {
        let mut r = OsRng;
        let mut s = Tile::new(&mut r);
        let mut vec = Vec::new();
        for _ in 0..=10 {
            let res = s.rank(&mut r, 3);
            vec.push(res);
        }
        assert!(vec.contains(&2));
    }

    #[test]
    fn rank_cant_have_more_than_max_rank() -> () {
        let mut r = OsRng;
        let mut s = Tile::new(&mut r);
        let mut vec = Vec::new();
        for _ in 0..=1000 {
            let res = s.rank(&mut r, 12);
            vec.push(res);
        }
        // rank of 12 is 4
        assert!(vec.into_iter().all(|r| r <= 4));
    }

    #[test]
    fn rank_can_be_0_if_max_greater_than_2() -> () {
        // the distribution is 1-based, our ranks are 0-based
        let mut r = OsRng;
        let mut s = Tile::new(&mut r);
        let mut vec = Vec::new();
        for _ in 0..=10 {
            let res = s.rank(&mut r, 12);
            vec.push(res);
        }
        assert!(vec.contains(&0));
    }
}