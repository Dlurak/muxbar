use std::fmt;

use crate::modules::battery::Battery;

/// Enum representing various icons used in the application
///
/// Icons can be static strings, battery levels, or other predefined symbols.
#[derive(Clone, Copy, Default)]
pub enum Icon {
    /// Default empty icon
    #[default]
    Empty,
    /// Manually specified icon
    Manual(&'static str),
    /// Time icon
    Time,
    /// Date icon
    Date,
    /// Hyprland icon
    Hyprland,
    /// I3 window manager icon
    I3,
    /// Arch Linux icon
    Arch,
    /// Detailed Tux (Linux mascot) icon
    DetailTux,
    /// Simple Tux (Linux mascot) icon
    SimpleTux,
    /// Battery level icon with percentage
    Battery(u8),
    /// Battery charging icon with percentage
    BatteryCharging(u8),
    /// Double server icon
    DoubleServer,
    /// Triple server icon
    TripleServer,
    /// CPU icon
    Cpu,
    /// Tmux icon
    Tmux,
    /// Nvidia
    Nvidia,
}

impl Icon {
    /// Creates a new battery icon based on battery information
    ///
    /// # Arguments
    /// * `info` - Reference to a Battery struct containing battery information
    ///
    /// # Returns
    /// An Option containing either a Battery or BatteryCharging icon
    pub fn new_battery(info: &Battery) -> Option<Self> {
        let perc = info.percentages;
        let charging = info.is_charging;

        if charging {
            Some(Icon::BatteryCharging(perc))
        } else {
            Some(Icon::Battery(perc))
        }
    }
}

impl fmt::Display for Icon {
    /// Formats the icon for display
    ///
    /// # Arguments
    /// * `f` - Formatter to write the formatted string
    ///
    /// # Returns
    /// A Result indicating success or failure
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Icon::Empty => write!(f, ""),
            Icon::Manual(s) => write!(f, "{}", s),
            Icon::Time => write!(f, ""),
            Icon::Date => write!(f, ""),
            Icon::Hyprland => write!(f, ""),
            Icon::I3 => write!(f, ""),
            Icon::Arch => write!(f, ""),
            Icon::DetailTux => write!(f, ""),
            Icon::SimpleTux => write!(f, "󰌽"),
            Icon::Battery(pec) => match pec {
                0..=5 => write!(f, "󰂎"),
                4..=15 => write!(f, "󰁺"),
                14..=25 => write!(f, "󰁻"),
                24..=35 => write!(f, "󰁼"),
                34..=45 => write!(f, "󰁽"),
                44..=55 => write!(f, "󰁾"),
                54..=65 => write!(f, "󰁿"),
                64..=75 => write!(f, "󰂀"),
                74..=85 => write!(f, "󰂁"),
                84..=95 => write!(f, "󰂂"),
                94..=100 => write!(f, "󰁹"),
                _ => write!(f, ""),
            },
            Icon::BatteryCharging(pec) => match pec {
                0..=5 => write!(f, "󰢟"),
                4..=15 => write!(f, "󰢜"),
                14..=25 => write!(f, "󰂆"),
                24..=35 => write!(f, "󰂇"),
                34..=45 => write!(f, "󰂈"),
                44..=55 => write!(f, "󰢝"),
                54..=65 => write!(f, "󰂉"),
                64..=75 => write!(f, "󰢞"),
                74..=85 => write!(f, "󰂊"),
                84..=95 => write!(f, "󰂋"),
                94..=100 => write!(f, "󰂅"),
                _ => write!(f, ""),
            },
            Icon::DoubleServer => write!(f, ""),
            Icon::TripleServer => write!(f, ""),
            Icon::Cpu => write!(f, ""),
            Icon::Tmux => write!(f, ""),
            Icon::Nvidia => write!(f, "󱎴"),
        }
    }
}
