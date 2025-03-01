//! Word Frequency Counter
//!
//! This is a command-line tool that combines the hash-table and word-processor libraries
//! to analyze word frequencies in text from files or URLs.

use clap::Parser;
use std::error::Error;

/// Command line arguments for the Word Frequency Counter application
#[derive(Parser, Debug)]
#[clap(
    name = "Word Frequency Counter",
    author = "Benjamin Rösner",
    version = "1.0",
    about = "Analyzes word frequency in text using a hash table"
)]
struct Args {
    /// Input source: file path or URL
    #[clap(short, long, value_parser)]
    input: String,

    /// Fixed hash table capacity (default: auto-estimate)
    #[clap(short, long, value_parser)]
    capacity: Option<usize>,

    /// Treat words as case-sensitive
    #[clap(long, action)]
    case_sensitive: bool,

    /// Include numbers as words
    #[clap(long, action)]
    include_numbers: bool,

    /// Skip common stop words (like "the", "a", "in")
    #[clap(long, action)]
    skip_stop_words: bool,

    /// Don't strip punctuation from words
    #[clap(long, action)]
    keep_punctuation: bool,

    /// Multiplier for capacity estimation (default: 1.5)
    #[clap(long, default_value = "1.5", value_parser)]
    capacity_factor: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();

    println!("Configuration:");
    println!("  Input source: {}", args.input);

    if let Some(capacity) = args.capacity {
        println!("  Fixed hash table capacity: {}", capacity);
    } else {
        println!(
            "  Hash table capacity: auto (factor: {})",
            args.capacity_factor
        );
    }

    println!("  Case sensitive: {}", args.case_sensitive);
    println!("  Include numbers: {}", args.include_numbers);
    println!("  Skip stop words: {}", args.skip_stop_words);
    println!("  Strip punctuation: {}", !args.keep_punctuation);

    // TODO: Implement the actual functionality
    println!("\nNot yet implemented.");

    Ok(())
}
