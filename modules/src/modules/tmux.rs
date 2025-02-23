use crate::{
    colors::{Color, Style},
    icons::Icon,
};
use std::fmt;

use super::ToModule;

pub enum TmuxContent {
    SessionName,
    WindowName,
    WindowIndex,
    PaneIndex,
    Hostname,
}

impl fmt::Display for TmuxContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match self {
            Self::SessionName => "#S",
            Self::WindowName => "#W",
            Self::WindowIndex => "#I",
            Self::PaneIndex => "#P",
            Self::Hostname => "#H",
        };
        write!(f, "{}", content)
    }
}

impl ToModule for TmuxContent {
    fn icon(&self) -> Option<Icon> {
        match self {
            Self::SessionName => Some(Icon::Tmux),
            Self::Hostname => Some(Icon::TripleServer),
            _ => None,
        }
    }
    fn style(&self) -> Style {
        let fg = match self {
            Self::SessionName => Color::Blue,
            Self::Hostname => Color::Green,
            _ => Color::Reset,
        };
        Style::new_with_fg(fg)
    }
}
