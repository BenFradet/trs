use crossterm::event::KeyCode;

pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  pub fn from_key_code(key_code: KeyCode) -> Option<Direction> {
    match key_code {
      KeyCode::Up | KeyCode::Char('w') => Some(Direction::Up),
      KeyCode::Down | KeyCode::Char('s') => Some(Direction::Down),
      KeyCode::Left | KeyCode::Char('a') => Some(Direction::Left),
      KeyCode::Right | KeyCode::Char('d') => Some(Direction::Right),
      _ => None
    }
  }
}