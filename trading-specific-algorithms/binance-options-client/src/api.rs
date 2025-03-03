use crate::error::BinanceOptionsClientError;
use crate::model::{OptionTicker, ParsingMetrics};
use log::{debug, error, info, warn};
use reqwest::{Client, Method};
use serde::de::DeserializeOwned;
use std::time::Instant;

/// Base URL for the Binance Options API.
const BASE_URL: &str = "https://eapi.binance.com";
/// Endpoint for retrieving ticker data.
const TICKER_ENDPOINT: &str = "/eapi/v1/ticker";

/// Client for interacting with the Binance Options API.
pub struct BinanceOptionsClient {
    /// The underlying HTTP client.
    client: Client,
    /// The base URL for API requests.
    base_url: String,
}

/// Represents an HTTP request to the Binance Options API.
pub struct Request {
    /// The API endpoint path.
    pub path: String,
    /// HTTP method to be used (GET, POST, etc.).
    pub method: Method,
    /// Query parameters as key-value pairs.
    pub params: Vec<(String, String)>,
    /// Indicates if an API key is required.
    pub requires_api_key: bool,
    /// Indicates if a signature is required.
    pub requires_signature: bool,
}

/// Builder for constructing a ticker request.
pub struct TickerRequest {
    symbol: Option<String>,
}

impl TickerRequest {
    /// Creates a new ticker request without any symbol filter.
    pub fn new() -> Self {
        Self { symbol: None }
    }

    /// Sets the ticker symbol for the request.
    ///
    /// # Arguments
    ///
    /// * `symbol` - A string slice representing the ticker symbol.
    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_owned());
        self
    }
}

impl Default for TickerRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl From<TickerRequest> for Request {
    /// Converts a `TickerRequest` into a generic `Request` using the pre-defined ticker endpoint.
    fn from(request: TickerRequest) -> Self {
        let mut params = vec![];

        if let Some(symbol) = request.symbol {
            params.push(("symbol".to_owned(), symbol));
        }

        Request {
            path: TICKER_ENDPOINT.to_owned(),
            method: Method::GET,
            params,
            requires_api_key: false,
            requires_signature: false,
        }
    }
}

