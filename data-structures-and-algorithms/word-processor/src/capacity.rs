//! # Capacity Estimation Module
//!
//! This module estimates an appropriate capacity for the hash table needed to store words from text.
//!
//! ## Assignment Context
//!
//! The assignment requires implementing a *fixed-size* open addressing hash table using linear
//! probing for collision resolution, to store words from a Project Gutenberg book as keys.
//! Since the hash table has a fixed size, we need to determine an appropriate capacity upfront.
//!
//! ## Design Rationale and Trade-offs
//!
//! When designing the capacity estimation for the fixed-size hash table, I considered several approaches:
//!
//! 1. **Fixed capacity:** The simplest approach would be to use a fixed, large capacity for all texts.
//!    - Pros: Simple, no calculation needed
//!    - Cons: Inefficient for small texts, potentially insufficient for very large texts
//!
//! 2. **Character-based heuristic:** Estimate based on total character count divided by average word length.
//!    - Pros: Very fast, doesn't require parsing
//!    - Cons: Inaccurate because it doesn't account for actual word distribution
//!
//! 3. **Full text analysis:** Parse the entire text and count unique words.
//!    - Pros: Most accurate estimate based on actual content
//!    - Cons: Can be slow for very large texts
//!
//! 4. **Sampling-based estimation:** Analyze a representative sample of the text.
//!    - Pros: Balance of speed and accuracy for large texts
//!    - Cons: Less accurate than full analysis, introduces sampling complexity
//!
//! For this assignment, I implemented both the full analysis approach (for accuracy) and a
//! sampling-based approach (for potential scalability with larger texts). For the specific
//! Project Gutenberg text in the assignment, the full analysis approach is most appropriate
//! since the text is of manageable size and accuracy is important for a fixed-size hash table.

use crate::config::WordProcessorConfig;
use crate::error::{CapacityError, WordProcessorError};
use crate::parser;
use std::collections::HashSet;

/// Estimation methods for calculating hash table capacity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstimationMethod {
    /// Process the entire text for accurate capacity estimation
    FullAnalysis,

    /// Process only a sample of the text for faster capacity estimation
    Sampling(usize), // Sample size in characters
}

impl Default for EstimationMethod {
    fn default() -> Self {
        // Default to full analysis as it's most accurate
        Self::FullAnalysis
    }
}

/// Estimates the capacity needed for a hash table based on text content.
///
/// # Arguments
/// * `text` - The text to analyze
/// * `config` - Word processor configuration that affects word extraction
/// * `method` - The estimation method to use (full or sampling-based)
///
/// # Returns
/// The estimated capacity needed for a hash table to store all unique words,
/// or an error if estimation fails
///
/// # Errors
/// Returns `CapacityError::EmptyText` if the input text is empty
/// Returns `CapacityError::InvalidSampleSize` if a sampling method is used with size 0
/// Returns `CapacityError::InvalidCapacityFactor` if the capacity factor in config is <= 0
pub fn estimate_capacity(
    text: &str,
    config: &WordProcessorConfig,
    method: EstimationMethod,
) -> Result<usize, WordProcessorError> {
    // Validate inputs
    if text.is_empty() {
        return Err(CapacityError::EmptyText.into());
    }

    if config.capacity_factor <= 0.0 {
        return Err(CapacityError::InvalidCapacityFactor(config.capacity_factor).into());
    }

    if let EstimationMethod::Sampling(size) = method {
        if size == 0 {
            return Err(CapacityError::InvalidSampleSize(size).into());
        }
    }

    // Perform estimation using the selected method
    match method {
        EstimationMethod::FullAnalysis => Ok(estimate_capacity_full(text, config)),
        EstimationMethod::Sampling(sample_size) => {
            Ok(estimate_capacity_sample(text, config, sample_size))
        }
    }
}

/// Estimates capacity by analyzing the full text
fn estimate_capacity_full(text: &str, config: &WordProcessorConfig) -> usize {
    // Parse the entire text to get actual words according to config
    let words = parser::parse_text(text, config);

    // Count unique words
    let unique_words = count_unique_words(&words);

    // Apply the capacity factor from config
    let capacity = (unique_words as f32 * config.capacity_factor).ceil() as usize;

    // Ensure we return at least 1
    capacity.max(1)
}

