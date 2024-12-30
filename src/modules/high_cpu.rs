use super::Module;
use crate::colors::{Color, Style};
use crate::icons::Icon;
use std::fmt;
use sysinfo::System;

pub struct HighCpuModule {
    process_info: String,
}

impl fmt::Display for HighCpuModule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.process_info)
    }
}

impl HighCpuModule {
    pub fn new() -> Box<Module<HighCpuModule>> {
        let mut system = System::new_all();
        system.refresh_all();

        let process_info = system
            .processes()
            .values()
            .max_by_key(|p| p.cpu_usage() as u32)
            .map(|p| {
                let name = p.name().to_str().unwrap_or("").to_string();
                if name.len() > 30 {
                    format!("{}...", &name[..27])
                } else {
                    name
                }
            })
            .unwrap_or_else(|| "No process found".to_string());

        Box::new(Module::new(
            HighCpuModule { process_info },
            Some(Icon::Manual("󰑓")),
            Style {
                fg: Color::Any("color61"),
                bg: Color::Reset,
                bold: true,
            },
        ))
    }
}
