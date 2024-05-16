// Those are only constructed in config.rs
#[allow(dead_code)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
    Magenta,
    Reset,
}

pub struct Style {
    fg: Color,
    bg: Color,
}

impl Style {
    pub fn new(fg: Color, bg: Color) -> Self {
        Self { fg, bg }
    }

    pub fn default() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }

    pub fn display(self) -> String {
        format!("{}{}", foreground_color(self.fg), background_color(self.bg))
    }
}

pub fn foreground_color(color: Color) -> &'static str {
    match color {
        Color::White => "#[fg=white]",
        Color::Black => "#[fg=black]",
        Color::Red => "#[fg=red]",
        Color::Green => "#[fg=green]",
        Color::Yellow => "#[fg=yellow]",
        Color::Blue => "#[fg=blue]",
        Color::Cyan => "#[fg=cyan]",
        Color::Magenta => "#[fg=magenta]",
        Color::Reset => "#[default]",
    }
}

pub fn background_color(color: Color) -> &'static str {
    match color {
        Color::White => "#[bg=white]",
        Color::Black => "#[bg=black]",
        Color::Red => "#[bg=red]",
        Color::Green => "#[bg=green]",
        Color::Yellow => "#[bg=yellow]",
        Color::Blue => "#[bg=blue]",
        Color::Cyan => "#[bg=cyan]",
        Color::Magenta => "#[bg=magenta]",
        Color::Reset => "#[bg=default]",
    }
}
