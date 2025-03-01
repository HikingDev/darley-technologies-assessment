//! # Error Types for Word Processor
//!
//! This module defines all error types used throughout the word-processor library.

use std::error::Error;
use std::fmt;
use std::io;

/// The main error type for the word-processor library
#[derive(Debug)]
pub enum WordProcessorError {
    /// Errors related to input/output operations
    Io(IoError),

    /// Errors related to text parsing
    Parser(ParserError),

    /// Errors related to capacity estimation
    Capacity(CapacityError),

    /// Other general errors
    Other(String),
}

/// Errors that can occur during IO operations
#[derive(Debug)]
pub enum IoError {
    /// Error reading from a file
    FileReadError(io::Error),

    /// Error writing to a file
    FileWriteError(io::Error),

    /// Error fetching content from a URL
    UrlFetchError(String),
}

/// Errors that can occur during text parsing
#[derive(Debug)]
pub enum ParserError {
    /// Invalid regex pattern
    InvalidPattern(String),

    /// Text encoding error
    EncodingError(String),
}

/// Errors that can occur during capacity estimation
#[derive(Debug)]
pub enum CapacityError {
    /// Error when attempting to estimate capacity from empty text
    EmptyText,

    /// Error when sample size is invalid (e.g., zero)
    InvalidSampleSize(usize),

    /// Capacity factor is invalid (e.g., negative or zero)
    InvalidCapacityFactor(f32),
}

// Implement standard error traits

impl fmt::Display for WordProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "IO error: {}", err),
            Self::Parser(err) => write!(f, "Parser error: {}", err),
            Self::Capacity(err) => write!(f, "Capacity error: {}", err),
            Self::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileReadError(err) => write!(f, "File read error: {}", err),
            Self::FileWriteError(err) => write!(f, "File write error: {}", err),
            Self::UrlFetchError(err) => write!(f, "URL fetch error: {}", err),
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPattern(err) => write!(f, "Invalid pattern: {}", err),
            Self::EncodingError(err) => write!(f, "Encoding error: {}", err),
        }
    }
}

impl fmt::Display for CapacityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyText => write!(f, "Cannot estimate capacity from empty text"),
            Self::InvalidSampleSize(size) => write!(f, "Invalid sample size: {}", size),
            Self::InvalidCapacityFactor(factor) => write!(f, "Invalid capacity factor: {}", factor),
        }
    }
}

// Implement Error trait for all error types
impl Error for WordProcessorError {}
impl Error for IoError {}
impl Error for ParserError {}
impl Error for CapacityError {}

// Implement conversions from specific errors to the main error type
impl From<IoError> for WordProcessorError {
    fn from(err: IoError) -> Self {
        Self::Io(err)
    }
}

impl From<ParserError> for WordProcessorError {
    fn from(err: ParserError) -> Self {
        Self::Parser(err)
    }
}

impl From<CapacityError> for WordProcessorError {
    fn from(err: CapacityError) -> Self {
        Self::Capacity(err)
    }
}

impl From<io::Error> for WordProcessorError {
    fn from(err: io::Error) -> Self {
        Self::Io(IoError::FileReadError(err))
    }
}

impl From<String> for WordProcessorError {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}

impl From<&str> for WordProcessorError {
    fn from(err: &str) -> Self {
        Self::Other(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = WordProcessorError::Capacity(CapacityError::EmptyText);
        assert_eq!(
            err.to_string(),
            "Capacity error: Cannot estimate capacity from empty text"
        );
    }

    #[test]
    fn test_error_conversion() {
        let io_err =
            IoError::FileReadError(io::Error::new(io::ErrorKind::NotFound, "file not found"));
        let word_proc_err: WordProcessorError = io_err.into();

        match word_proc_err {
            WordProcessorError::Io(_) => assert!(true),
            _ => panic!("Expected Io error variant"),
        }
    }
}