/// Estimates capacity by analyzing a sample of the text
fn estimate_capacity_sample(text: &str, config: &WordProcessorConfig, sample_size: usize) -> usize {
    // Get a representative sample of the text
    let sample = get_text_sample(text, sample_size);

    // Parse the sample to get words
    let sample_words = parser::parse_text(&sample, config);

    // Count unique words in the sample
    let unique_words_in_sample = count_unique_words(&sample_words);

    // If sample has no words, return minimum capacity
    if sample_words.is_empty() {
        return 1;
    }

    // Calculate unique-to-total word ratio in the sample
    let unique_ratio = unique_words_in_sample as f32 / sample_words.len() as f32;

    // Estimate total words in the full text based on character counts
    let estimated_word_count =
        (text.len() as f32 / sample.len() as f32) * sample_words.len() as f32;

    // Estimate unique words in the full text
    let estimated_unique_words = estimated_word_count * unique_ratio;

    // Apply the capacity factor from config
    let capacity = (estimated_unique_words * config.capacity_factor).ceil() as usize;

    // Ensure we return at least 1
    capacity.max(1)
}

/// Takes a sample of the given text for analysis
fn get_text_sample(text: &str, sample_size: usize) -> String {
    if text.len() <= sample_size {
        // If text is smaller than sample size, use the whole text
        text.to_string()
    } else {
        // Take a sample from the middle of the text, which is often more representative
        // than the beginning (which may contain titles, etc.)
        let start = (text.len() - sample_size) / 2;
        let end = start + sample_size;

        // Make sure we don't split in the middle of a Unicode character
        let mut safe_start = start;
        while safe_start < text.len() && !text.is_char_boundary(safe_start) {
            safe_start += 1;
        }

        let mut safe_end = end;
        while safe_end < text.len() && !text.is_char_boundary(safe_end) {
            safe_end += 1;
        }

        if safe_end > text.len() {
            safe_end = text.len();
        }

        text[safe_start..safe_end].to_string()
    }
}

/// Count unique words in a list of words
fn count_unique_words(words: &[String]) -> usize {
    let mut seen = HashSet::new();
    for word in words {
        seen.insert(word);
    }
    seen.len()
}

/// Convenience function that uses full analysis estimation method (default)
///
/// This function provides a simpler API for the common case where
/// full text analysis is desired.
#[allow(dead_code)]
pub fn estimate_capacity_default(
    text: &str,
    config: &WordProcessorConfig,
) -> Result<usize, WordProcessorError> {
    estimate_capacity(text, config, EstimationMethod::FullAnalysis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_capacity_estimation() -> Result<(), WordProcessorError> {
        let text = "The quick brown fox jumps over the lazy dog. The fox is quick.";
        let config = WordProcessorConfig::default();

        let capacity = estimate_capacity(text, &config, EstimationMethod::FullAnalysis)?;

        // Should be enough to hold all unique words with some buffer
        assert!(capacity >= 8); // There are 8 unique words
        Ok(())
    }

    #[test]
    fn test_sampling_capacity_estimation() -> Result<(), WordProcessorError> {
        let text = "The quick brown fox jumps over the lazy dog. The fox is quick.";
        let config = WordProcessorConfig::default();

        let capacity = estimate_capacity(text, &config, EstimationMethod::Sampling(20))?;

        // Even with sampling, should give reasonable estimate
        assert!(capacity > 0);
        Ok(())
    }

    #[test]
    fn test_capacity_factor() -> Result<(), WordProcessorError> {
        let text = "The quick brown fox jumps over the lazy dog.";

        // Default factor (1.5)
        let config1 = WordProcessorConfig::default();
        let capacity1 = estimate_capacity(text, &config1, EstimationMethod::FullAnalysis)?;

        // Double the factor
        let mut config2 = WordProcessorConfig::default();
        config2.capacity_factor = 3.0;
        let capacity2 = estimate_capacity(text, &config2, EstimationMethod::FullAnalysis)?;

        // The ratio should be approximately 2x
        assert!(capacity2 > capacity1);
        assert!((capacity2 as f32 / capacity1 as f32 - 2.0).abs() < 0.5);
        Ok(())
    }

    #[test]
    fn test_empty_text() {
        let text = "";
        let config = WordProcessorConfig::default();

        let result = estimate_capacity(text, &config, EstimationMethod::FullAnalysis);

        assert!(result.is_err());
        if let Err(WordProcessorError::Capacity(CapacityError::EmptyText)) = result {
            // Expected error
        } else {
            panic!("Expected EmptyText error");
        }
    }
}
