pub mod styled;

use crate::utils::strings;
use crate::utils::system::{battery, cpu};
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
    Uptime,
    SessionName,
    WindowName,
    WindowIndex,
    PaneIndex,
    Hostname,
}

impl Module {
    fn display(self) -> Result<String, ()> {
        match self {
            Module::Manual(s) => Ok(String::from(s)),
            Module::Time(format) => {
                let now: DateTime<Local> = Local::now();

                Ok(now.format(format).to_string())
            }
            Module::Battery => battery::battery_status().map(|perc| format!("{}%", perc)),
            Module::SessionName => Ok(String::from("#S")),
            Module::WindowName => Ok(String::from("#W")),
            Module::WindowIndex => Ok(String::from("#I")),
            Module::PaneIndex => Ok(String::from("#P")),
            Module::Hostname => Ok(String::from("#H")),
            Module::Cpu(rounding) => Ok(strings::round(cpu::get_total_average(), rounding)),
            Module::Memory(rounding) => {
                let mut sys = System::new_with_specifics(
                    RefreshKind::new().with_memory(MemoryRefreshKind::everything()),
                );

                sys.refresh_memory();

                let total_memory = sys.total_memory();
                let used_memory = sys.used_memory();

                let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
                Ok(strings::round(memory_usage_percent, rounding))
            }
            Module::Uptime => {
                let uptime = System::uptime();
                let uptime = Duration::from_secs(uptime);

                Ok(format!("{}", strings::PrettyDuration::new(uptime)))
            }
        }
    }
}
