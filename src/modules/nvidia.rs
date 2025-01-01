use super::Module;
use crate::colors::{Color, Style};
use crate::icons::Icon;
use std::fmt;
use std::process::Command;
use std::str;

pub struct Nvidia {
    pub memory_used: String,
    pub memory_total: String,
    pub view_as_percentage: bool,
}

impl fmt::Display for Nvidia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.memory_used == "N/A" || self.memory_total == "N/A" {
            write!(f, "")
        } else if self.view_as_percentage {
            let used: f64 = self.memory_used.parse().unwrap_or(0.0);
            let total: f64 = self.memory_total.parse().unwrap_or(1.0);
            write!(f, "{:>3.0}%", (used / total) * 100.0)
        } else {
            write!(f, "{:>4}/{:>4}", self.memory_used, self.memory_total)
        }
    }
}

impl Nvidia {
    fn empty() -> Module<Nvidia> {
        Module::new(
            Nvidia {
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

    pub fn new(view_as_percentage: bool) -> Box<Module<Nvidia>> {
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
                    Nvidia {
                        memory_used,
                        memory_total,
                        view_as_percentage,
                    },
                    Some(Icon::Nvidia),
                    Style {
                        fg: Color::Any("color34"),
                        bg: Color::Reset,
                        bold: false,
                    },
                ))
            }
            Err(_) => Box::new(Nvidia::empty()),
        }
    }
}
