use anyhow::Result;

mod distribution;
mod game;
mod series;
mod square;
mod state;
mod playfield;
mod direction;

fn main() -> Result<()> {
    game::Game::run()
}
