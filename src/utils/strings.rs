use std::fmt::{Display, Formatter, Result};
use std::time::Duration;

/// Formats a number as a percentage string with specified decimal places and minimum width
///
/// # Arguments
/// * `num` - The number to format
/// * `decimal_places` - Number of decimal places to show
/// * `minimum_digits` - Minimum width of the output string
///
/// # Returns
/// A formatted percentage string
pub fn round<T, U>(num: T, decimal_places: U, minimum_digits: U) -> String
where
    T: Display,
    U: Into<usize>,
{
    format!(
        "{:<2$.1$}%",
        num,
        decimal_places.into(),
        minimum_digits.into()
    )
}

/// A structure that represents duration in a human-readable format
pub struct PrettyDuration {
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl PrettyDuration {
    /// Creates a new PrettyDuration from a std::time::Duration
    ///
    /// # Arguments
    /// * `duration` - The Duration to convert
    ///
    /// # Returns
    /// A new PrettyDuration instance with calculated components
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

    /// Converts the duration components into a vector of formatted strings
    ///
    /// # Returns
    /// `Vec<String>` containing non-zero components with their units
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

/// Implements Display trait for PrettyDuration
/// Shows up to two largest non-zero units, or "0 S" if duration is zero
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// Tests creation of PrettyDuration with all components
    #[test]
    fn test_pretty_duration_new() {
        let duration = Duration::new(90061, 0); // 1 day, 1 hour, 1 minute, 1 second
        let pretty_duration = PrettyDuration::new(duration);

        assert_eq!(pretty_duration.days, 1);
        assert_eq!(pretty_duration.hours, 1);
        assert_eq!(pretty_duration.minutes, 1);
        assert_eq!(pretty_duration.seconds, 1);
    }

    /// Tests string formatting for various duration lengths
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

        assert_eq!(pretty_duration.to_string(), "0 S");
    }
}
