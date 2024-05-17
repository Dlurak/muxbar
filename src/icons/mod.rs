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

    DoubleServer,
    TripleServer,
    Cpu,
}

impl Icon {
    pub fn new_battery(percentages: Result<u8, ()>) -> Option<Self> {
        match percentages {
            Ok(p) => Some(Self::Battery(p)),
            Err(_) => None,
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
                0..=20 => write!(f, ""),
                21..=40 => write!(f, ""),
                41..=60 => write!(f, ""),
                61..=80 => write!(f, ""),
                81..=100 => write!(f, ""),
                _ => write!(f, ""),
            },
            Icon::DoubleServer => write!(f, ""),
            Icon::TripleServer => write!(f, ""),
            Icon::Cpu => write!(f, ""),
        }
    }
}
