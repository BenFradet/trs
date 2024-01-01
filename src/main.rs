use anyhow::Result;

mod game;
mod series;
mod square;
mod state;

fn main() -> Result<()> {
    game::Game::run()
}
