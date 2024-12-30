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

        let process_info: Vec<String> = system
            .processes()
            .values()
            .filter(|p| p.cpu_usage() > 0.001)
            // .collect::<Vec<_>>()
            // .max_by_key(|p| (p.cpu_usage() * 100.0) as u32)
            .map(|p| {
                let name = p.name().to_str().unwrap_or("").to_string();
                println!("{:?} {:?}", p.name(), p.cpu_usage());
                if name.len() > 30 {
                    format!("{}...", &name[..27])
                } else {
                    format!("{} {}", name, p.cpu_usage())
                }
            })
            .collect();
        // println!("{}", process_info);
        // .unwrap_or_else(|| "No process found".to_string());

        // #[cfg(debug_assertions)]
        // println!("{:?}", process_info);

        Box::new(Module::new(
            HighCpuModule {
                process_info: "porces".to_string(),
            },
            Some(Icon::Manual("󰑓")),
            Style {
                fg: Color::Any("color61"),
                bg: Color::Reset,
                bold: true,
            },
        ))
    }
}
