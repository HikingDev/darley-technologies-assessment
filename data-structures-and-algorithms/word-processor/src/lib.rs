/*!
word-processor
==============

A library for:
  - Configuring word parsing (see `WordProcessorConfig`).
  - Parsing raw text into words (`parse_text`).
  - Estimating table capacity (`estimate_capacity`).
  - (Optional) reading text from files or fetching text from URLs (see `io` module).

This module re-exports the main structs and functions.
*/

mod capacity;
mod config;
pub mod error;
pub mod io;
mod parser;

// Re-export the main structs and functions
pub use config::WordProcessorConfig;
pub use error::WordProcessorError;
pub use parser::parse_text;
pub use capacity::{estimate_capacity, EstimationMethod};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_check() {
        let conf = WordProcessorConfig::default();
        let words = parse_text("Hello World!", &conf);
        assert_eq!(words, vec!["hello", "world"]);
    }
}
