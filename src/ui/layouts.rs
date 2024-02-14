use ratatui::layout::{Constraint, Direction, Layout};
use once_cell::sync::Lazy;

pub const TILE_HEIGHT: u16 = 7;
pub const TILE_WIDTH: u16 = 14;
pub const TILE_NUMBER: u16 = 4;

// having to use lazy static because Layout::new is not const and Layout's fields are private
pub static MAIN_LAYOUT: Lazy<Layout> = Lazy::new(||Layout::new(
    Direction::Vertical,
    [
        Constraint::Length(2),
        Constraint::Length(TILE_HEIGHT),
        Constraint::Length(40),
        Constraint::Min(0),
    ],
));

pub static HORIZONTAL_SEP: Lazy<Layout> = Lazy::new(||Layout::new(
    Direction::Horizontal,
    [
        Constraint::Length(TILE_WIDTH),
        Constraint::Length(TILE_WIDTH),
        Constraint::Length(TILE_WIDTH),
        Constraint::Length(TILE_WIDTH),
        Constraint::Min(0),
    ],
));

pub static GAME_LAYOUT_H: Lazy<Layout> = Lazy::new(||Layout::new(
    Direction::Horizontal,
    [
        Constraint::Max(TILE_WIDTH * TILE_NUMBER),
        Constraint::Min(0),
    ],
));

pub static GAME_LAYOUT_V: Lazy<Layout> = Lazy::new(||Layout::new(
    Direction::Vertical,
    [
        Constraint::Max(TILE_HEIGHT * TILE_NUMBER),
        Constraint::Min(0),
    ],
));

pub static ROW_LAYOUT: Lazy<Layout> = Lazy::new(||Layout::new(
    Direction::Vertical,
    [
        Constraint::Length(TILE_HEIGHT),
        Constraint::Length(TILE_HEIGHT),
        Constraint::Length(TILE_HEIGHT),
        Constraint::Length(TILE_HEIGHT),
        Constraint::Min(0),
    ],
));

pub fn popup_layout(percent: u16, dir: Direction) -> Layout {
    Layout::new(
        dir,
        [
            Constraint::Percentage((100 - percent) / 2),
            Constraint::Percentage(percent),
            Constraint::Percentage((100 - percent) / 2),
        ],
    )
}
