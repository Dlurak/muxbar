use super::ToModule;
use crate::{
    colors::{Color, Style},
    icons::Icon,
};
use battery::{units, Manager, State};
use std::{fmt, time::Duration};

pub struct Battery {
    percentage: u8,
    is_charging: bool,
    minimum_digits: usize,
    manager: Manager,
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

    fn update(&mut self) {
        let battery = self
            .manager
            .batteries()
            .ok()
            .and_then(|mut batteries| batteries.next())
            .and_then(|b| b.ok());

        if let Some(battery) = battery {
            self.percentage = battery.state_of_charge().get::<units::ratio::percent>() as u8;
            self.is_charging = battery.state() != State::Discharging;
        }
    }

    fn next_render_time(&self) -> Option<Duration> {
        let percentage: f32 = self.percentage.into();
        let secs = percentage.sqrt();
        Some(Duration::from_secs_f32(secs))
    }
}

impl Battery {
    pub fn try_new(minimum_digits: usize) -> Result<Option<Self>, battery::errors::Error> {
        let manager = Manager::new()?;
        let mut batteries = manager.batteries()?;
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
            manager,
        }))
    }
}
