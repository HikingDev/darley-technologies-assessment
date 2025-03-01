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
pub mod io;
mod parser;

pub use capacity::estimate_capacity;
pub use config::WordProcessorConfig;
pub use parser::parse_text;

// If you want to re-export the `io` APIs directly, you could do so here, e.g.:
// pub use io::{read_text_from_path, fetch_text_from_url};

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
