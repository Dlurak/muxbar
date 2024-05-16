pub fn conditional_insert<T>(something: T, enabled: bool) -> Option<T> {
    if enabled {
        Some(something)
    } else {
        None
    }
}
