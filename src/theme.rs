use ratatui::style::{Color, Modifier, Style};

// Loshell default pallete (bladerunner vibes)
pub struct Theme;

impl Theme {
    // core pallete
    pub const BG: Color = Color::Rgb(0x0B, 0x0F, 0x1A); // midnight indigo
    pub const FG: Color = Color::Rgb(0xAA, 0xB4, 0xD6); // soft grey-blue
    pub const CYAN: Color = Color::Rgb(0x00, 0xF0, 0xFF); // neon cyan
    pub const PURPLE: Color = Color::Rgb(0x6A, 0x5A, 0xCD); // muted purple
    pub const PINK: Color = Color::Rgb(0xFF, 0x4F, 0xD8); // soft neon pink
    pub const DIM: Color = Color::Rgb(0x4D, 0x56, 0x70); // subdued border/text

    // Common styles
    pub fn base() -> Style {
        Style::default().fg(Self::FG).bg(Self::BG)
    }

    pub fn frame() -> Style {
        Style::default().fg(Self::DIM).bg(Self::BG)
    }

    pub fn title() -> Style {
        Style::default()
            .fg(Self::PURPLE)
            .bg(Self::BG)
            .add_modifier(Modifier::BOLD)
    }

    pub fn accent() -> Style {
        Style::default().fg(Self::CYAN).bg(Self::BG)
    }

    pub fn hot() -> Style {
        Style::default()
            .fg(Self::PINK)
            .bg(Self::BG)
            .add_modifier(Modifier::BOLD)
    }
}
