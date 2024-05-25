pub fn conditional_insert<T>(something: T, enabled: bool) -> Option<T> {
    if enabled {
        Some(something)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_insert_enabled() {
        let value = 42;
        let result = conditional_insert(value, true);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_conditional_insert_disabled() {
        let value = 42;
        let result = conditional_insert(value, false);
        assert_eq!(result, None);
    }
}
