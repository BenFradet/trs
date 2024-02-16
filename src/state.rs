use rand::Rng;

use crate::{
    math::series::Series, model::direction::Direction, model::grid::Grid, model::tile::Tile,
};

pub struct State {
    pub grid: Grid,
    pub tile: Tile,
    pub game_over: bool,
    past_grid: Grid,
    past_tile: Tile,
    series: Series,
}

impl State {
    pub fn from_base_values<R: Rng + ?Sized, I>(r: &mut R, base_values: I) -> State
    where
        I: IntoIterator<Item = u32>,
    {
        let g = Grid::rand(r, base_values);
        let t = Tile::new(r);
        State {
            grid: g,
            tile: t,
            game_over: false,
            past_grid: g,
            past_tile: t,
            series: Series::new(1, 2, 2),
        }
    }

    pub fn shift<R: Rng + ?Sized>(&mut self, r: &mut R, direction: Direction) -> &mut State {
        self.past_grid = self.grid;
        self.past_tile = self.tile;
        let new_tile = self.tile.current();
        let (new_grid, next_tile_inserted, game_over) = self.grid.shift(r, direction, new_tile);
        self.grid = new_grid;
        self.game_over = game_over;
        if next_tile_inserted {
            let max = self.grid.matrix.max();
            self.tile = self.tile.next(r, max);
        }
        self
    }

    pub fn shift_back(&mut self) -> &mut State {
        self.grid = self.past_grid;
        self.tile = self.past_tile;
        self
    }

    pub fn score(&self) -> u64 {
        self.grid.matrix.fold(0, |acc, e| {
            if e < 3 {
                acc
            } else {
                let rank = self.series.n(e);
                acc + (3_u64).pow(rank - 1)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{Matrix4, SMatrix};
    use rand::rngs::OsRng;

    use super::*;

    fn new_grid(matrix: SMatrix<u32, 4, 4>) -> Grid {
        Grid { matrix }
    }

    fn new_state<R: Rng + ?Sized>(r: &mut R, m: SMatrix<u32, 4, 4>) -> State {
        let g = new_grid(m);
        let t = Tile::new(r);
        State {
            grid: g,
            tile: t,
            game_over: false,
            past_grid: g,
            past_tile: t,
            series: Series::new(1, 2, 2),
        }
    }

    #[test]
    fn score_is_0_if_no_gt_3() -> () {
        let mut r = OsRng;
        for i in 0..3 {
            let s = new_state(&mut r, Matrix4::repeat(i));
            assert_eq!(s.score(), 0);
        }
    }

    #[test]
    fn score_is_correct_if_gt_3() -> () {
        let mut r = OsRng;
        let series = Series::new(1, 2, 2);
        let grid_size = 16;
        for i in 2..10 {
            let u_i = series.u_n(i);
            let m = Matrix4::repeat(u_i);
            let s = new_state(&mut r, m);
            assert_eq!(s.score(), 3_u64.pow(i - 1) as u64 * grid_size);
        }
    }

    #[test]
    fn score_is_in_concordance_with_screenshot() -> () {
        let mut r = OsRng;
        let m = Matrix4::new(2, 3, 1, 3, 3, 1, 3, 2, 6, 3, 24, 2, 3, 48, 192, 96);
        let s = new_state(&mut r, m);
        assert_eq!(s.score(), 3267);
    }
}
