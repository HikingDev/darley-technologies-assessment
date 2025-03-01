//! # I/O Operations for Word Processor
//!
//! This module handles input/output operations for the word processor, such as:
//! - Reading text from local files
//! - Fetching text from URLs (when the "url" feature is enabled)
//! - Specifically handling the Project Gutenberg book required by the assignment

use std::fs;
use std::path::Path;

use crate::error::{IoError, WordProcessorError};

/// Reads text from a local file path.
///
/// # Arguments
/// * `path` - Path to the file to read
///
/// # Returns
/// The file content as a String, or an error if reading fails
///
/// # Example
/// ```no_run
/// use word_processor::io::read_from_file;
///
/// match read_from_file("path/to/document.txt") {
///     Ok(content) => println!("Read {} characters", content.len()),
///     Err(err) => eprintln!("Error reading file: {}", err),
/// }
/// ```
pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<String, WordProcessorError> {
    fs::read_to_string(path).map_err(|err| IoError::FileReadError(err).into())
}

/// URL of the Project Gutenberg book specified in the assignment.
pub const GUTENBERG_BOOK_URL: &str = "https://www.gutenberg.org/files/98/98-0.txt";

/// Fetches the Project Gutenberg book specified in the assignment.
///
/// # Returns
/// The book content as a String, or an error if fetching fails
///
/// # Note
/// Requires the "url" feature to be enabled.
#[cfg(feature = "url")]
pub fn fetch_gutenberg_book() -> Result<String, WordProcessorError> {
    fetch_from_url(GUTENBERG_BOOK_URL)
}

/// Not-enabled placeholder for fetch_gutenberg_book when the "url" feature is disabled.
///
/// # Returns
/// Always returns an error indicating the feature is not enabled.
#[cfg(not(feature = "url"))]
pub fn fetch_gutenberg_book() -> Result<String, WordProcessorError> {
    Err(IoError::UrlFetchError(
        "URL feature not enabled. Add the 'url' feature to Cargo.toml".to_string(),
    )
    .into())
}

/// Fetches text from a URL.
///
/// # Arguments
/// * `url` - The URL to fetch text from
///
/// # Returns
/// The fetched content as a String, or an error if fetching fails
///
/// # Note
/// Requires the "url" feature to be enabled.
#[cfg(feature = "url")]
pub fn fetch_from_url(url: &str) -> Result<String, WordProcessorError> {
    // When the "url" feature is enabled, this function will use reqwest
    // to fetch the content from the given URL
    use reqwest::blocking::Client;

    let client = Client::new();
    client
        .get(url)
        .send()
        .map_err(|err| IoError::UrlFetchError(err.to_string()).into())
        .and_then(|response| {
            response
                .text()
                .map_err(|err| IoError::UrlFetchError(err.to_string()).into())
        })
}

/// Not-enabled placeholder for fetch_from_url when the "url" feature is disabled.
///
/// # Returns
/// Always returns an error indicating the feature is not enabled.
#[cfg(not(feature = "url"))]
pub fn fetch_from_url(_url: &str) -> Result<String, WordProcessorError> {
    Err(IoError::UrlFetchError(
        "URL feature not enabled. Add the 'url' feature to Cargo.toml".to_string(),
    )
    .into())
}

/// Reads text from a string path, file path, or URL.
///
/// This function is a convenience wrapper that attempts to interpret the input
/// as a file path first, and if that fails, tries to interpret it as a URL
/// (if the "url" feature is enabled).
///
/// # Arguments
/// * `source` - A file path or URL to read from
///
/// # Returns
/// The content as a String, or an error if reading fails
///
/// # Example
/// ```no_run
/// use word_processor::io::read_from_source;
///
/// // Try to read from a file or URL
/// match read_from_source("path/or/url") {
///     Ok(content) => println!("Read {} characters", content.len()),
///     Err(err) => eprintln!("Error: {}", err),
/// }
/// ```
pub fn read_from_source(source: &str) -> Result<String, WordProcessorError> {
    // First try as a file path
    let file_result = read_from_file(source);

    // If that fails and looks like a URL, try as a URL
    if file_result.is_err() && (source.starts_with("http://") || source.starts_with("https://")) {
        #[cfg(feature = "url")]
        {
            return fetch_from_url(source);
        }
    }

    // Return the file result (either success or the original error)
    file_result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_from_file() -> Result<(), WordProcessorError> {
        // Create a temporary file with some content
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "Hello, world!").unwrap();

        // Read from the file
        let content = read_from_file(file.path())?;

        assert!(content.contains("Hello, world!"));
        Ok(())
    }

    #[test]
    fn test_read_from_file_not_found() {
        let result = read_from_file("nonexistent_file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_from_source_file() -> Result<(), WordProcessorError> {
        // Create a temporary file with some content
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "File content").unwrap();

        // Read from the file using read_from_source
        let content = read_from_source(file.path().to_str().unwrap())?;

        assert!(content.contains("File content"));
        Ok(())
    }

    // URL tests would go here if the feature is enabled
    #[cfg(feature = "url")]
    #[test]
    fn test_fetch_from_url() {
        // This test is only compiled with the "url" feature
        // In a real test, you might use a mock server instead of an actual HTTP request
    }
}
