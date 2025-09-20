pub fn validate_fone(fone: &str) -> bool {
    let fone_cleaned: String = fone.chars().filter(char::is_ascii_digit).collect();
    let len = fone_cleaned.len();
    (10..=15).contains(&len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_fone_valid() {
        assert!(validate_fone("1234567890")); // 10 digits
        assert!(validate_fone("12345678901")); // 11 digits
        assert!(validate_fone("123456789012345")); // 15 digits
        assert!(validate_fone("+1 (555) 123-4567")); // With formatting
        assert!(validate_fone("(11) 99999-9999")); // Brazilian format
    }

    #[test]
    fn test_validate_fone_too_short() {
        assert!(!validate_fone("123456789")); // 9 digits
        assert!(!validate_fone("12345"));
        assert!(!validate_fone(""));
    }

    #[test]
    fn test_validate_fone_too_long() {
        assert!(!validate_fone("1234567890123456")); // 16 digits
        assert!(!validate_fone("12345678901234567890"));
    }

    #[test]
    fn test_validate_fone_no_digits() {
        assert!(!validate_fone("abc-def-ghij"));
        assert!(!validate_fone("()- "));
    }

    #[test]
    fn test_validate_fone_mixed_content() {
        assert!(validate_fone("abc123def456ghi7890")); // 13 digits mixed with letters
        assert!(!validate_fone("abc123def45")); // Only 8 digits
    }
}
