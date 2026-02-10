use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeName {
    Bladerunner,
    CatppuccinMocha,
    Gruvbox,
    TokyoNight,
    RosePine,
    Monotropic,
}

impl ThemeName {
    pub const ALL: [ThemeName; 6] = [
        ThemeName::Bladerunner,
        ThemeName::CatppuccinMocha,
        ThemeName::Gruvbox,
        ThemeName::TokyoNight,
        ThemeName::RosePine,
        ThemeName::Monotropic,
    ];

    pub fn next(self) -> Self {
        let idx = Self::ALL.iter().position(|&t| t == self).unwrap_or(0);
        Self::ALL[(idx + 1) % Self::ALL.len()]
    }

    pub fn label(self) -> &'static str {
        match self {
            ThemeName::Bladerunner => "Blade Runner",
            ThemeName::CatppuccinMocha => "Catppuccin Mocha",
            ThemeName::Gruvbox => "Gruvbox",
            ThemeName::TokyoNight => "Tokyo Night",
            ThemeName::RosePine => "Rosé Pine",
            ThemeName::Monotropic => "Monotropic",
        }
    }
}

impl Default for ThemeName {
    fn default() -> Self {
        ThemeName::Bladerunner
    }
}

pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub secondary: Color,
    pub hot: Color,
    pub dim: Color,
}

impl Theme {
    pub fn from_name(name: ThemeName) -> Self {
        match name {
            ThemeName::Bladerunner => Self::bladerunner(),
            ThemeName::CatppuccinMocha => Self::catppuccin_mocha(),
            ThemeName::Gruvbox => Self::gruvbox(),
            ThemeName::TokyoNight => Self::tokyo_night(),
            ThemeName::RosePine => Self::rose_pine(),
            ThemeName::Monotropic => Self::monotropic(),
        }
    }

    fn bladerunner() -> Self {
        Self {
            bg: Color::Rgb(0x0B, 0x0F, 0x1A),
            fg: Color::Rgb(0xAA, 0xB4, 0xD6),
            accent: Color::Rgb(0x00, 0xF0, 0xFF),
            secondary: Color::Rgb(0x6A, 0x5A, 0xCD),
            hot: Color::Rgb(0xFF, 0x4F, 0xD8),
            dim: Color::Rgb(0x4D, 0x56, 0x70),
        }
    }

    fn catppuccin_mocha() -> Self {
        Self {
            bg: Color::Rgb(0x1E, 0x1E, 0x2E),
            fg: Color::Rgb(0xCD, 0xD6, 0xF4),
            accent: Color::Rgb(0x89, 0xB4, 0xFA),
            secondary: Color::Rgb(0xCB, 0xA6, 0xF7),
            hot: Color::Rgb(0xF5, 0xC2, 0xE7),
            dim: Color::Rgb(0x6C, 0x70, 0x86),
        }
    }

    fn gruvbox() -> Self {
        Self {
            bg: Color::Rgb(0x28, 0x28, 0x28),
            fg: Color::Rgb(0xEB, 0xDB, 0xB2),
            accent: Color::Rgb(0x83, 0xA5, 0x98),
            secondary: Color::Rgb(0xD3, 0x86, 0x9B),
            hot: Color::Rgb(0xFE, 0x80, 0x19),
            dim: Color::Rgb(0x92, 0x83, 0x74),
        }
    }

    fn tokyo_night() -> Self {
        Self {
            bg: Color::Rgb(0x1A, 0x1B, 0x26),
            fg: Color::Rgb(0xA9, 0xB1, 0xD6),
            accent: Color::Rgb(0x7A, 0xA2, 0xF7),
            secondary: Color::Rgb(0xBB, 0x9A, 0xF7),
            hot: Color::Rgb(0xFF, 0x75, 0x7F),
            dim: Color::Rgb(0x56, 0x5F, 0x89),
        }
    }

    fn rose_pine() -> Self {
        Self {
            bg: Color::Rgb(0x19, 0x17, 0x24),
            fg: Color::Rgb(0xE0, 0xDE, 0xF4),
            accent: Color::Rgb(0x31, 0x74, 0x8F),
            secondary: Color::Rgb(0xC4, 0xA7, 0xE7),
            hot: Color::Rgb(0xEB, 0xBB, 0xBA),
            dim: Color::Rgb(0x6E, 0x6A, 0x86),
        }
    }

    fn monotropic() -> Self {
        Self {
            bg: Color::Rgb(0xFF, 0xFF, 0xFA),
            fg: Color::Rgb(0x11, 0x11, 0x11),
            accent: Color::Rgb(0x59, 0x3E, 0x2C),
            secondary: Color::Rgb(0x74, 0x5B, 0x4B),
            hot: Color::Rgb(0x8F, 0x79, 0x6C),
            dim: Color::Rgb(0xAA, 0x99, 0x8E),
        }
    }

    pub fn base(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
    }

    pub fn frame(&self) -> Style {
        Style::default().fg(self.dim).bg(self.bg)
    }

    pub fn title(&self) -> Style {
        Style::default()
            .fg(self.secondary)
            .bg(self.bg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn accent(&self) -> Style {
        Style::default().fg(self.accent).bg(self.bg)
    }

    pub fn hot(&self) -> Style {
        Style::default()
            .fg(self.hot)
            .bg(self.bg)
            .add_modifier(Modifier::BOLD)
    }
}
