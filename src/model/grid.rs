use nalgebra::{Matrix4, RowVector4, SMatrix, Vector4};
use rand::{distributions::Uniform, Rng};

use crate::utils::matrix_any::MatrixAny;

use super::{buckets::Buckets, dimension::Dimension, direction::Direction};

#[derive(Clone, Copy)]
pub struct Grid {
    pub matrix: SMatrix<u32, 4, 4>,
}

impl Grid {

    pub fn rand<R: Rng + ?Sized, I>(r: &mut R, base_values: I) -> Grid
    where
        I: IntoIterator<Item = u32>,
    {
        let grid_size = 16;
        let buckets = Buckets::new(r, base_values, grid_size);
        let elements = buckets.draw(r);
        let m = Matrix4::from_iterator(elements);
        Grid { matrix: m }
    }

    pub fn shift<R: Rng + ?Sized>(
        mut self,
        r: &mut R,
        dir: Direction,
        next_tile: u32,
    ) -> (Grid, bool, bool) {
        let reverse_needed = dir.reverse_needed();
        let dim = dir.associated_dimension();

        let mut next_tile_inserted = false;
        let mut mutated = false;

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
                let (mut new_line, muta, combined) =
                    Self::shift_line(&elements, next_tile, next_tile_inserted);
                if muta {
                    mutated = true;
                    if !next_tile_inserted && combined {
                        next_tile_inserted = true;
                    }
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

        if !next_tile_inserted && mutated {
            let idx = dir.index();
            let inverse_dim = dim.inverse();
            if let Some(line_with_next_tile) =
                Self::force_insert_next_tile(r, self.matrix, idx, inverse_dim, next_tile)
            {
                if inverse_dim == Dimension::Col {
                    self.matrix
                        .set_column(idx, &Vector4::from_row_slice(&line_with_next_tile));
                } else {
                    self.matrix
                        .set_row(idx, &RowVector4::from_row_slice(&line_with_next_tile));
                }
                next_tile_inserted = true;
            }
        }
        if next_tile_inserted {
            (self, next_tile_inserted, false)
        } else {
            let game_over = self.game_over();
            (self, next_tile_inserted, game_over)
        }
    }

    fn game_over(self) -> bool {
        // mutable => contains 0
        let mutable = self.matrix.iter().any(|e| *e == 0);
        // combinable
        !mutable && !self.matrix.any_col(Self::combinable) && !self.matrix.any_row(Self::combinable)
    }

    // if there was no combination, replace a 0 with next tile
    fn force_insert_next_tile<R: Rng + ?Sized>(
        r: &mut R,
        matrix: SMatrix<u32, 4, 4>,
        index: usize,
        dim: Dimension,
        next_tile: u32,
    ) -> Option<Vec<u32>> {
        match Self::get_line(matrix, index, dim) {
            Some(slice) => {
                let mut zeros = Vec::with_capacity(slice.len());
                for i in 0..slice.len() {
                    if slice[i] == 0 {
                        zeros.push(i);
                    }
                }
                if zeros.len() == 0 {
                    None
                } else {
                    let i = r.sample(Uniform::new(0, zeros.len()));
                    let idx = zeros[i];
                    let mut values = slice.to_vec();
                    let _ = std::mem::replace(&mut values[idx], next_tile);
                    Some(values.as_slice().into())
                }
            }
            None => None,
        }
    }

    fn get_line(matrix: SMatrix<u32, 4, 4>, index: usize, dim: Dimension) -> Option<Vec<u32>> {
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

    fn shift_line(
        elements: &[u32],
        next_tile: u32,
        next_tile_inserted: bool,
    ) -> (Vec<u32>, bool, bool) {
        fn inner(
            elements: &[u32],
            mut acc: Vec<u32>,
            mutated: bool,
            combined: bool,
            combiner: &dyn Fn(u32, u32) -> Option<u32>,
        ) -> (Vec<u32>, bool, bool) {
            if !combined {
                match elements {
                    [h1, h2, t @ ..] => {
                        if let Some(value) = combiner(*h1, *h2) {
                            acc.push(value);
                            inner(t, acc, true, true, combiner)
                        } else if h1 == &0 {
                            acc.push(*h2);
                            // todo: find a way to avoid the vec allocation
                            let mut es: Vec<u32> = t.to_vec();
                            es.insert(0, 0);
                            inner(es.as_slice(), acc, true, combined, combiner)
                        } else {
                            acc.push(*h1);
                            inner(&elements[1..], acc, mutated, combined, combiner)
                        }
                    }
                    [h, t @ ..] => {
                        acc.push(*h);
                        inner(t, acc, mutated, combined, combiner)
                    }
                    _ => (acc, mutated, combined),
                }
            } else {
                // todo: find a way to short-circuit
                match elements {
                    [h, t @ ..] => {
                        acc.push(*h);
                        inner(t, acc, mutated, combined, combiner)
                    }
                    _ => (acc, mutated, combined),
                }
            }
        }

        let (mut res, mutated, combined) = inner(
            elements,
            Vec::with_capacity(elements.len()),
            false,
            false,
            &Self::combiner,
        );
        if combined {
            if next_tile_inserted {
                res.push(0);
            } else {
                res.push(next_tile);
            }
            (res, mutated, combined)
        } else {
            (res, mutated, combined)
        }
    }

    fn combiner(h1: u32, h2: u32) -> Option<u32> {
        if h1 == h2 && h1 > 2 {
            Some(h1 * 2)
        } else if h1 + h2 == 3 && h1 < 3 && h2 < 3 {
            Some(h1 + h2)
        } else {
            None
        }
    }

    fn combinable(elements: &[u32]) -> bool {
        fn inner(es: &[u32], acc: bool, f: &dyn Fn(u32, u32) -> bool) -> bool {
            if acc {
                acc
            } else {
                match es {
                    [h1, h2, _t @ ..] => {
                        if f(*h1, *h2) {
                            true
                        } else {
                            inner(&es[1..], acc, f)
                        }
                    }
                    _ => false,
                }
            }
        }
        inner(elements, false, &|h1, h2| Self::combiner(h1, h2).is_some())
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;

    use super::*;

    fn new_grid(matrix: SMatrix<u32, 4, 4>) -> Grid {
        Grid { matrix }
    }

    #[test]
    fn combinable_true_if_identical_ge_3() -> () {
        let slice = &[1, 3, 3, 1];
        let res = Grid::combinable(slice);
        assert!(res);
    }

    #[test]
    fn combinable_true_if_1_2() -> () {
        let slice = &[1, 2, 3, 1];
        let res = Grid::combinable(slice);
        assert!(res);
    }

    #[test]
    fn game_over_is_false_if_there_is_a_0() -> () {
        let mut m = Matrix4::repeat(1);
        m[(1, 3)] = 0;
        let g = new_grid(m);
        assert!(!g.game_over());
    }

    #[test]
    fn game_over_is_false_if_there_is_a_col_combination_possible() -> () {
        let mut m = Matrix4::repeat(1);
        m[(1, 0)] = 3;
        m[(2, 0)] = 3;
        let g = new_grid(m);
        assert!(!g.game_over());
    }

    #[test]
    fn game_over_is_false_if_there_is_a_row_combination_possible() -> () {
        let mut m = Matrix4::repeat(1);
        m[(1, 0)] = 3;
        m[(1, 1)] = 3;
        let g = new_grid(m);
        assert!(!g.game_over());
    }

    #[test]
    fn force_insert_next_tile_some_if_zero_found_up() -> () {
        let mut r = OsRng;
        let mut m = Matrix4::repeat(1);
        m[(3, 3)] = 0;
        let res = Grid::force_insert_next_tile(&mut r, m, 3, Dimension::Col, 12);
        let exp = vec![1, 1, 1, 12];
        assert_eq!(res, Some(exp));
    }

    #[test]
    fn force_insert_next_tile_some_if_zero_found_down() -> () {
        let mut r = OsRng;
        let mut m = Matrix4::repeat(1);
        m[(3, 0)] = 0;
        let res = Grid::force_insert_next_tile(&mut r, m, 0, Dimension::Col, 12);
        let exp = vec![1, 1, 1, 12];
        assert_eq!(res, Some(exp));
    }

    #[test]
    fn force_insert_next_tile_some_if_zero_found_left() -> () {
        let mut r = OsRng;
        let mut m = Matrix4::repeat(1);
        m[(0, 3)] = 0;
        let res = Grid::force_insert_next_tile(&mut r, m, 0, Dimension::Row, 12);
        let exp = vec![1, 1, 1, 12];
        assert_eq!(res, Some(exp));
    }

    #[test]
    fn force_insert_next_tile_some_if_zero_found_right() -> () {
        let mut r = OsRng;
        let mut m = Matrix4::repeat(1);
        m[(3, 3)] = 0;
        let res = Grid::force_insert_next_tile(&mut r, m, 3, Dimension::Row, 12);
        let exp = vec![1, 1, 1, 12];
        assert_eq!(res, Some(exp));
    }

    #[test]
    fn force_insert_next_tile_none_if_idx_out_of_bounds() -> () {
        let mut r = OsRng;
        let m = Matrix4::repeat(1);
        let res = Grid::force_insert_next_tile(&mut r, m, 4, Dimension::Col, 12);
        assert_eq!(res, None);
    }

    #[test]
    fn force_insert_next_tile_none_if_no_zeros() -> () {
        let mut r = OsRng;
        let m = Matrix4::repeat(1);
        let res_up = Grid::force_insert_next_tile(&mut r, m, 3, Dimension::Col, 12);
        assert_eq!(res_up, None);
        let res_down = Grid::force_insert_next_tile(&mut r, m, 0, Dimension::Col, 12);
        assert_eq!(res_down, None);
        let res_left = Grid::force_insert_next_tile(&mut r, m, 3, Dimension::Row, 12);
        assert_eq!(res_left, None);
        let res_right = Grid::force_insert_next_tile(&mut r, m, 0, Dimension::Row, 12);
        assert_eq!(res_right, None);
    }

    #[test]
    fn shift_grid_does_one_transformation_reversed_per_col() -> () {
        let mut r = OsRng;
        let m = Matrix4::new(1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2);
        let g = new_grid(m);
        let (res, _, _) = g.shift(&mut r, Direction::Down, 12);
        let expected = Matrix4::new(12, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3);
        assert_eq!(res.matrix, expected);
    }

    #[test]
    fn shift_grid_does_one_transformation_reversed_per_row() -> () {
        let mut r = OsRng;
        let m = Matrix4::new(1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2);
        let g = new_grid(m);
        let (res, _, _) = g.shift(&mut r, Direction::Right, 12);
        let expected = Matrix4::new(12, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3);
        assert_eq!(res.matrix, expected);
    }

    #[test]
    fn shift_grid_does_no_more_than_one_transformation_per_col() -> () {
        let mut r = OsRng;
        let m = Matrix4::new(1, 1, 1, 1, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2);
        let g = new_grid(m);
        let (res, _, _) = g.shift(&mut r, Direction::Up, 12);
        let expected = Matrix4::new(3, 3, 3, 3, 1, 1, 1, 1, 2, 2, 2, 2, 12, 0, 0, 0);
        assert_eq!(res.matrix, expected);
    }

    #[test]
    fn shift_grid_does_no_more_than_one_transformation_per_row() -> () {
        let mut r = OsRng;
        let m = Matrix4::new(1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2);
        let g = new_grid(m);
        let (res, _, _) = g.shift(&mut r, Direction::Left, 12);
        let expected = Matrix4::new(3, 1, 2, 12, 3, 1, 2, 0, 3, 1, 2, 0, 3, 1, 2, 0);
        assert_eq!(res.matrix, expected);
    }

    #[test]
    fn shift_grid_does_not_mutate_if_immutable() -> () {
        let mut r = OsRng;
        let m = Matrix4::repeat(1);
        let g = new_grid(m);
        let (res, _, _) = g.shift(&mut r, Direction::Up, 12);
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
        let expected = vec![1, 1, 1, 1];
        assert_eq!(res, Some(expected));
    }

    #[test]
    fn get_line_should_return_row_if_dim_is_row() -> () {
        let m = Matrix4::new(0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0);
        let res = Grid::get_line(m, 1, Dimension::Row);
        let expected = vec![1, 1, 1, 1];
        assert_eq!(res, Some(expected));
    }

    #[test]
    fn shift_line_mutate_zeros() -> () {
        let array = [1, 0, 2, 2];
        let (res, mutated, combined) = Grid::shift_line(&array, 12, true);
        assert!(mutated);
        assert!(!combined);
        let expected = vec![1, 2, 2, 0];
        assert_eq!(res, expected);
    }

    #[test]
    fn shift_line_should_not_mutate_if_immutable() -> () {
        let array = [3, 6, 9, 12];
        let (res, mutated, combined) = Grid::shift_line(&array, 12, true);
        assert!(!mutated);
        assert!(!combined);
        let expected: Vec<u32> = array.into();
        assert_eq!(res, expected);
    }

    #[test]
    fn shift_line_should_combine_if_adjacent_are_same() -> () {
        let array = [12, 12, 3, 6];
        let (res, mutated, combined) = Grid::shift_line(&array, 12, true);
        assert!(mutated);
        assert!(combined);
        let expected = vec![24, 3, 6, 0];
        assert_eq!(res, expected);
    }

    #[test]
    fn shift_line_should_combine_only_once() -> () {
        let array = [12, 12, 6, 6];
        let (res, mutated, combined) = Grid::shift_line(&array, 12, true);
        assert!(mutated);
        assert!(combined);
        let expected = vec![24, 6, 6, 0];
        assert_eq!(res, expected);
    }

    #[test]
    fn shift_line_should_combine_1_2() -> () {
        let array = [1, 2, 6, 6];
        let (res, mutated, combined) = Grid::shift_line(&array, 12, true);
        assert!(mutated);
        assert!(combined);
        let expected = vec![3, 6, 6, 0];
        assert_eq!(res, expected);
    }

    #[test]
    fn shift_line_should_combine_2_1() -> () {
        let array = [2, 1, 6, 6];
        let (res, mutated, combined) = Grid::shift_line(&array, 12, true);
        assert!(mutated);
        assert!(combined);
        let expected = vec![3, 6, 6, 0];
        assert_eq!(res, expected);
    }
}
