use anyhow::Result;

mod distribution;
mod game;
mod series;
mod square;
mod state;
mod playfield;
mod direction;
mod buckets;
mod grid;

fn main() -> Result<()> {
    game::Game::run()
}
