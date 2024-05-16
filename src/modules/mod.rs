pub mod battery;
pub mod styled;
use chrono::{DateTime, Local};

// Those are only constructed in config.rs
#[allow(dead_code)]
pub enum Module {
    Manual(&'static str),
    Time(&'static str),
    Battery,
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
        }
    }
}
