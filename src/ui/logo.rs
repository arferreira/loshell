use ratatui::{
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::theme::Theme;

pub fn logo() -> Paragraph<'static> {
    let line = Line::from(vec![
        Span::styled("▌", Theme::hot()),
        Span::styled("LOSHELL", Theme::hot()),
        Span::styled(" - ", Theme::base()),
        Span::styled("A room for your mind", Theme::frame()),
    ]);

    Paragraph::new(line).style(Theme::base())
}

pub const LOGO_WIDTH: u16 = 32;
pub const LOGO_HEIGHT: u16 = 1;
