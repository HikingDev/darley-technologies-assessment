use crate::error::BinanceOptionsClientError;
use crate::model::OptionTicker;
use log::debug;
use serde_json::Deserializer;
use serde_json::Value;

/// Defines the available JSON parsing strategies.
///
/// # Variants
///
/// * `Streaming` - Uses incremental, memory-efficient JSON parsing.
/// * `Direct` - Parses the entire JSON document at once.
pub enum ParsingStrategy {
    /// Stream-based parsing for reduced memory usage
    Streaming,
    /// Full document parsing for potentially faster processing
    Direct,
}

impl Default for ParsingStrategy {
    /// Returns the default parsing strategy (Streaming).
    ///
    /// The streaming strategy is chosen by default because it offers
    /// better memory efficiency when processing large JSON payloads.
    fn default() -> Self {
        ParsingStrategy::Streaming
    }
}

/// Parses ticker JSON data using streaming deserialization.
///
/// This function leverages `serde_json::Deserializer` to iterate over
/// the JSON input and parse it into a vector of `OptionTicker` entries,
/// reducing memory usage for large payloads.
///
/// # Arguments
///
/// * `json_data` - A string slice containing JSON ticker data.
///
/// # Returns
///
/// A vector of `OptionTicker` entries on success.
pub fn parse_ticker_streaming(
    json_data: &str,
) -> Result<Vec<OptionTicker>, BinanceOptionsClientError> {
    debug!(
        "Raw JSON response (first 200 chars): {}",
        json_data.chars().take(200).collect::<String>()
    );

    // Create a stream deserializer that first yields a single top-level Value.
    let mut stream = Deserializer::from_str(json_data).into_iter::<Value>();

    // Expect the first (and only) value to be the JSON array.
    let top_value = stream
        .next()
        .ok_or_else(|| BinanceOptionsClientError::Unknown("No JSON data".to_string()))??;

    match top_value {
        Value::Array(arr) => {
            let mut tickers = Vec::with_capacity(1600); // max 1400 Tickers currently so a little buffer wont harm
            // Iterate over each element in the array and deserialize it.
            for item in arr {
                let ticker: OptionTicker =
                    serde_json::from_value(item).map_err(BinanceOptionsClientError::JsonParse)?;
                tickers.push(ticker);
            }
            Ok(tickers)
        }
        _ => Err(BinanceOptionsClientError::Unknown(
            "Expected JSON array at top-level".to_string(),
        )),
    }
}

/// Parses ticker JSON data using direct deserialization.
///
/// This function uses `serde_json::from_str` to parse the entire JSON document
/// at once into a vector of `OptionTicker` entries. This approach may be faster
/// for small payloads but uses more memory than streaming deserialization.
///
/// # Arguments
///
/// * `json_data` - A string slice containing JSON ticker data.
///
/// # Returns
///
/// A vector of `OptionTicker` entries on success.
pub fn parse_ticker_direct(
    json_data: &str,
) -> Result<Vec<OptionTicker>, BinanceOptionsClientError> {
    let tickers: Vec<OptionTicker> =
        serde_json::from_str(json_data).map_err(BinanceOptionsClientError::JsonParse)?;
    Ok(tickers)
}

/// Parses ticker JSON data using the specified strategy.
///
/// This function provides a unified interface to parse ticker data with either
/// streaming or direct deserialization based on the provided strategy.
///
/// # Arguments
///
/// * `data` - A string slice containing JSON ticker data.
/// * `strategy` - An optional parsing strategy. If None, defaults to streaming deserialization.
///
/// # Returns
///
/// A vector of `OptionTicker` entries on success.
pub fn parse_ticker(
    data: &str,
    strategy: Option<ParsingStrategy>,
) -> Result<Vec<OptionTicker>, BinanceOptionsClientError> {
    match strategy.unwrap_or_default() {
        ParsingStrategy::Streaming => parse_ticker_streaming(data),
        ParsingStrategy::Direct => parse_ticker_direct(data),
    }
}
