use anyhow::Result;

mod game;
mod square;
mod state;

fn main() -> Result<()> {
    game::Game::run()
}
