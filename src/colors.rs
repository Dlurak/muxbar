// Those are only constructed in config.rs
#[allow(dead_code)]
#[derive(Clone, Copy, Default)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
    Magenta,
    #[default]
    Reset,
    Any(&'static str),
}

impl Color {
    fn get_tmux_color_code(&self, background: bool) -> String {
        let prefix = if background { "bg" } else { "fg" };
        match self {
            Color::White => format!("#[{}=white]", prefix),
            Color::Black => format!("#[{}=black]", prefix),
            Color::Red => format!("#[{}=red]", prefix),
            Color::Green => format!("#[{}=green]", prefix),
            Color::Yellow => format!("#[{}=yellow]", prefix),
            Color::Blue => format!("#[{}=blue]", prefix),
            Color::Cyan => format!("#[{}=cyan]", prefix),
            Color::Magenta => format!("#[{}=magenta]", prefix),
            Color::Reset => format!("#[{}=default]", prefix),
            Color::Any(color) => format!("#[{}={}]", prefix, color),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
}

impl Style {
    pub fn default() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            bold: false,
        }
    }

    pub fn display(self) -> String {
        format!(
            "{}{}{}",
            self.fg.get_tmux_color_code(false),
            self.bg.get_tmux_color_code(true),
            bold(self.bold),
        )
    }
}

fn bold(is_bold: bool) -> &'static str {
    if is_bold {
        "#[bold]"
    } else {
        "#[nobold]"
    }
}
