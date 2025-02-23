pub mod styled;

use crate::utils::strings;
use crate::utils::system::{battery::BatteryInformation, cpu};
use chrono::{DateTime, Local};
use std::time::Duration;
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

// Those are only constructed in config.rs
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Module {
    Manual(&'static str),
    Time(&'static str),
    Battery,
    Cpu(usize),
    Memory(usize),
    Swap(usize),
    Uptime,
    SessionName,
    WindowName,
    WindowIndex,
    PaneIndex,
    Hostname,
}

impl Module {
    fn display(self) -> Option<String> {
        match self {
            Module::Manual(s) => Some(String::from(s)),
            Module::Time(format) => {
                let now: DateTime<Local> = Local::now();

                Some(now.format(format).to_string())
            }
            Module::Battery => {
                let info = BatteryInformation::new().ok()??;
                Some(format!("{}%", info.percentages))
            }
            Module::SessionName => Some(String::from("#S")),
            Module::WindowName => Some(String::from("#W")),
            Module::WindowIndex => Some(String::from("#I")),
            Module::PaneIndex => Some(String::from("#P")),
            Module::Hostname => Some(String::from("#H")),
            Module::Cpu(rounding) => Some(strings::round(cpu::get_total_average(), rounding)),
            Module::Memory(rounding) => {
                let mut sys = System::new_with_specifics(
                    RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
                );

                sys.refresh_memory();

                let total_memory = sys.total_memory();
                let used_memory = sys.used_memory();

                let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
                Some(strings::round(memory_usage_percent, rounding))
            }
            Module::Uptime => {
                let uptime = System::uptime();
                let uptime = Duration::from_secs(uptime);

                Some(format!("{}", strings::PrettyDuration::new(uptime)))
            }
            Module::Swap(rounding) => {
                let mut sys = System::new_with_specifics(
                    RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
                );

                sys.refresh_memory();

                let total_swap = sys.total_swap();
                let used_swap = sys.used_swap();

                let swap_usage_percent = (used_swap as f64 / total_swap as f64) * 100.0;
                Some(strings::round(swap_usage_percent, rounding))
            }
        }
    }
}
