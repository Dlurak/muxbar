use std::fmt::Display;

/// Enum representing various colors
///
/// These colors are used for foreground and background styling.
/// See [tmux color palette](https://superuser.com/questions/285381/how-does-the-tmux-color-palette-work)
/// for strings to be used with the `Any`` variant
#[derive(Clone, Copy)]
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
    Any(&'static str),
}

/// Struct representing a style with foreground and background colors and bold attribute
#[derive(Clone, Copy)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
}

impl Display for Style {
    /// Formats the style for display
    ///
    /// Combines foreground color, background color, and bold attribute into a single string.
    ///
    /// # Arguments
    /// * `f` - Formatter to write the formatted string
    ///
    /// # Returns
    /// A Result indicating success or failure
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.fg.get_tmux_color_code(false),
            self.bg.get_tmux_color_code(true),
            bold(self.bold),
        )
    }
}

impl Display for Color {
    /// Formats the color for display
    ///
    /// Converts the color into its corresponding foreground color string.
    ///
    /// # Arguments
    /// * `f` - Formatter to write the formatted string
    ///
    /// # Returns
    /// A Result indicating success or failure
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_tmux_color_code(false))
    }
}

impl Default for Style {
    /// Provides a default style
    ///
    /// The default style has both foreground and background colors set to Reset and bold set to false.
    ///
    /// # Returns
    /// A default Style instance
    fn default() -> Self {
        Self {
            fg: Color::Reset,
            bg: Color::Reset,
            bold: false,
        }
    }
}

impl Color {
    /// Returns the tmux color code string for a given color
    /// # Arguments
    /// * `background` - A boolean indicating if the color is for background
    ///
    /// # Returns
    /// A string representing the tmux color code
    pub fn get_tmux_color_code(&self, background: bool) -> String {
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

/// Returns the bold attribute string based on a boolean value
///
/// # Arguments
/// * `is_bold` - Boolean indicating whether the text should be bold
///
/// # Returns
/// A static string representing the bold attribute
fn bold(is_bold: bool) -> &'static str {
    if is_bold {
        "#[bold]"
    } else {
        "#[nobold]"
    }
}
