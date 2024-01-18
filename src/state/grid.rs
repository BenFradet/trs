use nalgebra::{SMatrix, Matrix4};
use rand::Rng;

use crate::state::buckets::Buckets;

pub struct Grid {
    pub matrix: SMatrix<u32, 4, 4>,
}

impl Grid {
    pub fn new<R: Rng + ?Sized>(r: &mut R, base_values: Box<[u32]>) -> Grid {
        let grid_size = 16;
        let buckets = Buckets::new(r, base_values, grid_size);
        let elements = buckets.draw(r);
        Grid {
            matrix: Matrix4::from_iterator(elements),
        }
    }

    fn shift(elements: &[u32]) -> (Vec<u32>, bool) {
      let (mut res, mutated) = Self::rec(elements, Vec::new(), false);
      if mutated {
        // not sure that works
        res.insert(0, 0);
        (res, mutated)
      } else {
        (res, mutated)
      }
    }

    // todo: too much game-specific logic, need to abstract things
    fn rec(elements: &[u32], mut acc: Vec<u32>, mutated: bool) -> (Vec<u32>, bool) {
      if !mutated {
        match elements {
          [h1, h2, t @ ..] =>
            if h1 == h2 && h1 > &2 {
              acc.push(h1 * 2);
              Self::rec(t, acc, true)
            } else if h1 + h2 == 3 && h1 < &3 && h2 < &3 {
              acc.push(h1 + h2);
              Self::rec(t, acc, true)
            } else {
              acc.push(*h1);
              Self::rec(&elements[1..], acc, mutated)
            },
          [h, t @ ..] => {
            acc.push(*h);
            Self::rec(t, acc, mutated)
          },
          _ => (acc, mutated),
        }
      } else {
        // todo: find a way to short-circuit
        match elements {
          [h, t @ ..] => {
            acc.push(*h);
            Self::rec(t, acc, mutated)
          },
          _ => (acc, mutated),
        }
      }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn shift_should_not_mutate_if_immutable() -> () {
    let array = [3, 6, 9, 12];
    let (res, mutated) = Grid::shift(&array);
    assert!(!mutated);
    assert_eq!(res, array.to_vec());
  }

  #[test]
  fn shift_should_mutate_if_adjacent_are_same() -> () {
    let array = [12, 12, 3, 6];
    let (res, mutated) = Grid::shift(&array);
    assert!(mutated);
    assert_eq!(res, [0, 24, 3, 6].to_vec());
  }

  #[test]
  fn shift_should_mutate_only_once() -> () {
    let array = [12, 12, 6, 6];
    let (res, mutated) = Grid::shift(&array);
    assert!(mutated);
    assert_eq!(res, [0, 24, 6, 6].to_vec());
  }

  #[test]
  fn shift_should_mutate_1_2() -> () {
    let array = [1, 2, 6, 6];
    let (res, mutated) = Grid::shift(&array);
    assert!(mutated);
    assert_eq!(res, [0, 3, 6, 6].to_vec());
  }

  #[test]
  fn shift_should_mutate_2_1() -> () {
    let array = [2, 1, 6, 6];
    let (res, mutated) = Grid::shift(&array);
    assert!(mutated);
    assert_eq!(res, [0, 3, 6, 6].to_vec());
  }
}