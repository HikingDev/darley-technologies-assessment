//! Word Frequency Counter
//!
//! This is a command-line tool that combines the hash-table and word-processor libraries
//! to analyze word frequencies in text from files or URLs.

use clap::Parser;
use std::error::Error;
use std::str::FromStr;

/// Capacity configuration for the hash table
#[derive(Debug, Clone)]
enum CapacityConfig {
    /// Fixed size specified by user
    Fixed(usize),

    /// Automatically determine using full text analysis (default)
    Auto,

    /// Use sampling-based estimation with specified sample size
    Sampling(usize),
}

impl FromStr for CapacityConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("auto") {
            Ok(CapacityConfig::Auto)
        } else if s.starts_with("sample:") {
            // Extract the number after "sample:", e.g., "sample:1000" -> 1000
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() != 2 {
                return Err("Sampling format should be 'sample:SIZE'".into());
            }

            let size = parts[1]
                .parse::<usize>()
                .map_err(|_| "Invalid sample size, must be a positive number".to_string())?;

            Ok(CapacityConfig::Sampling(size))
        } else {
            // Try to parse as a fixed number
            match s.parse::<usize>() {
                Ok(size) => Ok(CapacityConfig::Fixed(size)),
                Err(_) => Err(format!(
                    "Invalid capacity: '{}'. Use a number, 'auto', or 'sample:SIZE'",
                    s
                )),
            }
        }
    }
}

impl std::fmt::Display for CapacityConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CapacityConfig::Fixed(size) => write!(f, "{} (fixed)", size),
            CapacityConfig::Auto => write!(f, "auto (full analysis)"),
            CapacityConfig::Sampling(size) => write!(f, "auto (sampling with {} chars)", size),
        }
    }
}

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
    #[clap(short, long, value_parser, required(true))]
    input: String,

    /// Hash table capacity: a number for fixed size, 'auto' for full analysis,
    /// or 'sample:SIZE' for sampling-based estimation
    #[clap(short, long, value_parser, default_value = "auto")]
    capacity: CapacityConfig,

    /// Treat words as case-sensitive (default: false)
    #[clap(long, action, default_value = "false")]
    case_sensitive: bool,

    /// Include numbers as words (default: false)
    #[clap(long, action, default_value = "false")]
    include_numbers: bool,

    /// Skip common stop words like "the", "a", "in" (default: false)
    #[clap(long, action, default_value = "false")]
    skip_stop_words: bool,

    /// Don't strip punctuation from words (default: false)
    #[clap(long, action, default_value = "false")]
    keep_punctuation: bool,

    /// Multiplier for capacity estimation
    #[clap(long, default_value = "1.5", value_parser)]
    capacity_factor: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();

    println!("Configuration:");
    println!("  Input source: {}", args.input);
    println!("  Hash table capacity: {}", args.capacity);
    println!("  Capacity factor: {}", args.capacity_factor);
    println!("  Case sensitive: {}", args.case_sensitive);
    println!("  Include numbers: {}", args.include_numbers);
    println!("  Skip stop words: {}", args.skip_stop_words);
    println!("  Strip punctuation: {}", !args.keep_punctuation);

    // TODO: Implement the actual functionality
    println!("\nNot yet implemented.");

    Ok(())
}
