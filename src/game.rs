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
use rand::{rngs::OsRng, Rng};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{Paragraph, Block, Borders},
    Frame, Terminal,
};

use crate::{state::State, ui::square::Square};

pub struct Game {
    title: &'static str,
    instruction: &'static str,
    state: State,
    tile_width: u16,
    tile_height: u16,
    tile_number: u16,
}

impl Game {
    fn new(r: &mut OsRng) -> Game {
        // todo: initial matrix should be gen'd
        Game {
            title: "Threes",
            instruction: "use ← 	↑ 	→ 	↓ to play",
            state: State::new(
                r,
                Box::new([4, 2, 2, 2]),
            ),
            tile_width: 14,
            tile_height: 7,
            tile_number: 4,
        }
    }

    pub fn run() -> Result<()> {
        let mut terminal = init_terminal()?;
        let mut r = OsRng;
        let mut game = Game::new(&mut r);
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
                    if game.handle_key_event(&mut r, key).is_break() {
                        break;
                    }
                }
                _ => (),
            }
        }
        restore_terminal()
    }

    fn ui(&mut self, frame: &mut Frame) -> () {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(7),
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

        let horizontal_sep = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(self.tile_width),
                Constraint::Length(self.tile_width),
                Constraint::Length(self.tile_width),
                Constraint::Length(self.tile_width),
                Constraint::Min(0),
            ]);

        // next tile
        let next_tile_block = Block::new()
            .borders(Borders::ALL)
            .title("next tile".dark_gray());
        let next_tile_widget = Square::from_elem(self.state.next_tile)
            .block(next_tile_block);
        frame.render_widget(next_tile_widget, horizontal_sep.split(main_layout[1])[0]);

        // game block
        let game_block = Block::new()
            .borders(Borders::ALL)
            .title(self.instruction.dark_gray());
        let game_layout_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Max(self.tile_width * self.tile_number),
                Constraint::Min(0),
            ]);
        let game_layout_v = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Max(self.tile_height * self.tile_number),
                Constraint::Min(0),
            ]);
        let game_area = game_layout_h.split(game_layout_v.split(main_layout[2])[0])[0];
        frame.render_widget(game_block, game_area);

        // game
        let game_rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(self.tile_height),
                Constraint::Length(self.tile_height),
                Constraint::Length(self.tile_height),
                Constraint::Length(self.tile_height),
                Constraint::Min(0),
            ])
            .split(main_layout[2]);
        let game_areas = game_rows
            .iter()
            .flat_map(|row| {
                horizontal_sep
                    .split(*row)
                    .iter()
                    .copied()
                    .take(4) // ignore min 0
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for i in 0..=3 {
            for j in 0..=3 {
                let elem = self.state.grid.matrix[(i, j)];
                frame.render_widget(Square::from_elem(elem), game_areas[i * 4 + j])
            }
        }
    }

    fn handle_key_event<R: Rng + ?Sized>(&mut self, r: &mut R, key: event::KeyEvent) -> ControlFlow<()> {
        if let Some(dir) = crate::model::direction::Direction::from_key_code(key.code) {
            self.state.shift(r, dir);
        } else if key.code == KeyCode::Char('q') {
            return ControlFlow::Break(())
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
