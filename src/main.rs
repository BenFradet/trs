use anyhow::Result;

mod math;
mod game;
mod square;
mod state;
mod playfield;

fn main() -> Result<()> {
    game::Game::run()
}
