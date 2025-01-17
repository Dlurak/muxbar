pub mod battery;
pub mod datetime;
pub mod high_cpu;
pub mod nvidia;
pub mod systemstats;
pub mod tmux;

use std::fmt;

use crate::colors;
use crate::colors::Style;
use crate::icons::Icon;

#[derive(Clone, Copy)]
pub struct Module<T> {
    content: T,
    icon: Option<Icon>,
    style: Style,
}

impl<T> Module<T> {
    pub fn new(content: T, icon: Option<Icon>, style: colors::Style) -> Self {
        Self {
            content,
            icon,
            style,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Module<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = format!(
            "{}{} {}{}",
            self.style,
            self.icon.unwrap_or(Icon::Empty),
            self.content,
            Style::default()
        );
        match format!("{}{}", self.icon.unwrap_or(Icon::Empty), self.content)
            .trim()
            .is_empty()
        {
            true => write!(f, ""),
            false => write!(f, "{}", res),
        }
    }
}

impl<T> Default for Module<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            content: T::default(),
            icon: None,
            style: Style::default(),
        }
    }
}
