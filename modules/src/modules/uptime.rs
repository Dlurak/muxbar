use sysinfo::System;

use super::ToModule;
use crate::{
    colors::{Color, Style},
    icons::Icon,
    pretty_duration::PrettyDuration,
};
use std::{fmt, time::Duration};

pub struct Uptime(Duration);

impl Uptime {
    pub fn new() -> Self {
        let duration = Duration::from_secs(System::uptime());
        Self(duration)
    }
}

impl Default for Uptime {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Uptime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", PrettyDuration::new(self.0))
    }
}

impl ToModule for Uptime {
    fn icon(&self) -> Option<Icon> {
        Some(Icon::Time)
    }
    fn style(&self) -> Style {
        Style::new_with_fg(Color::Yellow)
    }
}
