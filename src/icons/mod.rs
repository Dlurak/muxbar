use std::fmt;

// Those are only constructed in config.rs
#[allow(dead_code)]
pub enum Icon {
    Manual(&'static str),
    Time,
    Hyprland,
    I3,
    Arch,
    DetailTux,
    SimpleTux,
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
        }
    }
}
