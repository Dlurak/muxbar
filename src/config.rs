use std::fmt::Display;

use crate::colors::{Color, Style};
use crate::icons::Icon;
use crate::modules::battery::Battery;
use crate::modules::datetime::DateTime;
use crate::modules::high_cpu::HighCpu;
use crate::modules::nvidia::Nvidia;
use crate::modules::systemstats::{Cpu, Memory, Swap};
use crate::modules::tmux::TmuxContent;
use crate::modules::Module;

/// Should return a vector of display modules for the status bar
///
/// Each module implements the Display trait
/// The modules are arranged in the order they will appear in the status bar.
pub fn get_modules() -> Vec<Box<dyn Display + Send>> {
    vec![
        HighCpu::new(None, None),
        // System CPU usage module with custom styling
        Box::new(Module::new(
            Cpu {
                minimum_digits: 2,
                decimal_places: 0,
            },
            Some(Icon::Cpu),
            Style {
                fg: Color::Cyan,
                bg: Color::Reset,
                bold: false,
            },
        )),
        // System Memory usage module with custom styling
        Box::new(Module::new(
            Memory::default(),
            Some(Icon::DoubleServer),
            Style {
                fg: Color::Yellow,
                bg: Color::Reset,
                bold: false,
            },
        )),
        // System Memory usage module with custom styling
        Box::new(Module::new(
            Swap::default(),
            Some(Icon::TripleServer),
            Style {
                fg: Color::Red,
                bg: Color::Reset,
                bold: false,
            },
        )),
        Nvidia::new(true),
        // Battery module with warning indicators
        Battery::get_with_warning(),
        // Tmux session information modules
        // TmuxContent::SessionName.get_standard(),
        // Box::new(Module::new(
        //     TmuxContent::Hostname,
        //     Some(Icon::DoubleServer),
        //     Style {
        //         fg: Color::White,
        //         bg: Color::Reset,
        //         bold: false,
        //     },
        // )),
        TmuxContent::PaneIndex.get_standard(),
        // Use a preset of the DateTime module
        DateTime::date(),
        // Configure DateTime module with custom format and style
        Box::new(Module::new(
            DateTime("%H:%M"),
            Some(Icon::Time),
            Style {
                fg: Color::Magenta,
                bg: Color::Reset,
                bold: false,
            },
        )),
    ]
}

/// Returns the string to be displayed before all modules
pub fn pre_modules() -> &'static str {
    ""
}

/// Returns the string to be displayed after all modules
pub fn post_modules() -> &'static str {
    " "
}

/// Returns the string to be displayed between modules
pub fn between_modules() -> &'static str {
    "  "
}
