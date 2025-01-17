use crate::{colors::Style, icons::Icon};
use chrono::Local;
use std::fmt;

pub trait Content {
    fn show(&self) -> Option<String>;
}

pub struct Module<T: Content> {
    pub icon: Icon,
    pub style: Style,
    pub content: T,
}

impl<T: Content> fmt::Display for Module<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} {}{}",
            self.style,
            self.icon,
            self.content.show().unwrap_or_default(),
            Style::default()
        )
    }
}

pub struct TimeModule;

impl Content for TimeModule {
    fn show(&self) -> Option<String> {
        Some(Local::now().format("%H:%M").to_string())
    }
}

pub struct DateModule;

impl Content for DateModule {
    fn show(&self) -> Option<String> {
        Some(Local::now().format("%Y-%m-%d").to_string())
    }
}
