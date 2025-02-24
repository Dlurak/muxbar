use super::ToModule;
use crate::{
    colors::{Color, Style},
    icons::Icon,
};
use std::{
    fmt,
    time::{Duration, Instant},
};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct Cpu {
    usage: f32,
    minimum_digits: usize,
    decimal_places: usize,
    system: System,
}

impl Cpu {
    pub fn new(minimum_digits: usize, decimal_places: usize) -> Self {
        let specifics = RefreshKind::new().with_cpu(CpuRefreshKind::everything());
        let system = System::new_with_specifics(specifics);
        Self {
            usage: 0.0,
            minimum_digits,
            decimal_places,
            system,
        }
    }
}

impl fmt::Display for Cpu {
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

impl ToModule for Cpu {
    fn style(&self) -> Style {
        Style::new_with_fg(Color::Cyan)
    }

    fn icon(&self) -> Option<Icon> {
        Some(Icon::Cpu)
    }

    fn update(&mut self) {
        let s = &mut self.system;

        s.refresh_cpu();

        let cpus = s.cpus();
        let cpu_sum = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();
        self.usage = cpu_sum / cpus.len() as f32;
    }

    fn next_render_time(&self) -> Option<Instant> {
        Some(Instant::now() + Duration::from_secs_f32(2.5))
    }
}
