use crate::utils::system::battery::BatteryInformation;
use std::fmt;

// Those are only constructed in config.rs
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Icon {
    Manual(char),
    Time,
    Hyprland,
    I3,
    Arch,
    DetailTux,
    SimpleTux,
    Battery(u8),
    BatteryCharging(u8),
    DoubleServer,
    TripleServer,
    Cpu,
    Tmux,
}

impl From<char> for Icon {
    fn from(value: char) -> Self {
        Self::Manual(value)
    }
}

impl Icon {
    pub fn new_battery(info: BatteryInformation) -> Self {
        let perc = info.percentages;
        let charging = info.is_charging;

        if charging {
            Icon::BatteryCharging(perc)
        } else {
            Icon::Battery(perc)
        }
    }
}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Icon::Manual(s) => write!(f, "{}", s),
            Icon::Time => write!(f, ""),
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
        }
    }
}
