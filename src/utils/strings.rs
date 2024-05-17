use std::fmt::Display;

pub fn round<T, U>(num: T, decimal_places: U) -> String
where
    T: Display,
    U: Into<usize>,
{
    format!("{:.1$}%", num, decimal_places.into())
}
