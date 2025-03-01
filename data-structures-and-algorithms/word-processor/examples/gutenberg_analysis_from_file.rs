//! Example that reads "A Tale of Two Cities" from a local file
//! and analyzes its word content using the word processor.

use std::collections::HashMap;
use std::error::Error;
use word_processor::{EstimationMethod, WordProcessorConfig, estimate_capacity, io, parse_text};

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the downloaded Gutenberg book file
    let file_path = "../docs/98-0.txt";

    println!("Reading 'A Tale of Two Cities' from local file...");
    let text = io::read_from_file(file_path)?;
    println!("Successfully read text ({} bytes)", text.len());

    // Configure the word processor
    let config = WordProcessorConfig::default()
        .case_sensitive(false) // Treat "Word" and "word" as the same
        .include_numbers(false) // Skip pure number tokens
        .strip_punctuation(true); // Remove surrounding punctuation

    println!("Parsing text into words...");
    let words = parse_text(&text, &config);
    println!("Found {} total words", words.len());

    // Count word frequencies
    println!("Counting word frequencies...");
    let mut word_counts: HashMap<&String, usize> = HashMap::new();
    for word in &words {
        *word_counts.entry(word).or_insert(0) += 1;
    }
    println!("Found {} unique words", word_counts.len());

    // Calculate capacity
    println!("Estimating hash table capacity...");
    let capacity = estimate_capacity(&text, &config, EstimationMethod::FullAnalysis)?;
    println!("Recommended hash table capacity: {}", capacity);

    // Top 10 most common words
    println!("\nTop 10 most common words:");
    let mut word_frequency: Vec<_> = word_counts.into_iter().collect();
    word_frequency.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (word, count)) in word_frequency.iter().take(10).enumerate() {
        println!("{}. '{}' - {} occurrences", i + 1, word, count);
    }

    // Distribution of word lengths
    println!("\nWord length distribution:");
    let mut length_counts = HashMap::new();
    for word in &words {
        *length_counts.entry(word.len()).or_insert(0) += 1;
    }

    let mut lengths: Vec<_> = length_counts.iter().collect();
    lengths.sort_by_key(|&(len, _)| *len);

    for (length, count) in lengths {
        let percentage = (*count as f64 / words.len() as f64) * 100.0;
        println!(
            "{} characters: {} words ({:.2}%)",
            length, count, percentage
        );
    }

    // Sample sentences containing the most common word
    if let Some((most_common_word, _)) = word_frequency.first() {
        println!("\nSample sentences containing '{}':", most_common_word);

        // Split text into sentences (very simple approach)
        let sentences: Vec<&str> = text
            .split(&['.', '!', '?'][..])
            .filter(|s| s.contains(&**most_common_word))
            .take(3)
            .collect();

        for (i, sentence) in sentences.iter().enumerate() {
            let trimmed = sentence.trim();
            if !trimmed.is_empty() {
                println!("{}. \"{}\"", i + 1, trimmed);
            }
        }
    }

    println!("\nAnalysis complete!");
    Ok(())
}
