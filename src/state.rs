use rand::Rng;

use crate::{
    model::direction::Direction,
    model::grid::Grid, model::tile::Tile,
};

pub struct State {
    pub grid: Grid,
    pub tile: Tile,
}

impl State {
    pub fn new<R: Rng + ?Sized>(r: &mut R, base_values: Box<[u32]>) -> State {
        State {
            grid: Grid::rand(r, base_values),
            tile: Tile::new(r),
        }
    }

    pub fn shift<R: Rng + ?Sized>(&mut self, r: &mut R, direction: Direction) -> &mut State {
        let new_tile = self.tile.current();
        self.grid.mov(direction, new_tile);
        let max = self.grid.matrix.max();
        self.tile.next(r, max);
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