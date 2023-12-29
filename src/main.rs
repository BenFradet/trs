use anyhow::Result;

mod game;
mod square;

fn main() -> Result<()> {
    game::Game::run()
}
