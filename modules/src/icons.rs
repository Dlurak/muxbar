use std::fmt;

// Those are only constructed in config.rs
#[derive(Clone, Copy)]
pub enum Icon {
    Manual(char),
    Time,
    Calendar,
    Hyprland,
    I3,
    Nix,
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

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manual(s) => write!(f, "{}", s),
            Self::Time => write!(f, ""),
            Self::Calendar => write!(f, ""),
            Self::Hyprland => write!(f, ""),
            Self::I3 => write!(f, ""),
            Self::Nix => write!(f, "󱄅"),
            Self::DetailTux => write!(f, ""),
            Self::SimpleTux => write!(f, "󰌽"),
            Self::Battery(pec) => match pec {
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
            Self::BatteryCharging(pec) => match pec {
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
            Self::DoubleServer => write!(f, ""),
            Self::TripleServer => write!(f, "󰒋"),
            Self::Cpu => write!(f, ""),
            Self::Tmux => write!(f, ""),
        }
    }
}
