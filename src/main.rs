use anyhow::Result;

mod distribution;
mod game;
mod series;
mod square;
mod state;

fn main() -> Result<()> {
    game::Game::run()
}