impl BinanceOptionsClient {
    /// Creates a new instance of `BinanceOptionsClient`.
    pub fn new() -> Self {
        info!(
            "Creating new BinanceOptionsClient with base URL: {}",
            BASE_URL
        );
        Self {
            client: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    /// Sends an HTTP request to the Binance Options API and returns the deserialized response.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type into which the response will be deserialized.
    ///
    /// # Errors
    ///
    /// Returns a `BinanceOptionsClientError` if the network request fails,
    /// the response status is unsuccessful, or JSON parsing fails.
    pub async fn send_request<T: DeserializeOwned>(
        &self,
        request: Request,
    ) -> Result<T, BinanceOptionsClientError> {
        let url = format!("{}{}", self.base_url, request.path);
        debug!(
            "Sending request to: {} with method: {:?}",
            url, request.method
        );

        if !request.params.is_empty() {
            debug!("Request parameters: {:?}", request.params);
        }

        let mut request_builder = match request.method {
            Method::GET => self.client.get(&url),
            Method::POST => self.client.post(&url),
            Method::PUT => self.client.put(&url),
            Method::DELETE => self.client.delete(&url),
            _ => {
                error!("Unsupported HTTP method: {:?}", request.method);
                return Err(BinanceOptionsClientError::Unknown(
                    "Unsupported HTTP method".to_string(),
                ));
            }
        };

        if !request.params.is_empty() {
            request_builder = request_builder.query(&request.params);
        }

        let response = match request_builder.send().await {
            Ok(resp) => resp,
            Err(e) => {
                error!("Network error: {}", e);
                return Err(BinanceOptionsClientError::Network(e));
            }
        };

        if !response.status().is_success() {
            warn!("Request failed with status: {}", response.status());
            return Err(BinanceOptionsClientError::from_response(response).await);
        }

        let text = match response.text().await {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to get response text: {}", e);
                return Err(BinanceOptionsClientError::Network(e));
            }
        };

        debug!(
            "Received response (first 200 chars): {}",
            text.chars().take(200).collect::<String>()
        );

        let data = match serde_json::from_str(&text) {
            Ok(d) => d,
            Err(e) => {
                error!("JSON parse error: {}", e);
                error!("JSON data: {}", text);
                return Err(BinanceOptionsClientError::JsonParse(e));
            }
        };

        info!("Request completed successfully");
        Ok(data)
    }

    /// Retrieves raw ticker data as a JSON string from the Binance Options API.
    ///
    /// # Returns
    ///
    /// A string containing the raw JSON response.
    pub async fn get_ticker_raw(
        &self,
        symbol: Option<&str>,
    ) -> Result<String, BinanceOptionsClientError> {
        info!(
            "Getting raw ticker data{}",
            symbol.map_or(String::new(), |s| format!(" for symbol: {}", s))
        );

        // Start with a TickerRequest.
        let mut ticker_req = TickerRequest::new();
        if let Some(s) = symbol {
            ticker_req = ticker_req.symbol(s);
        }

        // Convert to Request (which has `path` and `params` fields).
        let req: Request = ticker_req.into();

        let url = format!("{}{}", self.base_url, req.path);
        let mut request_builder = self.client.get(&url);
        if !req.params.is_empty() {
            request_builder = request_builder.query(&req.params);
        }

        let response = request_builder
            .send()
            .await
            .map_err(BinanceOptionsClientError::Network)?;
        if !response.status().is_success() {
            return Err(BinanceOptionsClientError::from_response(response).await);
        }
        let text = response
            .text()
            .await
            .map_err(BinanceOptionsClientError::Network)?;
        Ok(text)
    }

    /// Parses ticker JSON data using the specified parsing strategy (default is streaming)
    /// and measures performance metrics.
    ///
    /// # Arguments
    ///
    /// * `json_data` - A string slice containing JSON ticker data.
    /// * `strategy` - An optional parsing strategy. If `None` is provided, streaming is used.
    ///
    /// # Returns
    ///
    /// A tuple containing a vector of `OptionTicker` entries and the associated parsing metrics.
    pub fn parse_ticker_with_metrics(
        &self,
        json_data: &str,
        strategy: Option<crate::parser::ParsingStrategy>,
    ) -> Result<(Vec<OptionTicker>, ParsingMetrics), BinanceOptionsClientError> {
        info!(
            "Parsing ticker data using selected strategy (default is streaming) and measuring performance"
        );
        let start = Instant::now();

        // Delegate to the parser module.
        let tickers = crate::parser::parse_ticker(json_data, strategy)?;

        let duration = start.elapsed();
        let entry_count = tickers.len().max(1);
        let total_time_ms = duration.as_secs_f64() * 1000.0;
        let time_per_entry_ms = total_time_ms / entry_count as f64;

        let metrics = ParsingMetrics {
            time_per_entry_ms,
            entries_parsed: entry_count,
            total_time_ms,
        };

        info!(
            "Parsed {} ticker entries in {:.3} ms ({:.6} ms per entry)",
            entry_count, total_time_ms, time_per_entry_ms
        );

        Ok((tickers, metrics))
    }

    /// Parses ticker JSON data using the specified parsing strategy (default is streaming).
    ///
    /// # Arguments
    ///
    /// * `json_data` - A string slice containing JSON ticker data.
    /// * `strategy` - An optional parsing strategy. If `None` is provided, streaming is used.
    ///
    /// # Returns
    ///
    /// A vector of `OptionTicker` entries.
    pub fn parse_ticker(
        &self,
        json_data: &str,
        strategy: Option<crate::parser::ParsingStrategy>,
    ) -> Result<Vec<OptionTicker>, BinanceOptionsClientError> {
        info!("Parsing ticker data using selected strategy (default is streaming)");

        // Delegate to the parser module.
        let tickers = crate::parser::parse_ticker(json_data, strategy)?;

        info!("Parsed {} ticker entries", tickers.len());

        Ok(tickers)
    }
}

#[cfg(test)]
mod tests {
    use super::{Method, Request, TickerRequest};
    use crate::api::TICKER_ENDPOINT;

    #[test]
    fn ticker_request_convert_to_request_test() {
        let request: Request = TickerRequest::new().symbol("BTC-200730-9000-C").into();

        assert_eq!(request.path, TICKER_ENDPOINT.to_string());
        assert_eq!(request.method, Method::GET);
        assert_eq!(
            request.params,
            vec![("symbol".to_owned(), "BTC-200730-9000-C".to_string())]
        );
    }
}
