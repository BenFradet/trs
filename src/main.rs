use anyhow::Result;

mod game;
mod math;
mod model;
mod state;
mod ui;

fn main() -> Result<()> {
    game::Game::run()
}
