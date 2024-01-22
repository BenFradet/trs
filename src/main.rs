use anyhow::Result;

mod math;
mod game;
mod ui;
mod model;
mod state;

fn main() -> Result<()> {
    game::Game::run()
}
