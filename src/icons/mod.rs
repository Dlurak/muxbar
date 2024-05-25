use std::fmt;

// Those are only constructed in config.rs
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Icon {
    Manual(&'static str),
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
}

impl Icon {
    pub fn new_battery(percentages: Result<u8, ()>, is_charging: Result<bool, ()>) -> Option<Self> {
        let perc = percentages.ok()?;
        let charging = is_charging.ok()?;

        if charging {
            Some(Icon::BatteryCharging(perc))
        } else {
            Some(Icon::Battery(perc))
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
        }
    }
}
