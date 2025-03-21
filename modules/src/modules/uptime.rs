use super::ToModule;
use crate::{
    colors::{Color, Style},
    icons::Icon,
    pretty_duration::PrettyDuration,
};
use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use sysinfo::System;

pub struct Uptime {
    boot_time: SystemTime,
    uptime: PrettyDuration,
}

impl Uptime {
    pub fn new() -> Option<Self> {
        let boot_time = UNIX_EPOCH + Duration::from_secs(System::boot_time());
        let uptime = boot_time.elapsed().ok()?;
        let uptime = PrettyDuration::new(uptime);
        Some(Self { boot_time, uptime })
    }
}

impl fmt::Display for Uptime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uptime)
    }
}

impl ToModule for Uptime {
    fn icon(&self) -> Option<Icon> {
        Some(Icon::Time)
    }
    fn style(&self) -> Style {
        Style::new_with_fg(Color::Yellow)
    }
    fn update(&mut self) {
        if let Ok(uptime) = self.boot_time.elapsed() {
            self.uptime = PrettyDuration::new(uptime)
        }
    }
    fn next_render_time(&self) -> Option<Duration> {
        // TODO: Calculate the actual duration
        let duration = Duration::from_secs(5);
        Some(duration)
    }
}
