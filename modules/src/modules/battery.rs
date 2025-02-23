use super::ToModule;
use crate::{
    colors::{Color, Style},
    icons::Icon,
};
use battery::{units, Manager, State};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Battery {
    pub percentage: u8,
    pub is_charging: bool,
    minimum_digits: usize,
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0width$}%",
            self.percentage,
            width = self.minimum_digits
        )
    }
}

impl ToModule for Battery {
    fn style(&self) -> Style {
        Style::new_with_fg(Color::Green)
    }

    fn icon(&self) -> Option<Icon> {
        let perc = self.percentage;

        if self.is_charging {
            Some(Icon::BatteryCharging(perc))
        } else {
            Some(Icon::Battery(perc))
        }
    }
}

impl Battery {
    pub fn try_new(minimum_digits: usize) -> Result<Option<Self>, battery::errors::Error> {
        let mut batteries = Manager::new()?.batteries()?;
        let battery = match batteries.next() {
            Some(val) => val,
            None => return Ok(None),
        }?;

        let percentage = battery.state_of_charge().get::<units::ratio::percent>() as u8;

        let is_charging = battery.state() != State::Discharging;

        Ok(Some(Self {
            percentage,
            is_charging,
            minimum_digits,
        }))
    }
}
