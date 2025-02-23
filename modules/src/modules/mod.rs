mod battery;
mod cpu;
mod datetime;
mod memory;
mod tmux;
mod uptime;
mod warning;

pub use battery::*;
pub use cpu::*;
pub use datetime::*;
pub use memory::*;
pub use tmux::*;
pub use uptime::*;
pub use warning::*;

use crate::{colors::Style, icons::Icon};
use std::fmt;

/// A trait that defines the neccessary methods for items that can be converted into modules. These
/// are only defaults. The style and icon can be overwritten
pub trait ToModule {
    /// Returns the styling information for the module.
    fn style(&self) -> Style;

    /// Returns an optional icon for the module.
    fn icon(&self) -> Option<Icon>;
}

/// A struct that represents a module in the tmux status bar, containing content, an optional icon, and styling information.
pub struct Module<T: fmt::Display> {
    /// The content to be displayed in the module.
    content: T,

    /// An optional icon to be displayed alongside the content.
    icon: Option<Icon>,

    /// The style to be applied to the module's content.
    style: Style,
}

impl<T: fmt::Display + ToModule> From<T> for Module<T> {
    /// Converts a value that implements `ToModule` into a `Module`.
    ///
    /// This method initializes a `Module` struct by using the `style` and `icon` methods of
    /// the input type to set the style and icon, while using the input value as the content.
    fn from(value: T) -> Self {
        Self {
            icon: value.icon(),
            style: value.style(),
            content: value,
        }
    }
}

impl<T: fmt::Display + Sync + ToModule + 'static> From<T> for Module<Box<dyn fmt::Display + Sync>> {
    fn from(value: T) -> Self {
        Self {
            icon: value.icon(),
            style: value.style(),
            content: Box::new(value) as Box<dyn fmt::Display + Sync>,
        }
    }
}

impl<T: fmt::Display + Default + ToModule> Default for Module<T> {
    fn default() -> Self {
        let content = T::default();
        Self {
            icon: content.icon(),
            style: content.style(),
            content,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Module<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = self.icon.map(|icon| icon.to_string()).unwrap_or_default();
        write!(
            f,
            "{}{} {}{}",
            self.style.display(),
            icon,
            self.content,
            Style::default().display()
        )
    }
}

impl<T: fmt::Display> Module<T> {
    pub fn new(content: T) -> Self {
        Self {
            content,
            icon: None,
            style: Style::default(),
        }
    }

    pub fn set_icon(self, icon: Option<Icon>) -> Self {
        Self { icon, ..self }
    }

    pub fn set_style(self, style: Style) -> Self {
        Self { style, ..self }
    }
}
