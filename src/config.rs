use crate::colors::{Color, Style};
use crate::modules::{styled::StyledModule, Module};

pub fn get_modules() -> Vec<StyledModule> {
    vec![StyledModule::new(
        Module::Time("%Y-%m-%d %H:%M:%S"),
        Style::new(Color::Green, Color::Reset),
    )]
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
