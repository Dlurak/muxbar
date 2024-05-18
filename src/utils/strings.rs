use std::fmt::{Display, Formatter, Result};
use std::time::Duration;

pub fn round<T, U>(num: T, decimal_places: U) -> String
where
    T: Display,
    U: Into<usize>,
{
    format!("{:.1$}%", num, decimal_places.into())
}

pub struct PrettyDuration {
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl PrettyDuration {
    pub fn new(duration: Duration) -> Self {
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

    fn to_parts(&self) -> Vec<String> {
        [
            (self.days, "D"),
            (self.hours, "H"),
            (self.minutes, "M"),
            (self.seconds, "S"),
        ]
        .iter()
        .filter_map(|&(value, unit)| {
            if value > 0 {
                Some(format!("{}{}", value, unit))
            } else {
                None
            }
        })
        .collect()
    }
}

impl Display for PrettyDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let parts = self.to_parts();
        let output = match parts.len() {
            0 => "0 S".to_string(),
            1 => parts[0].clone(),
            _ => format!("{} {}", parts[0], parts[1]),
        };

        write!(f, "{}", output)
    }
}
