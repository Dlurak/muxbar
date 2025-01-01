use super::Module;
use crate::colors::{Color, Style};
use crate::icons::Icon;
use std::collections::HashMap;
use std::fmt;
use std::process::Command;

/// A module to display the process with the highest CPU usage.
///
/// NOTE that the high CPU process is determined by the `ps` command.
/// So, this is only available on linux or macos.
pub struct HighCpu {
    /// Field to store process information
    process_info: String,
}

impl fmt::Display for HighCpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.process_info)
    }
}

impl Default for HighCpu {
    fn default() -> Self {
        Self {
            process_info: "".to_string(),
        }
    }
}

impl HighCpu {
    /// Creates a new instance of HighCpuModule.
    ///
    /// This method runs the `ps` command to get the process information,
    /// processes the output to find the process with the highest CPU usage,
    /// and returns a boxed Module containing the HighCpuModule.
    ///
    /// Note, that this is only available on linux or macos.
    /// When the "ps" command fails, it returns a default Module.
    ///
    /// # Returns
    ///
    /// A `Box<Module<HighCpuModule>>` containing the process information.
    pub fn new(fg: Option<Color>, bg: Option<Color>) -> Box<Module<HighCpu>> {
        let output_raw = Command::new("ps")
            .arg("axo")
            .arg("pid,pcpu,comm")
            .output()
            .ok();
        let output = match output_raw {
            Some(output) => output,
            None => return Box::new(Module::default()),
        };

        let process_info = if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut process_map: HashMap<String, f32> = HashMap::new();

            stdout.lines().skip(1).for_each(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let (Some(cpu_usage), Some(name)) = (parts.get(1), parts.get(2)) {
                    let cpu_usage: f32 = cpu_usage.parse().unwrap_or(0.0);
                    if cpu_usage > 0.001 && *name != "ps" {
                        *process_map.entry(name.to_string()).or_insert(0.0) += cpu_usage;
                    }
                }
            });

            process_map
                .into_iter()
                .max_by_key(|&(_, cpu_usage)| (cpu_usage * 100.0) as i32)
                .map(|(name, cpu_usage)| {
                    if name.len() > 30 {
                        format!("{}... {:.2}%", &name[..27], cpu_usage)
                    } else {
                        format!("{} {:.2}%", name, cpu_usage)
                    }
                })
                .unwrap_or_else(|| "No high CPU process found".to_string())
        } else {
            "Failed to get process info".to_string()
        };

        Box::new(Module::new(
            HighCpu { process_info },
            Some(Icon::Manual("󰑓")),
            Style {
                fg: fg.unwrap_or(Color::Any("color61")),
                bg: bg.unwrap_or(Color::Reset),
                bold: true,
            },
        ))
    }
}
