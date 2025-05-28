pub fn to_ut8(content: &str) -> String {
    content
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || c.is_ascii_punctuation())
        .collect()
}
