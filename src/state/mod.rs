use rand::{distributions::Uniform, Rng};

use crate::{math::distribution::Distribution, math::series::Series, state::direction::Direction, state::grid::Grid};

pub mod buckets;
pub mod grid;
pub mod direction;
pub mod dimension;

pub struct State {
    pub grid: Grid,
    pub next_tile: u32,
    series: Series,
    distribution: Distribution,
}

impl State {
    pub fn new<R: Rng + ?Sized>(r: &mut R, base_values: Box<[u32]>) -> State {
        State {
            grid: Grid::new(r, base_values),
            next_tile: r.sample(Uniform::new(1, 3)),
            series: Series::new(1, 2, 2),
            distribution: Distribution::new(0.5),
        }
    }

    pub fn shift<R: Rng + ?Sized>(&mut self, r: &mut R, direction: Direction) -> &mut State {
        match direction {
            Direction::Up => {
                self.grid.matrix = self.grid.matrix.remove_row(0).insert_row(3, 0);
                self.grid.matrix[(3, 0)] = self.next_tile;
            },
            Direction::Down => {
                self.grid.matrix = self.grid.matrix.remove_row(3).insert_row(0, 0);
                self.grid.matrix[(0, 0)] = self.next_tile;
            }
            Direction::Left => {
                self.grid.matrix = self.grid.matrix.remove_column(0).insert_column(3, 0);
                self.grid.matrix[(0, 3)] = self.next_tile;
            },
            Direction::Right => {
                self.grid.matrix = self.grid.matrix.remove_column(3).insert_column(0, 0);
                self.grid.matrix[(0, 0)] = self.next_tile;
            }
        }
        self.gen_next_tile(r)
    }

    fn gen_next_tile<R: Rng + ?Sized>(&mut self, r: &mut R) -> &mut State {
        let max = self.grid.matrix.max();
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
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;

    use super::*;

    #[test]
    fn next_tile_is_less_than_or_equal_to_max() -> () {
        let mut r = OsRng;
        let s = State::new(&mut r, Box::new([1, 1, 1, 1, 1, 1, 6]));
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
        let mut s = State::new(&mut r, Box::new([15, 1]));
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
        let mut s = State::new(&mut r, Box::new([12, 1]));
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
        let mut s = State::new(&mut r, Box::new([12, 1]));
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
        let mut s = State::new(&mut r, Box::new([12, 1]));
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
        let mut s = State::new(&mut r, Box::new([12, 1]));
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
        let mut s = State::new(&mut r, Box::new([0, 16]));
        let res = s.shift(&mut r, Direction::Right);
        for i in 0..=3 {
            for j in 0..=3 {
                if i == 0 && j == 0 {
                    let tile = res.grid.matrix[(i, j)];
                    println!("{}", tile);
                    assert!(tile == 1 || tile == 2);
                } else if j == 0 {
                    assert_eq!(res.grid.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.grid.matrix[(i, j)], 1);
                }
            }
        }
    }

    #[test]
    fn shift_left_fills_right_with_zeroes() -> () {
        let mut r = OsRng;
        let mut s = State::new(&mut r, Box::new([0, 16]));
        let res = s.shift(&mut r, Direction::Left);
        for i in 0..=3 {
            for j in 0..=3 {
                if i == 0 && j == 3 {
                    let tile = res.grid.matrix[(i, j)];
                    assert!(tile == 1 || tile == 2);
                } else if j == 3 {
                    assert_eq!(res.grid.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.grid.matrix[(i, j)], 1);
                }
            }
        }
    }

    #[test]
    fn shift_up_fills_bottom_with_zeroes() -> () {
        let mut r = OsRng;
        let mut s = State::new(&mut r, Box::new([0, 16]));
        let res = s.shift(&mut r, Direction::Up);
        for i in 0..=3 {
            for j in 0..=3 {
                if i == 3 && j == 0 {
                    let tile = res.grid.matrix[(i, j)];
                    assert!(tile == 1 || tile == 2);
                } else if i == 3 {
                    assert_eq!(res.grid.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.grid.matrix[(i, j)], 1);
                }
            }
        }
    }

    #[test]
    fn shift_down_fills_up_with_zeroes() -> () {
        let mut r = OsRng;
        let mut s = State::new(&mut r, Box::new([0, 16]));
        let res = s.shift(&mut r, Direction::Down);
        for i in 0..=3 {
            for j in 0..=3 {
                if i == 0 && j == 0 {
                    let tile = res.grid.matrix[(i, j)];
                    assert!(tile == 1 || tile == 2);
                } else if i == 0 {
                    assert_eq!(res.grid.matrix[(i, j)], 0);
                } else {
                    assert_eq!(res.grid.matrix[(i, j)], 1);
                }
            }
        }
    }
}
