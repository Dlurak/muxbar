use battery::{Manager, State};
use std::error::Error;
use std::fmt;

use crate::colors::{Color, Style};
use crate::icons::Icon;

use super::Module;

/// Represents the battery status including charge level and charging state
#[derive(Copy, Clone, Default)]
pub struct Battery {
    /// Battery charge percentage (0-100)
    pub percentages: u8,
    /// Indicates whether the battery is currently charging
    pub is_charging: bool,
}

impl Battery {
    /// Creates a new Battery instance with current system battery information
    ///
    /// # Returns
    /// - `Ok(Battery)` containing current battery status
    /// - `Err` if battery information cannot be retrieved
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let manager = Manager::new()?;
        let mut batteries = manager.batteries()?;
        if let Some(Ok(battery)) = batteries.next() {
            let percentages = battery
                .state_of_charge()
                .get::<battery::units::ratio::percent>() as u8;

            let is_charging = battery.state() != State::Discharging;

            Ok(Self {
                percentages,
                is_charging,
            })
        } else {
            Ok(Self {
                percentages: 0,
                is_charging: false,
            })
        }
    }
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.percentages == 0 {
            write!(f, "")
        } else {
            let battery_text = format!("{:>3}%", self.percentages);
            write!(f, "{:>3}", battery_text)
        }
    }
}

impl Battery {
    /// Returns a standard battery module with default green styling
    ///
    /// # Returns
    /// A boxed Module containing battery information with default styling
    pub fn get_standard() -> Box<Module<Battery>> {
        let battery = match Battery::new() {
            Ok(battery) => battery,
            Err(_) => return Box::new(Module::default()),
        };

        let battery_icon = Icon::new_battery(&battery);
        Box::new(Module::new(
            battery,
            battery_icon,
            Style {
                fg: Color::Green,
                bg: Color::Reset,
                bold: false,
            },
        ))
    }
    /// Returns a battery module with warning styling when battery is low
    ///
    /// Creates a module that shows a warning message with red background
    /// when battery is below 20% and not charging
    ///
    /// # Returns
    /// A boxed Module containing battery information with conditional styling
    pub fn get_with_warning(bg: Option<Color>) -> Box<Module<String>> {
        let battery = match Battery::new() {
            Ok(battery) => battery,
            Err(_) => return Box::new(Module::default()),
        };
        let battery_icon = Icon::new_battery(&battery);
        if battery.percentages == 0 {
            Box::new(Module::default())
        } else if battery.percentages > 20 || battery.is_charging {
            Box::new(Module::new(
                battery.to_string(),
                battery_icon,
                Style {
                    fg: Color::Green,
                    bg: bg.unwrap_or(Color::Reset),
                    bold: false,
                },
            ))
        } else {
            Box::new(Module::new(
                format!(" {}{} ", battery, " LOW BATTERY"),
                battery_icon,
                Style {
                    fg: Color::Black,
                    bg: Color::Red,
                    bold: true,
                },
            ))
        }
    }
    /// Returns a battery module with custom styling
    ///
    /// # Arguments
    /// * `style` - The custom Style to apply to the battery module
    ///
    /// # Returns
    /// A boxed Module containing battery information with the specified style
    pub fn get_styled(style: Style) -> Box<Module<Battery>> {
        let battery = match Battery::new() {
            Ok(battery) => battery,
            Err(_) => return Box::new(Module::default()),
        };
        let battery_icon = Icon::new_battery(&battery);
        Box::new(Module::new(battery, battery_icon, style))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_information_default() {
        let battery_info = Battery::default();
        assert_eq!(battery_info.percentages, 0);
        assert!(!battery_info.is_charging);
    }
}
