use once_cell::sync::Lazy;
use regex::Regex;

static LOWERCASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-z]").unwrap());
static UPPERCASE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-Z]").unwrap());
static DIGIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
static SPECIAL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[@$!%*?&]").unwrap());
static VALID_CHARS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z\d@$!%*?&]{8,}$").unwrap());

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8
        && LOWERCASE_REGEX.is_match(password)
        && UPPERCASE_REGEX.is_match(password)
        && DIGIT_REGEX.is_match(password)
        && SPECIAL_REGEX.is_match(password)
        && VALID_CHARS_REGEX.is_match(password)
}
