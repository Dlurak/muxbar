use std::fmt;

use crate::{
    colors::{Color, Style},
    icons::Icon,
};

use super::ToModule;

pub struct Warning<T: fmt::Display>(T);

impl<T: fmt::Display> Warning<T> {
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

impl Warning<&str> {
    pub fn from_battery(battery: &super::Battery, threshold: u8) -> Option<Self> {
        (battery.percentage < threshold && !battery.is_charging).then_some(Warning("LOW BATTERY"))
    }
}

impl<T: fmt::Display> fmt::Display for Warning<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  {}  ", self.0)
    }
}

impl<T: fmt::Display> ToModule for Warning<T> {
    fn style(&self) -> Style {
        Style {
            bold: true,
            fg: Color::Black,
            bg: Color::Red,
        }
    }
    fn icon(&self) -> Option<Icon> {
        None
    }
}
