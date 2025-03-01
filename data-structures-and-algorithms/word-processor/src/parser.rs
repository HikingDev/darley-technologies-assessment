//! This module handles parsing text into words based on configuration settings.

use crate::config::WordProcessorConfig;

/// Parses text into words according to the provided configuration.
///
/// # Arguments
/// * `text` - The text to parse
/// * `config` - Configuration that controls parsing behavior
///
/// # Returns
/// A vector of parsed words
pub fn parse_text(text: &str, config: &WordProcessorConfig) -> Vec<String> {
    // Split by whitespace as the basic tokenization strategy
    let tokens = text.split_whitespace();

    // Process each token according to config
    let mut result = Vec::new();
    for token in tokens {
        // Process the token based on configuration
        let processed = process_token(token, config);

        // Add to results if we have a valid token
        if let Some(word) = processed {
            // Skip stop words if configured
            if !config.skip_stop_words || !is_stop_word(&word) {
                result.push(word);
            }
        }
    }

    result
}

/// Process a single token according to configuration
fn process_token(token: &str, config: &WordProcessorConfig) -> Option<String> {
    // Strip punctuation if configured
    let token = if config.strip_punctuation {
        strip_punctuation(token)
    } else {
        token.to_string()
    };

    // Skip empty tokens
    if token.is_empty() {
        return None;
    }

    // Apply token filtering based on configuration
    if !config.include_numbers && token.chars().all(|c| c.is_numeric()) {
        return None;
    }

    // Apply case sensitivity
    let token = if !config.case_sensitive {
        token.to_lowercase()
    } else {
        token
    };

    Some(token)
}

/// Strips punctuation from the beginning and end of a token
fn strip_punctuation(token: &str) -> String {
    let mut start = 0;
    let mut end = token.len();

    let chars: Vec<char> = token.chars().collect();

    // Skip leading punctuation
    while start < end && is_punctuation(chars[start]) {
        start += 1;
    }

    // Skip trailing punctuation
    while end > start && is_punctuation(chars[end - 1]) {
        end -= 1;
    }

    // Extract the substring without punctuation
    if start >= end {
        String::new()
    } else {
        chars[start..end].iter().collect()
    }
}

/// Check if a character is punctuation
fn is_punctuation(c: char) -> bool {
    c.is_ascii_punctuation()
}

/// Check if a word is a common stop word
fn is_stop_word(word: &str) -> bool {
    // Common English stop words
    static STOP_WORDS: [&str; 25] = [
        "the", "and", "a", "an", "in", "on", "at", "of", "to", "for", "with", "by", "as", "is",
        "are", "was", "were", "be", "been", "being", "this", "that", "these", "those", "it",
    ];

    let lower_word = word.to_lowercase();
    STOP_WORDS.contains(&lower_word.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WordProcessorConfig;

    #[test]
    fn test_basic_parsing() {
        let text = "Hello, World! This is a test.";
        let config = WordProcessorConfig::default();

        let words = parse_text(text, &config);
        assert_eq!(words, vec!["Hello", "World", "This", "is", "a", "test"]);
    }

    #[test]
    fn test_case_sensitivity() {
        let text = "Hello World";

        let mut config = WordProcessorConfig::default();
        config.case_sensitive = false;

        let words = parse_text(text, &config);
        assert_eq!(words, vec!["hello", "world"]);
    }
}
