use std::{
    io::{stdout, Stdout},
    ops::ControlFlow,
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
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
    Frame, Terminal,
};

use crate::{square::Square, state::State};

pub struct Game {
    title: &'static str,
    state: State,
}

impl Game {
    fn new() -> Game {
        Game {
            title: "Threes, use ← 	↑ 	→ 	↓ to play",
            state: State::new(Matrix4::new(0, 0, 1, 0, 0, 3, 3, 3, 1, 1, 0, 0, 0, 3, 2, 2)),
        }
    }

    pub fn run() -> Result<()> {
        let mut terminal = init_terminal()?;
        let mut game = Game::new();
        loop {
            let _ = terminal.draw(|frame| game.ui(frame));
            if !event::poll(Duration::from_millis(100))? {
                continue;
            }
            match event::read()? {
                Event::Key(key) => {
                    if key.kind != event::KeyEventKind::Press {
                        continue;
                    }
                    if game.handle_key_event(key).is_break() {
                        break;
                    }
                }
                _ => (),
            }
        }
        restore_terminal()
    }

    fn ui(&self, frame: &mut Frame) -> () {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),
                Constraint::Length(40),
                Constraint::Min(0),
            ])
            .split(frame.size());
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(self.title.dark_gray()).alignment(Alignment::Left)
            ]),
            main_layout[0],
        );

        let game_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),
                Constraint::Length(7),
                Constraint::Length(7),
                Constraint::Length(7),
                Constraint::Min(0),
            ])
            .split(main_layout[1]);
        let game_areas = game_rows
            .iter()
            .flat_map(|row| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(14),
                        Constraint::Length(14),
                        Constraint::Length(14),
                        Constraint::Length(14),
                        Constraint::Min(0),
                    ])
                    .split(*row)
                    .iter()
                    .copied()
                    .take(4) // ignore min 0
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for i in 0..=3 {
            for j in 0..=3 {
                let elem = self.state.matrix[(i, j)];
                frame.render_widget(Square::from_elem(elem), game_areas[i * 4 + j])
            }
        }
    }

    fn handle_key_event(&mut self, key: event::KeyEvent) -> ControlFlow<()> {
        match key.code {
            KeyCode::Char('q') => return ControlFlow::Break(()),
            KeyCode::Left => {
                self.state.shift_left();
            }
            KeyCode::Right => {
                self.state.shift_right();
            }
            KeyCode::Up => {
                self.state.shift_up();
            }
            KeyCode::Down => {
                self.state.shift_down();
            }
            _ => (),
        }
        ControlFlow::Continue(())
    }
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
