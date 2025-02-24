use std::fmt::{self, Display, Formatter, Result};
use std::time::Duration;

pub struct PrettyDuration {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

#[derive(Clone, Copy)]
enum Unit {
    Day,
    Hour,
    Minute,
    Second,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            Self::Day => "D",
            Self::Hour => "H",
            Self::Minute => "M",
            Self::Second => "S",
        };
        write!(f, "{s}")
    }
}

impl PrettyDuration {
    pub const fn new(duration: Duration) -> Self {
        let total_seconds = duration.as_secs();

        let days = total_seconds / (3600 * 24);
        let hours = (total_seconds % (3600 * 24)) / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        Self {
            days,
            hours,
            minutes,
            seconds,
        }
    }

    fn to_parts(&self) -> Vec<(u64, Unit)> {
        vec![
            (self.days, Unit::Day),
            (self.hours, Unit::Hour),
            (self.minutes, Unit::Minute),
            (self.seconds, Unit::Second),
        ]
        .into_iter()
        .filter(|(value, _)| (*value > 0))
        .collect()
    }
}

impl Display for PrettyDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let parts = self.to_parts();
        match parts.len() {
            0 => write!(f, "0S"),
            1 => write!(f, "{}{}", parts[0].0, parts[0].1),
            _ => write!(
                f,
                "{}{} {}{}",
                parts[0].0, parts[0].1, parts[1].0, parts[1].1
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_pretty_duration_new() {
        let duration = Duration::new(90061, 0); // 1 day, 1 hour, 1 minute, 1 second
        let pretty_duration = PrettyDuration::new(duration);

        assert_eq!(pretty_duration.days, 1);
        assert_eq!(pretty_duration.hours, 1);
        assert_eq!(pretty_duration.minutes, 1);
        assert_eq!(pretty_duration.seconds, 1);
    }

    #[test]
    fn test_pretty_duration_display() {
        let duration = Duration::new(90061, 0); // 1 day, 1 hour, 1 minute, 1 second
        let pretty_duration = PrettyDuration::new(duration);

        assert_eq!(pretty_duration.to_string(), "1D 1H");

        let duration = Duration::new(3661, 0); // 1 hour, 1 minute, 1 second
        let pretty_duration = PrettyDuration::new(duration);

        assert_eq!(pretty_duration.to_string(), "1H 1M");

        let duration = Duration::new(61, 0); // 1 minute, 1 second
        let pretty_duration = PrettyDuration::new(duration);

        assert_eq!(pretty_duration.to_string(), "1M 1S");

        let duration = Duration::new(1, 0); // 1 second
        let pretty_duration = PrettyDuration::new(duration);

        assert_eq!(pretty_duration.to_string(), "1S");

        let duration = Duration::new(0, 0); // 0 seconds
        let pretty_duration = PrettyDuration::new(duration);

        assert_eq!(pretty_duration.to_string(), "0S");
    }
}
