pub fn validate_fone(fone: &str) -> bool {
    let fone_cleaned: String = fone.chars().filter(char::is_ascii_digit).collect();
    let len = fone_cleaned.len();
    (10..=15).contains(&len)
}
