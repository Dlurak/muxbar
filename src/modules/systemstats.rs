use crate::utils::strings;
use std::fmt;
use std::time::Duration;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use std::thread;

/// Represents CPU usage statistics with formatting options
pub struct Cpu {
    /// Number of decimal places to display
    pub decimal_places: usize,
    /// Minimum number of digits to display
    pub minimum_digits: usize,
}
impl Default for Cpu {
    fn default() -> Self {
        Self {
            decimal_places: 0,
            minimum_digits: 2,
        }
    }
}
impl Cpu {
    /// Returns the average CPU usage across all cores as a percentage
    fn get_total_average() -> f32 {
        let mut s = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );

        // NOTE: Documentation says that it should b ecalled twice.
        s.refresh_cpu_all();
        thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        s.refresh_cpu_all();

        let cpus = s.cpus();

        let cpu_sum = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();

        cpu_sum / cpus.len() as f32
    }
}
impl fmt::Display for Cpu {
    /// Formats the CPU usage as a string with specified decimal places and minimum digits
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            strings::round(
                Cpu::get_total_average(),
                self.decimal_places,
                self.minimum_digits,
            )
        )
    }
}

/// Represents memory usage statistics with formatting options
pub struct Memory {
    /// Number of decimal places to display
    pub decimal_places: usize,
    /// Minimum number of digits to display
    pub minimum_digits: usize,
}
impl Default for Memory {
    fn default() -> Self {
        Self {
            decimal_places: 0,
            minimum_digits: 2,
        }
    }
}
impl fmt::Display for Memory {
    /// Formats the memory usage as a percentage string with specified decimal places and minimum digits
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
        );

        sys.refresh_memory();

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();

        let memory_usage_percent = (used_memory as f64 / total_memory as f64) * 100.0;
        write!(
            f,
            "{}",
            strings::round(
                memory_usage_percent,
                self.decimal_places,
                self.minimum_digits
            )
        )
    }
}

/// Represents swap memory usage statistics with formatting options
pub struct Swap {
    /// Number of decimal places to display
    pub decimal_places: usize,
    /// Minimum number of digits to display
    pub minimum_digits: usize,
}
impl Default for Swap {
    fn default() -> Self {
        Self {
            decimal_places: 0,
            minimum_digits: 2,
        }
    }
}
impl fmt::Display for Swap {
    /// Formats the swap usage as a percentage string with specified decimal places and minimum digits
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_memory(MemoryRefreshKind::everything()),
        );

        sys.refresh_memory();

        let total_swap = sys.total_swap();
        let used_swap = sys.used_swap();

        let swap_usage_percent = (used_swap as f64 / total_swap as f64) * 100.0;
        write!(
            f,
            "{}",
            strings::round(swap_usage_percent, self.decimal_places, self.minimum_digits)
        )
    }
}

/// Represents system uptime information
pub struct Uptime;
impl fmt::Display for Uptime {
    /// Formats the system uptime as a human-readable duration string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let uptime = System::uptime();
        let uptime = Duration::from_secs(uptime);
        write!(f, "{}", strings::PrettyDuration::new(uptime))
    }
}
