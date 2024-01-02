use nalgebra::{Matrix4, SMatrix};
use rand::{Rng, distributions::Uniform};

use crate::{series::Series, distribution::Distribution};

pub struct State<R: Rng + Sized> {
    pub matrix: SMatrix<u32, 4, 4>,
    series: Series,
    distribution: Distribution,
    random: R,
    next_val: u32,
}

impl<R: Rng + Sized> State<R> {
    // todo: next_val should be r.sample(Uniform::new(1, 2))
    pub fn new(r: R, m: Matrix4<u32>) -> State<R> {
        State {
            matrix: m,
            series: Series::new(1, 2, 2),
            distribution: Distribution::new(0.5),
            random: r,
            next_val: 1,
        }
    }

    fn next_val(&mut self) -> &mut State<R> {
        let max = self.matrix.max();
        let rank = self.rank(max);
        self.next_val = self.series.u_n(rank);
        self
    }

    fn rank(&mut self, max: u32) -> u32 {
        let max_rank = self.series.n(max);
        match max_rank {
            0 | 1 => self.random.sample(Uniform::new(0, 1)),
            _ => {
                let sampled_rank = self.distribution.sample(&mut self.random);
                (sampled_rank - 1).min(max_rank)
            },
        }
    }

    pub fn shift_right(&mut self) -> &mut State<R> {
        self.matrix = self.matrix
            .remove_column(3)
            .insert_column(0, 0);
        self.matrix[(0, 0)] = self.next_val;
        self.next_val()
    }

    pub fn shift_left(&mut self) -> &mut State<R> {
        self.matrix = self.matrix
            .remove_column(0)
            .insert_column(3, 0);
        self.matrix[(0, 3)] = self.next_val;
        self.next_val()
    }

    pub fn shift_up(&mut self) -> &mut State<R> {
        self.matrix = self.matrix
            .remove_row(0)
            .insert_row(3, 0);
        self.matrix[(3, 0)] = self.next_val;
        self.next_val()
    }

    pub fn shift_down(&mut self) -> &mut State<R> {
        self.matrix = self.matrix
            .remove_row(3)
            .insert_row(0, 0);
        self.matrix[(0, 0)] = self.next_val;
        self.next_val()
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;

    use super::*;

    #[test]
    fn shift_right_fills_left_with_zeroes() -> () {
        let r = OsRng;
        let mut s = State::new(r, Matrix4::repeat(1));
        let res = s.shift_right();
        for i in 0..=3 {
            for j in 0..=3 {
                if j == 0 {
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
                if j == 3 {
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
                if i == 3 {
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
                if i == 0 {
                    assert_eq!(res.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.matrix[(i, j)], 1);
                }
            }
        }
    }
}
