use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::Color,
    style::Style,
    text::Line,
    widgets::{Block, Widget},
};

#[derive(Debug, Clone)]
pub struct Square<'a> {
    label: Line<'a>,
    theme: Theme,
    block: Option<Block<'a>>,
}

#[derive(Debug, Clone)]
pub struct Theme {
    text: Color,
    background: Color,
    shadow: Color,
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
pub fn other_theme(elem: u32) -> Theme {
    // this is not normalizing, everything above will be the same colour
    let max: f32 = 100.0;
    let scale: f32 = (elem as f32).min(max) / max;
    let factor: f32 = (1.0 - scale).max(0.3);
    let color: u8 = (255.0 * factor) as u8;
    Theme {
        text: Color::Black,
        background: Color::Rgb(color, color, color),
        shadow: Color::Rgb((255.0 * factor) as u8, (204.0 * factor) as u8, (102.0 * factor) as u8),
    }
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::other_theme;

    #[test]
    fn theme_is_modified_by_input_value() -> () {
        let theme = other_theme(12);
        assert_eq!(theme.background, Color::Rgb(224, 224, 224));
    }
}

impl<'a> Square<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Square<'a> {
        Square {
            label: label.into(),
            theme: EMPTY_THEME,
            block: None,
        }
    }

    pub fn theme(mut self, theme: Theme) -> Square<'a> {
        self.theme = theme;
        self
    }

    pub fn from_elem(elem: u32) -> Square<'a> {
        match elem {
            0 => Square::new("").theme(EMPTY_THEME),
            1 => Square::new("1").theme(ONE_THEME),
            2 => Square::new("2").theme(TWO_THEME),
            other => Square::new(elem.to_string()).theme(other_theme(other)),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Square<'a> {
        self.block = Some(block);
        self
    }

    fn render_block(&mut self, area: &mut Rect, buf: &mut Buffer) -> () {
        if let Some(block) = self.block.take() {
            let inner_area = block.inner(*area);
            block.render(*area, buf);
            *area = inner_area
        }
    }
}

impl<'a> Widget for Square<'a> {
    fn render(
        mut self,
        mut area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) -> () {
        let Theme {
            background,
            text,
            shadow,
        } = self.theme;
        let inner_rect = area.inner(&Margin::new(2, 1));
        buf.set_style(inner_rect, Style::new().bg(background).fg(text));

        self.render_block(&mut area, buf);

        if inner_rect.height > 1 {
            buf.set_string(
                inner_rect.x,
                inner_rect.y + inner_rect.height - 1,
                "▁".repeat(inner_rect.width as usize),
                Style::new().bg(background).fg(shadow),
            );
        }

        buf.set_line(
            inner_rect.x + (inner_rect.width.saturating_sub(self.label.width() as u16)) / 2,
            inner_rect.y + (inner_rect.height.saturating_sub(1)) / 2,
            &self.label,
            inner_rect.width,
        );
    }
}
