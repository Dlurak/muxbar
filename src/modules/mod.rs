pub mod styled;
use chrono::{DateTime, Local};

// Those are only constructed in config.rs
#[allow(dead_code)]
pub enum Module {
    Manual(&'static str),
    Time(&'static str),
}

impl Module {
    fn display(self) -> String {
        match self {
            Module::Manual(s) => String::from(s),
            Module::Time(format) => {
                let now: DateTime<Local> = Local::now();

                now.format(format).to_string()
            }
        }
    }
}
