/// This module provides functionality for displaying date and time.
use chrono::Local;
use core::fmt;

use crate::{
    colors::{Color, Style},
    icons::Icon,
};

use super::Module;

/// A struct representing a DateTime format.
pub struct DateTime(pub &'static str);

impl fmt::Display for DateTime {
    /// Formats the DateTime using the provided format string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Local::now().format(self.0))
    }
}

impl DateTime {
    /// Creates a new `Module` displaying the current date.
    ///
    /// # Returns
    ///
    /// A boxed `Module` containing the current date.
    pub fn date() -> Box<Module<DateTime>> {
        Box::new(Module::new(
            DateTime("%Y-%m-%d"),
            Some(Icon::Date),
            Style {
                fg: Color::Green,
                bg: Color::Reset,
                bold: false,
            },
        ))
    }

    /// Creates a new `Module` displaying the current time.
    ///
    /// # Returns
    ///
    /// A boxed `Module` containing the current time.
    pub fn time() -> Box<Module<DateTime>> {
        Box::new(Module::new(
            DateTime("%H:%M"),
            Some(Icon::Time),
            Style {
                fg: Color::Magenta,
                bg: Color::Reset,
                bold: false,
            },
        ))
    }
}
