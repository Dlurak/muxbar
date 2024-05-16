pub mod styled;
use crate::utils::battery;
use chrono::{DateTime, Local};

// Those are only constructed in config.rs
#[allow(dead_code)]
pub enum Module {
    Manual(&'static str),
    Time(&'static str),
    Battery,
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
        }
    }
}
