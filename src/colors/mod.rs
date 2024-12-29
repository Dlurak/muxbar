use std::fmt::Display;

/// Enum representing various colors
///
/// These colors are used for foreground and background styling.
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
            foreground_color(self.fg),
            background_color(self.bg),
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
        write!(f, "{}", foreground_color(*self))
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

/// Returns the foreground color string for a given color
///
/// # Arguments
/// * `color` - The Color enum value
///
/// # Returns
/// A static string representing the foreground color
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

/// Returns the background color string for a given color
///
/// # Arguments
/// * `color` - The Color enum value
///
/// # Returns
/// A static string representing the background color
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
