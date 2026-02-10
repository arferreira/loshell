use ratatui::{
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::theme::{Theme, ThemeName};

pub fn logo<'a>(theme: &Theme, theme_name: ThemeName) -> Paragraph<'a> {
    let line = Line::from(vec![
        Span::styled("▌", theme.hot()),
        Span::styled("LOSHELL", theme.hot()),
        Span::styled(" - ", theme.base()),
        Span::styled("A room for your mind", theme.frame()),
        Span::styled("  [", theme.frame()),
        Span::styled(theme_name.label(), theme.accent()),
        Span::styled("]", theme.frame()),
    ]);

    Paragraph::new(line).style(theme.base())
}

pub const LOGO_WIDTH: u16 = 55;
pub const LOGO_HEIGHT: u16 = 1;
