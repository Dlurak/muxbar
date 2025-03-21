/// Color of a Module
#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
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

impl TryFrom<Color> for ansi_term::Colour {
    type Error = ();
    fn try_from(value: Color) -> Result<Self, Self::Error> {
        match value {
            Color::Black => Ok(Self::Black),
            Color::White => Ok(Self::White),
            Color::Red => Ok(Self::Red),
            Color::Green => Ok(Self::Green),
            Color::Yellow => Ok(Self::Yellow),
            Color::Blue => Ok(Self::Blue),
            Color::Cyan => Ok(Self::Cyan),
            Color::Magenta => Ok(Self::Purple),
            Color::Reset | Color::Any(_) => Err(()),
        }
    }
}

#[derive(Clone, Copy, Default, Hash)]
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
            bold: false,
        }
    }
}
