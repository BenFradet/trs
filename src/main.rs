use anyhow::Result;

mod distribution;
mod game;
mod series;
mod square;
mod state;
mod playfield;

fn main() -> Result<()> {
    game::Game::run()
}
