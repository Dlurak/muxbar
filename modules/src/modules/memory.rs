use super::ToModule;
use crate::{
    colors::{Color, Style},
    icons::Icon,
};
use std::{
    fmt,
    time::{Duration, Instant},
};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

pub struct Memory {
    usage: f32,
    minimum_digits: usize,
    decimal_places: usize,
    system: System,
}

impl Memory {
    pub fn new(minimum_digits: usize, decimal_places: usize) -> Self {
        let specifics = RefreshKind::new().with_memory(MemoryRefreshKind::everything());
        let mut system = System::new_with_specifics(specifics);
        system.refresh_memory();

        let usage = (system.used_memory() as f32 / system.total_memory() as f32) * 100.0;

        Self {
            usage,
            minimum_digits,
            decimal_places,
            system,
        }
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

    fn update(&mut self) {
        let system = &mut self.system;
        system.refresh_memory();
        self.usage = (system.used_memory() as f32 / system.total_memory() as f32) * 100.0;
    }

    fn next_render_time(&self) -> Option<Instant> {
        Some(Instant::now() + Duration::from_secs_f32(7.5))
    }
}
