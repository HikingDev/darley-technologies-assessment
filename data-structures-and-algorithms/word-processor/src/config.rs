//! Configuration for word processor behavior

/// Configuration for word-processing behavior.
///
/// This struct holds settings that control how text is processed into words,
/// including case sensitivity, punctuation handling, and capacity planning.
#[derive(Debug, Clone)]
pub struct WordProcessorConfig {
    /// Whether to perform case-sensitive matching. Default: true.
    pub case_sensitive: bool,

    /// Whether to include numbers in the word processing. Default: false.
    pub include_numbers: bool,

    /// A custom regex pattern
    pub custom_pattern: Option<String>,

    /// If true, strip punctuation from the ends of tokens. Default: true.
    pub strip_punctuation: bool,

    /// Whether to skip stop words. Default: false.
    pub skip_stop_words: bool,

    /// A factor to compute the hash table capacity:
    /// capacity = (word_count * capacity_factor).ceil().
    /// Default = 1.5
    pub capacity_factor: f32,
}

impl Default for WordProcessorConfig {
    fn default() -> Self {
        Self {
            case_sensitive: true,
            include_numbers: false,
            custom_pattern: None,
            strip_punctuation: true,
            skip_stop_words: false,
            capacity_factor: 1.5,
        }
    }
}

impl WordProcessorConfig {
    // Builder pattern methods for easier configuration

    /// Set case sensitivity
    pub fn case_sensitive(mut self, value: bool) -> Self {
        self.case_sensitive = value;
        self
    }

    /// Set whether to include numbers
    pub fn include_numbers(mut self, value: bool) -> Self {
        self.include_numbers = value;
        self
    }

    /// Set a custom pattern for word matching
    pub fn custom_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.custom_pattern = Some(pattern.into());
        self
    }

    /// Set whether to strip punctuation
    pub fn strip_punctuation(mut self, value: bool) -> Self {
        self.strip_punctuation = value;
        self
    }

    /// Set whether to skip stop words
    pub fn skip_stop_words(mut self, value: bool) -> Self {
        self.skip_stop_words = value;
        self
    }

    /// Set the capacity factor for hash table sizing
    pub fn capacity_factor(mut self, value: f32) -> Self {
        self.capacity_factor = value;
        self
    }
}
