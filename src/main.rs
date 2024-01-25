use anyhow::Result;

mod game;
mod math;
mod model;
mod state;
mod ui;
mod utils;

fn main() -> Result<()> {
    game::Game::run()
}
