use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use nalgebra::{Matrix4, SMatrix};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Widget,
    },
    Frame, Terminal,
};

pub struct Game {
    title: &'static str,
    state: SMatrix<u32, 4, 4>,
    marker: Marker,
}

impl Game {
    fn new() -> Game {
        Game {
            title: "Threes",
            state: Matrix4::new(
                0, 0, 1, 0,
                0, 3, 3, 3,
                1, 1, 0, 0,
                0, 3, 2, 2, 
            ),
            marker: Marker::HalfBlock,
        }
    }

    pub fn run() -> Result<()> {
        let mut terminal = init_terminal()?;
        let game = Game::new();
        loop {
            let _ = terminal.draw(|frame| game.ui(frame));
            if should_quit()? {
                break;
            }
        }
        restore_terminal()
    }

    fn ui(&self, frame: &mut Frame) -> () {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0)])
            .split(frame.size());
        frame.render_widget(self.boxes_canvas(main_layout[0]), main_layout[0]);
    }

    fn boxes_canvas(&self, area: Rect) -> impl Widget + '_ {
        let (left, right, bottom, top) =
            (0.0, area.width as f64, 0.0, area.height as f64 * 2.0 - 4.0);
        Canvas::default()
            .block(Block::default().borders(Borders::ALL).title(self.title))
            .marker(self.marker)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(|ctx| {
                for i in 0..=3 {
                    for j in 0..=3 {
                        let elem = self.state[(i, j)];
                        ctx.draw(&Rectangle {
                            x: 2.0 + i as f64 * 14.0,
                            y: 2.0 + j as f64 * 14.0,
                            width: 10.0,
                            height: 10.0,
                            color: color(elem),
                        });
                    }
                }
            })
    }
}

fn color(elem: u32) -> Color {
    if elem == 0 {
        return Color::DarkGray;
    } else if elem == 1 {
        return Color::Blue;
    } else if elem == 2 {
        return Color::Red;
    }
    Color::Indexed(56)
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode().context("failed to enable raw mode")?;
    stdout()
        .execute(EnterAlternateScreen)
        .context("failed to enter alternate mode")?;
    Terminal::new(CrosstermBackend::new(stdout())).context("terminal creation failed")
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    stdout()
        .execute(LeaveAlternateScreen)
        .context("failed to leave alternate screen")?;
    Ok(())
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(200)).context("failed to poll")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
