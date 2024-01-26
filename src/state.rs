use rand::Rng;

use crate::{model::direction::Direction, model::grid::Grid, model::tile::Tile};

pub struct State {
    pub grid: Grid,
    pub tile: Tile,
    past_grid: Grid,
    past_tile: Tile,
}

impl State {
    pub fn new<R: Rng + ?Sized>(r: &mut R, base_values: Box<[u32]>) -> State {
        let g = Grid::rand(r, base_values);
        let t = Tile::new(r);
        State {
            grid: g,
            tile: t,
            past_grid: g,
            past_tile: t,
        }
    }

    pub fn shift<R: Rng + ?Sized>(&mut self, r: &mut R, direction: Direction) -> &mut State {
        self.past_grid = self.grid;
        self.past_tile = self.tile;
        let new_tile = self.tile.current();
        let (_, next_tile_inserted, game_over) = self.grid.shift(r, direction, new_tile);
        if next_tile_inserted {
            let max = self.grid.matrix.max();
            self.tile.next(r, max);
        }
        self
    }

    pub fn shift_back(&mut self) -> &mut State {
        self.grid = self.past_grid;
        self.tile = self.past_tile;
        self
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;

    use super::*;

    #[test]
    fn shift_modifies_state() -> () {
        let mut r = OsRng;
        let mut s = State::new(&mut r, Box::new([4, 2, 2, 2]));
        println!("{}", s.grid.matrix);
        let res1 = s.shift(&mut r, Direction::Down);
        println!("{}", res1.grid.matrix);
    }
}
