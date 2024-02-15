use std::fmt::Display;

use num::{Num, NumCast};
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Widget},
};

use super::theme::{Theme, EMPTY_THEME, ONE_THEME, TWO_THEME};

#[derive(Debug, Clone)]
pub struct Square<'a> {
    label: Line<'a>,
    theme: Theme,
    block: Option<Block<'a>>,
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

    pub fn from_elem<T: Num + NumCast + Display + Copy>(elem: T) -> Square<'a> {
        match elem {
            zero if zero == T::zero() => Square::new("").theme(EMPTY_THEME),
            one if one == T::one() => Square::new("1").theme(ONE_THEME),
            two if two == T::one() + T::one() => Square::new("2").theme(TWO_THEME),
            other => Square::new(elem.to_string()).theme(Theme::new(other)),
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
