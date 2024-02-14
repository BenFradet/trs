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
    layout::{Alignment, Direction, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};

use crate::{state::State, ui::{layouts::{popup_layout, GAME_LAYOUT_H, GAME_LAYOUT_V, HORIZONTAL_SEP, MAIN_LAYOUT, ROW_LAYOUT}, square::{Square, OTHER_THEME}}};

pub struct Game {
    title: &'static str,
    instruction: &'static str,
    state: State,
}

impl Game {
    fn new(r: &mut OsRng) -> Game {
        // todo: initial matrix should be gen'd
        Game {
            title: "Threes",
            instruction: "use ← 	↑ 	→ 	↓ to play, q to quit, u to undo",
            state: State::from_base_values(r, Box::new([4, 2, 2, 2])),
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
        let score = self.state.score();

        let main_layout = MAIN_LAYOUT.split(frame.size());
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(self.title.dark_gray()).alignment(Alignment::Left)
            ]),
            main_layout[0],
        );

        // next tile
        let next_tile_block = Block::new()
            .borders(Borders::ALL)
            .title("next tile".dark_gray());
        let next_tile_widget = Square::from_elem(self.state.tile.current()).block(next_tile_block);
        frame.render_widget(next_tile_widget, HORIZONTAL_SEP.split(main_layout[1])[0]);

        // score
        let score_block = Block::new()
            .borders(Borders::ALL)
            .title("score".dark_gray());
        let next_tile_widget = Square::from_elem(score).theme(OTHER_THEME).block(score_block);
        frame.render_widget(next_tile_widget, HORIZONTAL_SEP.split(main_layout[1])[1]);

        // game
        let game_block = Block::new()
            .borders(Borders::ALL)
            .title(self.instruction.dark_gray());
        let game_area = GAME_LAYOUT_H.split(GAME_LAYOUT_V.split(main_layout[2])[0])[0];
        frame.render_widget(game_block, game_area);

        // game
        let row_layout = ROW_LAYOUT.split(main_layout[2]);
        let game_areas = row_layout
            .iter()
            .flat_map(|row| {
                HORIZONTAL_SEP
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

        if self.state.game_over {
            let block = Block::default().title("game over").borders(Borders::ALL);
            let area = centered_rect(40, 20, frame.size());
            frame.render_widget(Clear, area); //this clears out the background
            let text = format!("your score is {}, q to quit, r to restart", score);
            let paragraph = Paragraph::new(text.dark_gray());
            frame.render_widget(paragraph.block(block), area);
        }
    }

    fn handle_key_event<R: Rng + ?Sized>(
        &mut self,
        r: &mut R,
        key: event::KeyEvent,
    ) -> ControlFlow<()> {
        if let Some(dir) = crate::model::direction::Direction::from_key_code(key.code) {
            self.state.shift(r, dir);
        } else if key.code == KeyCode::Char('u') {
            self.state.shift_back();
        } else if self.state.game_over && key.code == KeyCode::Char('r') {
            self.state = State::from_base_values(r, Box::new([4, 2, 2, 2]));
        } else if key.code == KeyCode::Char('q') {
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout_v = popup_layout(percent_y, Direction::Vertical)
        .split(r);
    popup_layout(percent_x, Direction::Horizontal)
        .split(popup_layout_v[1])[1]
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
