use anyhow::Result;

mod game;

fn main() -> Result<()> {
    game::Game::run()
}
