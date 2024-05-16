// Those are only constructed in config.rs
#[allow(dead_code)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
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
        Color::Red => "#[fg=red]",
        Color::Green => "#[fg=green]",
        Color::Yellow => "#[fg=yellow]",
        Color::Blue => "#[fg=blue]",
        Color::Reset => "#[default]",
    }
}

pub fn background_color(color: Color) -> &'static str {
    match color {
        Color::Red => "#[bg=red]",
        Color::Green => "#[bg=green]",
        Color::Yellow => "#[bg=yellow]",
        Color::Blue => "#[bg=blue]",
        Color::Reset => "#[bg=default]",
    }
}
