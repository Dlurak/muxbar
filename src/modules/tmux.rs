use std::fmt;

use crate::{
    colors::{Color, Style},
    icons::Icon,
};

use super::Module;

/// Represents different types of TMux session information that can be displayed
pub enum TmuxContent {
    /// The name of the current TMux session (#S)
    SessionName,
    /// The name of the current TMux window (#W)
    WindowName,
    /// The index of the current TMux window (#I)
    WindowIndex,
    /// The index of the current TMux pane (#P)
    PaneIndex,
    /// The hostname of the TMux server (#H)
    Hostname,
}

impl fmt::Display for TmuxContent {
    /// Formats the TMux content as the corresponding TMux format string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TmuxContent::SessionName => "#S",
                TmuxContent::WindowName => "#W",
                TmuxContent::WindowIndex => "#I",
                TmuxContent::PaneIndex => "#P",
                TmuxContent::Hostname => "#H",
            }
        )
    }
}

impl TmuxContent {
    /// Creates a standard styled Module for the TMux content type
    ///
    /// Returns a boxed Module with predefined styling based on the content type:
    /// - SessionName: Cyan with SimpleTux icon
    /// - Hostname: Green with DoubleServer icon
    /// - WindowName: Blue with TMux icon
    /// - Others: Default style with SimpleTux icon
    pub fn get_standard(self) -> Box<Module<TmuxContent>> {
        match self {
            TmuxContent::SessionName => Box::new(Module::new(
                self,
                Some(Icon::SimpleTux),
                Style {
                    fg: Color::Cyan,
                    bg: Color::Reset,
                    bold: false,
                },
            )),
            TmuxContent::Hostname => Box::new(Module::new(
                self,
                Some(Icon::DoubleServer),
                Style {
                    fg: Color::Green,
                    bg: Color::Reset,
                    bold: false,
                },
            )),
            TmuxContent::WindowName => Box::new(Module::new(
                self,
                Some(Icon::Tmux),
                Style {
                    fg: Color::Blue,
                    bg: Color::Reset,
                    bold: false,
                },
            )),
            _ => Box::new(Module::new(self, Some(Icon::SimpleTux), Style::default())),
        }
    }
}
