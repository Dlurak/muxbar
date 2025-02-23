use super::ToModule;
use crate::{
    colors::{Color, Style},
    icons::Icon,
};
use std::{fmt, thread};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct Cpu {
    usage: f32,
    minimum_digits: usize,
    decimal_places: usize,
}

impl Cpu {
    pub fn new(minimum_digits: usize, decimal_places: usize) -> Self {
        Self {
            usage: Self::total_average(),
            minimum_digits,
            decimal_places,
        }
    }

    fn total_average() -> f32 {
        let mut s =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        s.refresh_cpu();

        let cpus = s.cpus();

        let cpu_sum = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();

        cpu_sum / cpus.len() as f32
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
}
