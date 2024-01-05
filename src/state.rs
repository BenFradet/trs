use nalgebra::{Matrix4, SMatrix};
use rand::{Rng, distributions::Uniform};

use crate::{series::Series, distribution::Distribution};

pub struct State<R: Rng + Sized> {
    pub matrix: SMatrix<u32, 4, 4>,
    series: Series,
    distribution: Distribution,
    random: R,
    next_val: Option<u32>,
}

impl<R: Rng + Sized> State<R> {
    // todo: next_val should be r.sample(Uniform::new(1, 3))
    pub fn new(r: R, m: Matrix4<u32>) -> State<R> {
        State {
            matrix: m,
            series: Series::new(1, 2, 2),
            distribution: Distribution::new(0.5),
            random: r,
            next_val: None,
        }
    }

    fn current_val(&mut self) -> u32 {
        match self.next_val {
            None => self.random.sample(Uniform::new(1, 3)),
            Some(other) => other,
        }
    }

    fn next_val(&mut self) -> &mut State<R> {
        let max = self.matrix.max();
        let rank = self.rank(max);
        self.next_val = Some(self.series.u_n(rank));
        self
    }

    fn rank(&mut self, max: u32) -> u32 {
        let max_rank = self.series.n(max);
        match max_rank {
            // high excluding
            0 | 1 => self.random.sample(Uniform::new(0, 2)),
            _ => {
                let sampled_rank = self.distribution.sample(&mut self.random);
                // ranks are 0-based, the distribution is 1 based
                (sampled_rank - 1).min(max_rank)
            },
        }
    }

    pub fn shift_right(&mut self) -> &mut State<R> {
        self.matrix = self.matrix
            .remove_column(3)
            .insert_column(0, 0);
        self.matrix[(0, 0)] = self.current_val();
        self.next_val()
    }

    pub fn shift_left(&mut self) -> &mut State<R> {
        self.matrix = self.matrix
            .remove_column(0)
            .insert_column(3, 0);
        self.matrix[(0, 3)] = self.current_val();
        self.next_val()
    }

    pub fn shift_up(&mut self) -> &mut State<R> {
        self.matrix = self.matrix
            .remove_row(0)
            .insert_row(3, 0);
        self.matrix[(3, 0)] = self.current_val();
        self.next_val()
    }

    pub fn shift_down(&mut self) -> &mut State<R> {
        self.matrix = self.matrix
            .remove_row(3)
            .insert_row(0, 0);
        self.matrix[(0, 0)] = self.current_val();
        self.next_val()
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;

    use super::*;

    #[test]
    fn next_val_is_less_than_or_equal_to_max() -> () {
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(12));
        let mut vec = Vec::new();
        for _ in 0..=1000 {
            let res = s.current_val();
            vec.push(res);
        }
        assert!(vec.into_iter().all(|r| r <= 12));
    }

    #[test]
    fn rank_0_or_1_if_max_1() -> () {
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(1));
        let mut vec = Vec::new();
        for _ in 0..=10 {
            let res = s.rank(1);
            vec.push(res);
        }
        assert!(vec.contains(&0));
        assert!(vec.contains(&1));
    }

    #[test]
    fn rank_0_or_1_if_max_2() -> () {
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(1));
        let mut vec = Vec::new();
        for _ in 0..=10 {
            let res = s.rank(2);
            vec.push(res);
        }
        assert!(vec.contains(&0));
        assert!(vec.contains(&1));
    }

    #[test]
    fn rank_possibly_2_if_max_3() -> () {
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(1));
        let mut vec = Vec::new();
        for _ in 0..=10 {
            let res = s.rank(3);
            vec.push(res);
        }
        assert!(vec.contains(&2));
    }

    #[test]
    fn rank_cant_have_more_than_max_rank() -> () {
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(1));
        let mut vec = Vec::new();
        for _ in 0..=1000 {
            let res = s.rank(12);
            vec.push(res);
        }
        // rank of 12 is 4
        assert!(vec.into_iter().all(|r| r <= 4));
    }

    #[test]
    fn rank_can_be_0_if_max_greater_than_2() -> () {
        // the distribution is 1-based, our ranks are 0-based
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(1));
        let mut vec = Vec::new();
        for _ in 0..=10 {
            let res = s.rank(12);
            vec.push(res);
        }
        assert!(vec.contains(&0));
    }

    #[test]
    fn shift_right_fills_left_with_zeroes() -> () {
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(1));
        let res = s.shift_right();
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
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(1));
        let res = s.shift_left();
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
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(1));
        let res = s.shift_up();
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
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(1));
        let res = s.shift_down();
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
