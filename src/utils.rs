pub fn is_alphanumeric(ch: Option<char>) -> bool {
    match ch {
        Some(ch) => ch.is_alphanumeric(),
        None => false,
    }
}
