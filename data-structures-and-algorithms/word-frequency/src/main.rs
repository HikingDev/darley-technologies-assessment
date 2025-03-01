//! Word Frequency Counter
//!
//! This is a command-line tool that combines the hash-table and word-processor libraries
//! to analyze word frequencies in text from files or URLs.

use clap::Parser;
use std::error::Error;
use std::str::FromStr;
use word_processor::{WordProcessorConfig, io, parse_text};

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

    /// Treat words as case-sensitive (default: true)
    #[clap(long, action, default_value = "true")]
    case_sensitive: bool,

    /// Include numbers as words (default: false)
    #[clap(long, action, default_value = "false")]
    include_numbers: bool,

    /// Skip common stop words like "the", "a", "in" (default: false)
    #[clap(long, action, default_value = "false")]
    skip_stop_words: bool,

    /// Strip punctuation from words (default: true)
    #[clap(long, action, default_value = "true")]
    strip_punctuation: bool,

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
    println!("  Strip punctuation: {}", !args.strip_punctuation);

    println!("\nReading from: {}", args.input);
    let text = if args.input.starts_with("http://") || args.input.starts_with("https://") {
        io::fetch_from_url(&args.input)?
    } else {
        io::read_from_file(&args.input)?
    };

    let word_processor_config = WordProcessorConfig::default()
        .case_sensitive(args.case_sensitive)
        .include_numbers(args.include_numbers)
        .skip_stop_words(args.skip_stop_words)
        .strip_punctuation(args.strip_punctuation)
        .capacity_factor(args.capacity_factor);

    // Parse text into words
    println!("Parsing text into words...");
    let words = parse_text(&text, &word_processor_config);
    println!("Found {} words in total", words.len());
    Ok(())
}
