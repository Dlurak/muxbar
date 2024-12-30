//
// ❯ nvidia-smi --query-gpu=memory.used,memory.total --format=csv
//
// memory.used [MiB], memory.total [MiB]
// 3 MiB, 2048 MiB

use super::Module;
use crate::colors::{Color, Style};
use crate::icons::Icon;
use std::fmt;
use std::process::Command;
use std::str;

pub struct NvidiaModule {
    pub memory_used: String,
    pub memory_total: String,
}

impl fmt::Display for NvidiaModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.memory_used == "N/A" || self.memory_total == "N/A" {
            write!(f, "")
        } else {
            write!(f, "GPU: {:>4}/{:>4}", self.memory_used, self.memory_total)
        }
    }
}

impl NvidiaModule {
    fn empty() -> Module<NvidiaModule> {
        Module::new(
            NvidiaModule {
                memory_used: "N/A".to_string(),
                memory_total: "N/A".to_string(),
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
