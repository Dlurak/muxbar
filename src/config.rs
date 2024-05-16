use crate::colors::{Color, Style};
use crate::icons::Icon;
use crate::modules::{battery::battery_status, styled::StyledModule, Module};

pub fn get_modules() -> Vec<StyledModule> {
    let battery_percentage = battery_status();

    let battery_icon = Icon::new_battery(battery_percentage);

    let mut base = vec![
        StyledModule::new(
            Module::Time("%H:%M"),
            Some(Icon::Time),
            Style::new(Color::Magenta, Color::Reset),
        ),
        StyledModule::new(
            Module::Battery,
            battery_icon,
            Style::new(Color::Green, Color::Reset),
        ),
    ];

    if battery_percentage.unwrap_or(100) < 20 {
        base.push(StyledModule::new(
            Module::Manual("  LOW BATTERY  "),
            None,
            Style::new(Color::Black, Color::Red),
        ));
    }

    base
}

pub fn pre_modules() -> &'static str {
    ""
}

pub fn post_modules() -> &'static str {
    " "
}

pub fn between_modules() -> &'static str {
    "  "
}
