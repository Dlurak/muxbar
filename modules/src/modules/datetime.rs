use super::ToModule;
use crate::{
    colors::{Color, Style},
    icons::Icon,
};
use chrono::Local;
use std::{
    fmt,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Copy)]
pub struct DateTime<'a> {
    format: &'a str,
}

impl<'a> DateTime<'a> {
    pub const fn new(format: &'a str) -> Self {
        Self { format }
    }

    pub const fn date() -> Self {
        Self { format: "%d.%m.%Y" }
    }

    pub const fn time() -> Self {
        Self { format: "%H:%M" }
    }
}

impl fmt::Display for DateTime<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Local::now().format(self.format))
    }
}

impl ToModule for DateTime<'_> {
    fn icon(&self) -> Option<Icon> {
        Some(Icon::Time)
    }
    fn style(&self) -> Style {
        Style::new_with_fg(Color::Magenta)
    }
    fn next_render_time(&self) -> Option<Instant> {
        let now_sys = SystemTime::now();
        let now_ins = Instant::now();
        let duration_since_epoch = now_sys.duration_since(UNIX_EPOCH).unwrap();
        let duration_since_epoch = Duration::from_secs(duration_since_epoch.as_secs());

        let now_instant_duration = now_ins.duration_since(now_ins - duration_since_epoch);
        let truncated = now_ins - (now_instant_duration - duration_since_epoch);

        Some(truncated + Duration::from_secs(1))
    }
}
