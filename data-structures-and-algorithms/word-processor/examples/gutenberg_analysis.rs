//! Example that fetches "A Tale of Two Cities" from Project Gutenberg
//! and analyzes its word content using the word processor.

use std::error::Error;
use word_processor::{
    EstimationMethod, WordProcessorConfig, WordProcessorError, estimate_capacity, io, parse_text,
};

#[derive(Debug)]
struct WordStats {
    total_words: usize,
    unique_words: usize,
    most_common: Vec<(String, usize)>,
    longest_words: Vec<String>,
    capacity_needed: usize,
}

fn analyze_text(text: &str) -> Result<WordStats, WordProcessorError> {
    // Configure the word processor
    let config = WordProcessorConfig::default()
        .case_sensitive(false) // Treat "Word" and "word" as the same
        .include_numbers(false) // Skip pure number tokens
        .strip_punctuation(true) // Remove surrounding punctuation
        .skip_stop_words(false); // Include common words like "the", "a", etc.

    println!("Parsing text into words...");
    // Parse the text into words
    let words = parse_text(text, &config);

    println!("Calculating unique words...");
    // Count word frequencies
    let mut word_counts: std::collections::HashMap<&String, usize> =
        std::collections::HashMap::new();
    for word in &words {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    // Find most common words
    let mut word_counts_vec: Vec<_> = word_counts.into_iter().collect();
    word_counts_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let most_common: Vec<_> = word_counts_vec
        .iter()
        .take(10)
        .map(|(word, count)| ((*word).clone(), *count))
        .collect();

    // Find longest words
    let mut longest_words: Vec<_> = word_counts_vec
        .iter()
        .map(|(word, _)| (*word).clone())
        .collect();
    longest_words.sort_by_key(|word| std::cmp::Reverse(word.len()));
    let longest_words = longest_words.into_iter().take(10).collect();

    // Estimate capacity needed for hash table
    println!("Estimating capacity...");
    let capacity_needed = estimate_capacity(text, &config, EstimationMethod::FullAnalysis)?;

    Ok(WordStats {
        total_words: words.len(),
        unique_words: word_counts_vec.len(),
        most_common,
        longest_words,
        capacity_needed,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Fetching 'A Tale of Two Cities' from Project Gutenberg...");

    // Attempt to fetch the book from Project Gutenberg
    let text = match io::fetch_gutenberg_book() {
        Ok(content) => {
            println!("Successfully fetched text ({} bytes)", content.len());
            content
        }
        Err(e) => {
            eprintln!("Error fetching from URL: {}", e);
            eprintln!("Falling back to local copy if available...");

            // Try to read from a local file as fallback
            match io::read_from_file("examples/tale_of_two_cities.txt") {
                Ok(content) => {
                    println!("Read from local file ({} bytes)", content.len());
                    content
                }
                Err(e) => {
                    eprintln!("Error reading local file: {}", e);
                    eprintln!("To run this example:");
                    eprintln!(
                        "1. Either enable the 'url' feature: cargo run --example gutenberg_analysis --features url"
                    );
                    eprintln!("2. Or download the text to examples/tale_of_two_cities.txt");
                    return Err(e.into());
                }
            }
        }
    };

    // Analyze the text
    let stats = analyze_text(&text)?;

    // Print the results
    println!("\n### Stats for 'A Tale of Two Cities' ###");
    println!("Total words: {}", stats.total_words);
    println!("Unique words: {}", stats.unique_words);
    println!("Required hash table capacity: {}", stats.capacity_needed);

    println!("\nMost common words:");
    for (i, (word, count)) in stats.most_common.iter().enumerate() {
        println!("{}. '{}' - {} occurrences", i + 1, word, count);
    }

    println!("\nLongest words:");
    for (i, word) in stats.longest_words.iter().enumerate() {
        println!("{}. '{}' - {} characters", i + 1, word, word.len());
    }

    Ok(())
}
