#[cfg(windows)]
use glob::MatchOptions;

#[allow(dead_code)]
pub fn contains_glob_chars(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

#[allow(dead_code)]
#[cfg(windows)]
pub fn get_glob_options() -> MatchOptions {
    MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    }
}

#[allow(dead_code)]
#[cfg(not(windows))]
pub fn get_glob_options() -> MatchOptions {
    MatchOptions::default()
}
