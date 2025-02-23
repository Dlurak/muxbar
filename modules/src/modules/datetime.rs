use super::ToModule;
use crate::{
    colors::{Color, Style},
    icons::Icon,
};
use chrono::Local;
use std::fmt;

pub struct DateTime<'a> {
    format: &'a str,
}

impl<'a> DateTime<'a> {
    pub const fn new(format: &'a str) -> Self {
        Self { format }
    }

    pub const fn date() -> Self {
        Self { format: "%d.%m.%Y" }
    }

    pub const fn time() -> Self {
        Self { format: "%H:%M" }
    }
}

impl fmt::Display for DateTime<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Local::now().format(self.format))
    }
}

impl ToModule for DateTime<'_> {
    fn icon(&self) -> Option<Icon> {
        Some(Icon::Time)
    }
    fn style(&self) -> Style {
        Style::new_with_fg(Color::Magenta)
    }
}
