pub const DEFAULT_CHARSET: &str = " .:-=+*#%@";

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_charset_not_empty() {
        assert!(!DEFAULT_CHARSET.is_empty());
    }

    #[test]
    fn test_charset_no_duplicates() {
        let chars: Vec<char> = DEFAULT_CHARSET.chars().collect();
        let unique: HashSet<char> = DEFAULT_CHARSET.chars().collect();
        assert_eq!(chars.len(), unique.len());
    }

    #[test]
    fn test_charset_maps_dark_to_light() {
        let chars: Vec<char> = DEFAULT_CHARSET.chars().collect();
        assert_eq!(chars[0], ' ');
        assert_eq!(*chars.last().unwrap(), '@');
    }
}
