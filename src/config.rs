use modules::modules::{Battery, Cpu, DateTime, Memory, TmuxContent, Warning};
use modules::{Color, Icon, Module, Style};
use std::fmt;

pub fn get_modules() -> Vec<Module<Box<dyn fmt::Display + Sync>>> {
    let battery = Battery::try_new(2).ok().flatten();
    [
        Some(
            Module::from(DateTime::date())
                .set_icon(Some(Icon::Calendar))
                .set_style(Style::new_with_fg(Color::Yellow)),
        ),
        Some(DateTime::new("%H:%M:%S").into()),
        Some(Cpu::new(2, 2).into()),
        battery.map(|x| x.into()),
        Some(Memory::new(2, 2).into()),
        Some(TmuxContent::SessionName.into()),
        battery
            .and_then(|bat| Warning::from_battery(&bat, 20))
            .map(|x| x.into()),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub const PRE_MODULES: &str = "";
pub const BETWEEN_MODULES: &str = " ";
pub const POST_MODULES: &str = " ";
