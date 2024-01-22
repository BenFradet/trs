use nalgebra::{Matrix4, RowVector4, SMatrix, Vector4};
use rand::Rng;

use super::{buckets::Buckets, dimension::Dimension, direction::Direction};

pub struct Grid {
    pub matrix: SMatrix<u32, 4, 4>,
}

impl Grid {
    fn new(matrix: SMatrix<u32, 4, 4>) -> Grid {
        Grid { matrix }
    }

    pub fn rand<R: Rng + ?Sized>(r: &mut R, base_values: Box<[u32]>) -> Grid {
        let grid_size = 16;
        let buckets = Buckets::new(r, base_values, grid_size);
        let elements = buckets.draw(r);
        Grid {
            matrix: Matrix4::from_iterator(elements),
        }
    }

    // todo: handle next tile
    pub fn mov(&mut self, direction: Direction) -> &mut Grid {
        let reverse_needed = direction.reverse_needed();
        let dim = direction.associated_dimension();
        self.shift_grid(dim, reverse_needed)
    }

    fn shift_grid(&mut self, dim: Dimension, reverse_needed: bool) -> &mut Grid {
        let size = if dim == Dimension::Col {
            self.matrix.ncols()
        } else {
            self.matrix.nrows()
        };
        for i in 0..size {
            if let Some(mut elements) = Self::get_line(self.matrix, i, dim) {
                if reverse_needed {
                    elements.reverse()
                }
                let (mut new_line, mutated) = Self::shift_line(&elements);
                if mutated {
                    if reverse_needed {
                        new_line.reverse()
                    }
                    if dim == Dimension::Col {
                        self.matrix
                            .set_column(i, &Vector4::from_row_slice(&new_line));
                    } else {
                        self.matrix
                            .set_row(i, &RowVector4::from_row_slice(&new_line));
                    }
                }
            }
        }
        self
    }

    fn get_line(matrix: SMatrix<u32, 4, 4>, index: usize, dim: Dimension) -> Option<Box<[u32]>> {
        if dim == Dimension::Col && index < matrix.ncols() {
            let col = matrix.column(index);
            Some(col.as_slice().into())
        } else if dim == Dimension::Row && index < matrix.nrows() {
            let row = matrix.row(index);
            // row views are not contiguous, hence the clone_owned
            Some(row.clone_owned().as_slice().into())
        } else {
            None
        }
    }

    fn shift_line(elements: &[u32]) -> (Box<[u32]>, bool) {
        let (mut res, mutated) = Self::rec(elements, Vec::with_capacity(elements.len()), false);
        if mutated {
            // todo next tile
            res.push(0);
            //res.insert(0, 0);
            (res.into_boxed_slice(), mutated)
        } else {
            (res.into_boxed_slice(), mutated)
        }
    }

    // todo: too much game-specific logic, need to abstract things
    fn rec(elements: &[u32], mut acc: Vec<u32>, mutated: bool) -> (Vec<u32>, bool) {
        if !mutated {
            match elements {
                [h1, h2, t @ ..] => {
                    if h1 == h2 && h1 > &2 {
                        acc.push(h1 * 2);
                        Self::rec(t, acc, true)
                    } else if h1 + h2 == 3 && h1 < &3 && h2 < &3 {
                        acc.push(h1 + h2);
                        Self::rec(t, acc, true)
                    } else {
                        acc.push(*h1);
                        Self::rec(&elements[1..], acc, mutated)
                    }
                }
                [h, t @ ..] => {
                    acc.push(*h);
                    Self::rec(t, acc, mutated)
                }
                _ => (acc, mutated),
            }
        } else {
            // todo: find a way to short-circuit
            match elements {
                [h, t @ ..] => {
                    acc.push(*h);
                    Self::rec(t, acc, mutated)
                }
                _ => (acc, mutated),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_grid_does_one_transformation_reversed_per_col() -> () {
        let m = Matrix4::new(1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2);
        let mut g = Grid::new(m);
        let res = g.shift_grid(Dimension::Col, true);
        let expected = Matrix4::new(0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3);
        assert_eq!(res.matrix, expected);
    }

    #[test]
    fn shift_grid_does_one_transformation_reversed_per_row() -> () {
        let m = Matrix4::new(1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2);
        let mut g = Grid::new(m);
        let res = g.shift_grid(Dimension::Row, true);
        let expected = Matrix4::new(0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3);
        assert_eq!(res.matrix, expected);
    }

    #[test]
    fn shift_grid_does_no_more_than_one_transformation_per_col() -> () {
        let m = Matrix4::new(1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2);
        let mut g = Grid::new(m);
        let res = g.shift_grid(Dimension::Col, false);
        let expected = Matrix4::new(3, 3, 3, 3, 1, 1, 1, 1, 2, 2, 2, 2, 0, 0, 0, 0);
        assert_eq!(res.matrix, expected);
    }

    #[test]
    fn shift_grid_does_no_more_than_one_transformation_per_row() -> () {
        let m = Matrix4::new(1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2);
        let mut g = Grid::new(m);
        let res = g.shift_grid(Dimension::Row, false);
        let expected = Matrix4::new(3, 1, 2, 0, 3, 1, 2, 0, 3, 1, 2, 0, 3, 1, 2, 0);
        assert_eq!(res.matrix, expected);
    }

    #[test]
    fn shift_grid_does_not_mutate_if_immutable() -> () {
        let m = Matrix4::repeat(1);
        let mut g = Grid::new(m);
        let res = g.shift_grid(Dimension::Col, false);
        assert_eq!(res.matrix, m);
    }

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
        let expected: Box<[u32]> = Box::new([1, 1, 1, 1]);
        assert_eq!(res, Some(expected));
    }

    #[test]
    fn get_line_should_return_row_if_dim_is_row() -> () {
        let m = Matrix4::new(0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0);
        let res = Grid::get_line(m, 1, Dimension::Row);
        let expected: Box<[u32]> = Box::new([1, 1, 1, 1]);
        assert_eq!(res, Some(expected));
    }

    #[test]
    fn shift_line_should_not_mutate_if_immutable() -> () {
        let array = [3, 6, 9, 12];
        let (res, mutated) = Grid::shift_line(&array);
        assert!(!mutated);
        let expected: Box<[u32]> = Box::new(array);
        assert_eq!(res, expected);
    }

    #[test]
    fn shift_line_should_mutate_if_adjacent_are_same() -> () {
        let array = [12, 12, 3, 6];
        let (res, mutated) = Grid::shift_line(&array);
        assert!(mutated);
        let expected: Box<[u32]> = Box::new([24, 3, 6, 0]);
        assert_eq!(res, expected);
    }

    #[test]
    fn shift_line_should_mutate_only_once() -> () {
        let array = [12, 12, 6, 6];
        let (res, mutated) = Grid::shift_line(&array);
        assert!(mutated);
        let expected: Box<[u32]> = Box::new([24, 6, 6, 0]);
        assert_eq!(res, expected);
    }

    #[test]
    fn shift_line_should_mutate_1_2() -> () {
        let array = [1, 2, 6, 6];
        let (res, mutated) = Grid::shift_line(&array);
        assert!(mutated);
        let expected: Box<[u32]> = Box::new([3, 6, 6, 0]);
        assert_eq!(res, expected);
    }

    #[test]
    fn shift_line_should_mutate_2_1() -> () {
        let array = [2, 1, 6, 6];
        let (res, mutated) = Grid::shift_line(&array);
        assert!(mutated);
        let expected: Box<[u32]> = Box::new([3, 6, 6, 0]);
        assert_eq!(res, expected);
    }
}
