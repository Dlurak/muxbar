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
use std::{fmt, time::Instant};

/// A trait that defines the neccessary methods for items that can be converted into modules. These
/// are only defaults. The style and icon can be overwritten
pub trait ToModule: fmt::Display {
    /// Returns the styling information for the module.
    fn style(&self) -> Style;

    /// Returns an optional icon for the module.
    fn icon(&self) -> Option<Icon>;

    fn next_render_time(&self) -> Option<Instant> {
        None
    }

    fn update(&mut self) {}

    fn include(&self) -> bool {
        true
    }
}

/// A struct that represents a module in the tmux status bar, containing content, an optional icon, and styling information.
#[derive(Clone, Hash)]
pub struct Module<T: fmt::Display> {
    /// The content to be displayed in the module.
    pub content: T,

    /// An optional icon to be displayed alongside the content.
    icon: (Option<Icon>, bool),

    /// The style to be applied to the module's content.
    style: (Style, bool),
}

impl<T: ToModule> From<T> for Module<T> {
    /// Converts a value that implements `ToModule` into a `Module`.
    ///
    /// This method initializes a `Module` struct by using the `style` and `icon` methods of
    /// the input type to set the style and icon, while using the input value as the content.
    fn from(value: T) -> Self {
        Self {
            icon: (value.icon(), false),
            style: (value.style(), false),
            content: value,
        }
    }
}

impl<T: ToModule + 'static> From<T> for Module<Box<dyn ToModule>> {
    fn from(value: T) -> Self {
        Self {
            icon: (value.icon(), false),
            style: (value.style(), false),
            content: Box::new(value) as Box<dyn ToModule>,
        }
    }
}

impl<T: ToModule + Default> Default for Module<T> {
    fn default() -> Self {
        let content = T::default();
        Self {
            icon: (content.icon(), false),
            style: (content.style(), false),
            content,
        }
    }
}

impl<T: fmt::Display> fmt::Display for Module<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = self.icon.0.map(|icon| icon.to_string()).unwrap_or_default();
        write!(f, "{}{} {}", self.style.0.display(), icon, self.content,)
    }
}

impl<T: fmt::Display> Module<T> {
    pub fn new(content: T) -> Self {
        Self {
            content,
            icon: (None, false),
            style: (Style::default(), false),
        }
    }

    pub fn set_icon(self, icon: Option<Icon>) -> Self {
        Self {
            icon: (icon, true),
            ..self
        }
    }

    pub fn set_style(self, style: Style) -> Self {
        Self {
            style: (style, true),
            ..self
        }
    }

    /// For internal use only!
    pub fn internal_set_mut_icon(&mut self, icon: Option<Icon>) {
        // do not overwrite user defined icons
        if !self.icon.1 {
            self.icon.0 = icon;
        }
    }

    /// For internal use only!
    pub fn internal_set_mut_style(&mut self, style: Style) {
        if !self.style.1 {
            self.style.0 = style;
        }
    }
}

impl Module<Box<dyn ToModule>> {
    pub fn update(&mut self) {
        self.content.update()
    }
}
