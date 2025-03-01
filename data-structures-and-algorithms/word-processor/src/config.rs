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
