use crate::{
    colors::{Color, Style},
    icons::Icon,
};
use std::{fmt, time::Duration};

use super::ToModule;

pub enum WarningRerender<'a> {
    Battery {
        manager: battery::Manager,
        threshold: u8,
        hourly_percentage_usage: f32,
    },
    Custom(&'a dyn Fn() -> (bool, Option<Duration>)),
}

pub struct Warning<'a, T: fmt::Display> {
    content: T,
    rerender: WarningRerender<'a>,
}

impl<'a, T: fmt::Display> Warning<'a, T> {
    pub const fn new(content: T, rerender: WarningRerender<'a>) -> Self {
        Self { content, rerender }
    }
}

impl Warning<'_, &str> {
    pub fn new_battery(
        threshold: u8,
        hourly_percentage_usage: f32,
    ) -> Result<Option<Self>, battery::errors::Error> {
        let manager = battery::Manager::new()?;
        Ok(Some(Warning {
            content: "LOW BATTERY",
            rerender: WarningRerender::Battery {
                manager,
                threshold,
                hourly_percentage_usage,
            },
        }))
    }
}

impl<T: fmt::Display> fmt::Display for Warning<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  {}  ", self.content)
    }
}

impl<T: fmt::Display> ToModule for Warning<'_, T> {
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

    fn include(&self) -> bool {
        match &self.rerender {
            WarningRerender::Custom(pred) => pred().0,
            WarningRerender::Battery {
                manager,
                threshold,
                hourly_percentage_usage: _,
            } => {
                let battery = manager
                    .batteries()
                    .ok()
                    .and_then(|mut batteries| batteries.next())
                    .and_then(|b| b.ok());

                battery.is_some_and(|battery| {
                    let percentage = battery
                        .state_of_charge()
                        .get::<battery::units::ratio::percent>()
                        as u8;
                    let is_discharging = battery.state() == battery::State::Discharging;
                    is_discharging && percentage <= *threshold
                })
            }
        }
    }

    fn next_render_time(&self) -> Option<Duration> {
        match &self.rerender {
            WarningRerender::Battery {
                manager,
                threshold,
                hourly_percentage_usage,
            } => {
                let battery = manager.batteries().ok()?.next()?.ok()?;

                let percentage = battery
                    .state_of_charge()
                    .get::<battery::units::ratio::percent>() as u8;
                let is_discharging = battery.state() == battery::State::Discharging;

                let perc_until_warn = percentage.checked_sub(*threshold);
                let duration = perc_until_warn.map_or(Duration::from_secs(4), |perc| {
                    let factor = if is_discharging { 3.0 / 6.0 } else { 3.0 / 4.0 };
                    let hours = (perc as f32) / hourly_percentage_usage * factor;
                    let secs = hours * 60.0 * 60.0;
                    Duration::from_secs_f32(secs).max(Duration::from_secs(4))
                });
                Some(duration)
            }
            WarningRerender::Custom(pred) => pred().1,
        }
    }
}
