use crate::colors::{Color, Style};
use crate::icons::Icon;
use crate::modules::{styled::StyledModule, Module};
use crate::utils::conditional_insert::conditional_insert;
use crate::utils::system::battery::battery_status;

pub fn get_modules() -> Vec<StyledModule> {
    let battery_percentage = battery_status();

    let battery_icon = Icon::new_battery(battery_percentage);

    vec![
        Some(StyledModule::new(
            Module::Uptime,
            None,
            Style {
                fg: Color::Magenta,
                bg: Color::Reset,
                bold: false,
            },
        )),
        Some(StyledModule::new(
            Module::Time("%H:%M"),
            Some(Icon::Time),
            Style {
                fg: Color::Magenta,
                bg: Color::Reset,
                bold: false,
            },
        )),
        Some(StyledModule::new(
            Module::Cpu(2),
            Some(Icon::Cpu),
            Style {
                fg: Color::Cyan,
                bg: Color::Reset,
                bold: false,
            },
        )),
        Some(StyledModule::new(
            Module::Memory(2),
            Some(Icon::DoubleServer),
            Style {
                fg: Color::Yellow,
                bg: Color::Reset,
                bold: false,
            },
        )),
        Some(StyledModule::new(
            Module::Battery,
            battery_icon,
            Style {
                fg: Color::Green,
                bg: Color::Reset,
                bold: false,
            },
        )),
        conditional_insert(
            StyledModule::new(
                Module::Manual("  LOW BATTERY  "),
                None,
                Style {
                    fg: Color::Black,
                    bg: Color::Red,
                    bold: true,
                },
            ),
            battery_percentage.unwrap_or(100) < 20,
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub fn pre_modules() -> &'static str {
    ""
}

pub fn post_modules() -> &'static str {
    " "
}

pub fn between_modules() -> &'static str {
    " "
}
