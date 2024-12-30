use super::Module;
use crate::colors::{Color, Style};
use crate::icons::Icon;
use std::fmt;
use std::process::Command;
use std::str;

pub struct NvidiaModule {
    pub memory_used: String,
    pub memory_total: String,
    pub view_as_percentage: bool,
}

impl fmt::Display for NvidiaModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.memory_used == "N/A" || self.memory_total == "N/A" {
            write!(f, "")
        } else if self.view_as_percentage {
            write!(f, "{}", self.memory_used)
        } else {
            write!(f, "{:>4}/{:>4}", self.memory_used, self.memory_total)
        }
    }
}

impl NvidiaModule {
    fn empty() -> Module<NvidiaModule> {
        Module::new(
            NvidiaModule {
                memory_used: "N/A".to_string(),
                memory_total: "N/A".to_string(),
                view_as_percentage: false,
            },
            None,
            Style {
                fg: Color::Reset,
                bg: Color::Reset,
                bold: false,
            },
        )
    }

    pub fn new() -> Box<Module<NvidiaModule>> {
        let output = Command::new("nvidia-smi")
            .arg("--query-gpu=memory.used,memory.total")
            .arg("--format=csv,noheader,nounits")
            .output();

        match output {
            Ok(output) => {
                let output_str = str::from_utf8(&output.stdout).unwrap_or("");
                let mut parts = output_str.split(',');
                let memory_used = parts.next().unwrap_or("N/A").trim().to_string();
                let memory_total = parts.next().unwrap_or("N/A").trim().to_string();
                Box::new(Module::new(
                    NvidiaModule {
                        memory_used,
                        memory_total,
                        view_as_percentage: false,
                    },
                    Some(Icon::Nvidia),
                    Style {
                        fg: Color::Any("color34"),
                        bg: Color::Reset,
                        bold: false,
                    },
                ))
            }
            Err(_) => Box::new(NvidiaModule::empty()),
        }
    }

    pub fn new_percentage() -> Box<Module<NvidiaModule>> {
        let output = Command::new("nvidia-smi")
            .arg("--query-gpu=memory.used,memory.total")
            .arg("--format=csv,noheader,nounits")
            .output();

        match output {
            Ok(output) => {
                let output_str = str::from_utf8(&output.stdout).unwrap_or("");
                let mut parts = output_str.split(',');
                let memory_used = parts.next().unwrap_or("N/A").trim().to_string();
                let memory_total = parts.next().unwrap_or("N/A").trim().to_string();

                let percentage = if memory_used != "N/A" && memory_total != "N/A" {
                    let used: f64 = memory_used.parse().unwrap_or(0.0);
                    let total: f64 = memory_total.parse().unwrap_or(1.0);
                    format!("{:>2.0}%", (used / total) * 100.0)
                } else {
                    "N/A".to_string()
                };

                Box::new(Module::new(
                    NvidiaModule {
                        memory_used: percentage.clone(),
                        memory_total: percentage,
                        view_as_percentage: true,
                    },
                    Some(Icon::Nvidia),
                    Style {
                        fg: Color::Any("color34"),
                        bg: Color::Reset,
                        bold: false,
                    },
                ))
            }
            Err(_) => Box::new(NvidiaModule::empty()),
        }
    }
}
