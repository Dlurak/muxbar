/// Color of a Module
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
    /// Reset the color the default
    #[default]
    Reset,
    /// Any color, see [this reference](https://superuser.com/questions/285381/how-does-the-tmux-color-palette-work)
    Any(&'static str),
}

impl Color {
    fn get_tmux_color_code(&self, background: bool) -> String {
        let prefix = if background { "bg" } else { "fg" };
        match self {
            Self::White => format!("#[{}=white]", prefix),
            Self::Black => format!("#[{}=black]", prefix),
            Self::Red => format!("#[{}=red]", prefix),
            Self::Green => format!("#[{}=green]", prefix),
            Self::Yellow => format!("#[{}=yellow]", prefix),
            Self::Blue => format!("#[{}=blue]", prefix),
            Self::Cyan => format!("#[{}=cyan]", prefix),
            Self::Magenta => format!("#[{}=magenta]", prefix),
            Self::Reset => format!("#[{}=default]", prefix),
            Self::Any(color) => format!("#[{}={}]", prefix, color),
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
    pub const fn new_with_fg(fg: Color) -> Self {
        Self {
            fg,
            bg: Color::Reset,
            bold: false
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

const fn bold(is_bold: bool) -> &'static str {
    if is_bold {
        "#[bold]"
    } else {
        "#[nobold]"
    }
}
