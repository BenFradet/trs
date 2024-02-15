use num::{Num, NumCast};
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub text: Color,
    pub background: Color,
    pub shadow: Color,
}

impl Theme {
    pub fn new<T: Num + NumCast>(elem: T) -> Theme {
        let (factor, color): (f64, u8) = if let Some(cast) = num::cast::<T, f64>(elem) {
            // this is not normalizing, everything above will be the same colour
            let max: f64 = 100.0;
            let scale: f64 = cast.min(max) / max;
            let factor: f64 = (1.0 - scale).max(0.3);
            let color: u8 = (255.0 * factor) as u8;
            (factor, color)
        } else {
            (1.0, 255)
        };
        Theme {
            text: Color::Black,
            background: Color::Rgb(color, color, color),
            shadow: Color::Rgb(
                (255.0 * factor) as u8,
                (204.0 * factor) as u8,
                (102.0 * factor) as u8,
            ),
        }
    }
}

pub const EMPTY_THEME: Theme = Theme {
    text: Color::Black,
    background: Color::Rgb(109, 130, 124),
    shadow: Color::Rgb(81, 119, 119),
};
pub const ONE_THEME: Theme = Theme {
    text: Color::Black,
    background: Color::Rgb(102, 204, 255),
    shadow: Color::Rgb(0, 67, 255),
};
pub const TWO_THEME: Theme = Theme {
    text: Color::Black,
    background: Color::Rgb(255, 102, 128),
    shadow: Color::Rgb(255, 0, 43),
};
pub const OTHER_THEME: Theme = Theme {
    text: Color::Black,
    background: Color::Rgb(255, 255, 255),
    shadow: Color::Rgb(255, 204, 102),
};

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::*;

    #[test]
    fn theme_is_modified_by_input_value() -> () {
        let theme = Theme::new::<u32>(12);
        assert_eq!(theme.background, Color::Rgb(224, 224, 224));
    }
}
