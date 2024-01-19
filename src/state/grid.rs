use nalgebra::{SMatrix, Matrix4, Vector4, RowVector4};
use rand::Rng;

use crate::state::buckets::Buckets;

use super::{direction::Direction, dimension::Dimension};

pub struct Grid {
    // todo: remove pub
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

    // todo: handle next tile
    pub fn mov(&mut self, direction: Direction) -> &mut Grid {
        match direction {
            Direction::Up => self.shift_grid(Dimension::Col, false),
            Direction::Down => self.shift_grid(Dimension::Col, true),
            Direction::Left => self.shift_grid(Dimension::Row, false),
            Direction::Right => self.shift_grid(Dimension::Row, true),
        }
    }

    fn shift_grid(&mut self, dim: Dimension, reverse_needed: bool) -> &mut Grid {
        let size = if dim == Dimension::Col { self.matrix.ncols() } else { self.matrix.nrows() };
        for i in 0..size {
            if let Some(mut elements) = Self::get_line(self.matrix, i, dim) {
                if reverse_needed { elements.reverse() }
                let (new_line, mutated) = Self::shift_line(&elements);
                if mutated {
                    if dim == Dimension::Col {
                        self.matrix.set_column(i, &Vector4::from_iterator(new_line));
                    } else {
                        self.matrix.set_row(i, &RowVector4::from_iterator(new_line));
                    }
                }
            }
        }
        self
    }

    fn get_line(matrix: SMatrix<u32, 4, 4>, index: usize, dim: Dimension) -> Option<Vec<u32>> {
        if dim == Dimension::Col && index < matrix.ncols() {
            let col = matrix.column(index);
            Some(col.as_slice().to_vec())
        } else if dim == Dimension::Row && index < matrix.nrows() {
            let row = matrix.row(index);
            // row views are not contiguous, hence the clone_owned
            Some(row.clone_owned().as_slice().to_vec())
        } else {
            None
        }
    }

    fn shift_line(elements: &[u32]) -> (Vec<u32>, bool) {
        let (mut res, mutated) = Self::rec(elements, Vec::new(), false);
        if mutated {
            // todo next tile
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
    fn get_line_none_if_index_is_oob_row() -> () {
        let m = Matrix4::repeat(1);
        let res = Grid::get_line(m, 4, Dimension::Row);
        assert_eq!(res, None);
    }

    #[test]
    fn get_line_none_if_index_is_oob_col() -> () {
        let m = Matrix4::repeat(1);
        let res = Grid::get_line(m, 4, Dimension::Col);
        assert_eq!(res, None);
    }

    #[test]
    fn get_line_should_return_col_if_dim_is_col() -> () {
        let m = Matrix4::new(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0);
        let res = Grid::get_line(m, 0, Dimension::Col);
        assert_eq!(res, Some(vec![1, 1, 1, 1]));
    }

    #[test]
    fn get_line_should_return_row_if_dim_is_row() -> () {
        let m = Matrix4::new(0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0);
        let res = Grid::get_line(m, 1, Dimension::Row);
        assert_eq!(res, Some(vec![1, 1, 1, 1]));
    }

    #[test]
    fn shift_line_should_not_mutate_if_immutable() -> () {
        let array = [3, 6, 9, 12];
        let (res, mutated) = Grid::shift_line(&array);
        assert!(!mutated);
        assert_eq!(res, array.to_vec());
    }

    #[test]
    fn shift_line_should_mutate_if_adjacent_are_same() -> () {
        let array = [12, 12, 3, 6];
        let (res, mutated) = Grid::shift_line(&array);
        assert!(mutated);
        assert_eq!(res, [0, 24, 3, 6].to_vec());
    }

    #[test]
    fn shift_line_should_mutate_only_once() -> () {
        let array = [12, 12, 6, 6];
        let (res, mutated) = Grid::shift_line(&array);
        assert!(mutated);
        assert_eq!(res, [0, 24, 6, 6].to_vec());
    }

    #[test]
    fn shift_line_should_mutate_1_2() -> () {
        let array = [1, 2, 6, 6];
        let (res, mutated) = Grid::shift_line(&array);
        assert!(mutated);
        assert_eq!(res, [0, 3, 6, 6].to_vec());
    }

    #[test]
    fn shift_line_should_mutate_2_1() -> () {
        let array = [2, 1, 6, 6];
        let (res, mutated) = Grid::shift_line(&array);
        assert!(mutated);
        assert_eq!(res, [0, 3, 6, 6].to_vec());
    }
}