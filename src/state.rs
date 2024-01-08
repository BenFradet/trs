use nalgebra::{Matrix4, SMatrix};
use rand::{distributions::Uniform, Rng};

use crate::{distribution::Distribution, series::Series};

pub struct State {
    pub matrix: SMatrix<u32, 4, 4>,
    pub next_tile: u32,
    series: Series,
    distribution: Distribution,
}

impl State {
    // todo: next_val should be r.sample(Uniform::new(1, 3))
    pub fn new<R: Rng + ?Sized>(r: &mut R, m: Matrix4<u32>) -> State {
        State {
            matrix: m,
            next_tile: r.sample(Uniform::new(1, 3)),
            series: Series::new(1, 2, 2),
            distribution: Distribution::new(0.5),
        }
    }

    //pub fn current_tile(&mut self) -> u32 {
    //    match self.next_tile {
    //        None => {
    //            let res = self.random.sample(Uniform::new(1, 3));
    //            self.next_tile = Some(res);
    //            res
    //        },
    //        Some(other) => other,
    //    }
    //}

    fn gen_next_tile<R: Rng + ?Sized>(&mut self, r: &mut R) -> &mut State {
        let max = self.matrix.max();
        let rank = self.rank(r, max);
        self.next_tile = self.series.u_n(rank);
        self
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

    pub fn shift_right<R: Rng + ?Sized>(&mut self, r: &mut R) -> &mut State {
        self.matrix = self.matrix.remove_column(3).insert_column(0, 0);
        self.matrix[(0, 0)] = self.next_tile;
        self.gen_next_tile(r)
    }

    pub fn shift_left<R: Rng + ?Sized>(&mut self, r: &mut R) -> &mut State {
        self.matrix = self.matrix.remove_column(0).insert_column(3, 0);
        self.matrix[(0, 3)] = self.next_tile;
        self.gen_next_tile(r)
    }

    pub fn shift_up<R: Rng + ?Sized>(&mut self, r: &mut R) -> &mut State {
        self.matrix = self.matrix.remove_row(0).insert_row(3, 0);
        self.matrix[(3, 0)] = self.next_tile;
        self.gen_next_tile(r)
    }

    pub fn shift_down<R: Rng + ?Sized>(&mut self, r: &mut R) -> &mut State {
        self.matrix = self.matrix.remove_row(3).insert_row(0, 0);
        self.matrix[(0, 0)] = self.next_tile;
        self.gen_next_tile(r)
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;

    use super::*;

    #[test]
    fn next_tile_is_less_than_or_equal_to_max() -> () {
        let mut r = OsRng;
        let mut s = State::new(&mut r, Matrix4::repeat(12));
        let mut vec = Vec::new();
        for _ in 0..=1000 {
            let res = s.next_tile;
            vec.push(res);
        }
        assert!(vec.into_iter().all(|r| r <= 12));
    }

    #[test]
    fn rank_0_or_1_if_max_1() -> () {
        let mut r = OsRng;
        let mut s = State::new(&mut r, Matrix4::repeat(1));
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
        let mut s = State::new(&mut r, Matrix4::repeat(1));
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
        let mut s = State::new(&mut r, Matrix4::repeat(1));
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
        let mut s = State::new(&mut r, Matrix4::repeat(1));
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
        let mut s = State::new(&mut r, Matrix4::repeat(1));
        let mut vec = Vec::new();
        for _ in 0..=10 {
            let res = s.rank(&mut r, 12);
            vec.push(res);
        }
        assert!(vec.contains(&0));
    }

    #[test]
    fn shift_right_fills_left_with_zeroes() -> () {
        let mut r = OsRng;
        let mut s = State::new(&mut r, Matrix4::repeat(1));
        let res = s.shift_right(&mut r);
        for i in 0..=3 {
            for j in 0..=3 {
                if i == 0 && j == 0 {
                    let tile = res.matrix[(i, j)];
                    println!("{}", tile);
                    assert!(tile == 1 || tile == 2);
                } else if j == 0 {
                    assert_eq!(res.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.matrix[(i, j)], 1);
                }
            }
        }
    }

    #[test]
    fn shift_left_fills_right_with_zeroes() -> () {
        let mut r = OsRng;
        let mut s = State::new(&mut r, Matrix4::repeat(1));
        let res = s.shift_left(&mut r);
        for i in 0..=3 {
            for j in 0..=3 {
                if i == 0 && j == 3 {
                    let tile = res.matrix[(i, j)];
                    assert!(tile == 1 || tile == 2);
                } else if j == 3 {
                    assert_eq!(res.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.matrix[(i, j)], 1);
                }
            }
        }
    }

    #[test]
    fn shift_up_fills_bottom_with_zeroes() -> () {
        let mut r = OsRng;
        let mut s = State::new(&mut r, Matrix4::repeat(1));
        let res = s.shift_up(&mut r);
        for i in 0..=3 {
            for j in 0..=3 {
                if i == 3 && j == 0 {
                    let tile = res.matrix[(i, j)];
                    assert!(tile == 1 || tile == 2);
                } else if i == 3 {
                    assert_eq!(res.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.matrix[(i, j)], 1);
                }
            }
        }
    }

    #[test]
    fn shift_down_fills_up_with_zeroes() -> () {
        let mut r = OsRng;
        let mut s = State::new(&mut r, Matrix4::repeat(1));
        let res = s.shift_down(&mut r);
        for i in 0..=3 {
            for j in 0..=3 {
                if i == 0 && j == 0 {
                    let tile = res.matrix[(i, j)];
                    assert!(tile == 1 || tile == 2);
                } else if i == 0 {
                    assert_eq!(res.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.matrix[(i, j)], 1);
                }
            }
        }
    }
}
