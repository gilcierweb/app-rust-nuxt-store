use unicode_normalization::UnicodeNormalization;

/// Simulates the behavior of ActiveSupport's `String#parameterize` (Rails)
pub fn parameterize(text: &str) -> String {
    text.nfd() // Decomposes characters (e.g., 'á' -> 'a' + '´')
        .filter(|c| c.is_ascii_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
        .split_whitespace() // Treats multiple spaces as one
        .collect::<Vec<_>>()
        .join("-")
}

/// Normalize display name: remove control characters, preserve unicode
pub fn normalize_display_name(name: &str) -> String {
    name.trim()
        .nfc()
        .collect::<String>()
        .chars()
        .filter(|c| {
            c.is_alphanumeric()
                || c.is_whitespace()
                || matches!(c, '.' | ',' | '\'' | '-' | '(' | ')' | '!')
        })
        .collect()
}
