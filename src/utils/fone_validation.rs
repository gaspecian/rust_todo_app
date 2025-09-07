pub fn validate_fone(fone: &str) -> bool {
    let fone_cleaned: String = fone.chars().filter(|c| c.is_digit(10)).collect();
    let len = fone_cleaned.len();
    len >= 10 && len <= 15
}