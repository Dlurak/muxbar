use super::ToModule;
use crate::{
    colors::{Color, Style},
    icons::Icon,
};
use std::fmt;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub struct Memory {
    usage: f32,
    minimum_digits: usize,
    decimal_places: usize,
}

impl Memory {
    pub fn new(minimum_digits: usize, decimal_places: usize) -> Self {
        Self {
            usage: Self::total_usage(),
            minimum_digits,
            decimal_places,
        }
    }

    fn total_usage() -> f32 {
        let mut sys = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
        );

        sys.refresh_memory();

        (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0width$.prec$}%",
            self.usage,
            width = self.minimum_digits,
            prec = self.decimal_places
        )
    }
}

impl ToModule for Memory {
    fn style(&self) -> Style {
        Style::new_with_fg(Color::Yellow)
    }

    fn icon(&self) -> Option<Icon> {
        Some(Icon::DoubleServer)
    }
}
