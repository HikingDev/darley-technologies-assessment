use reqwest::StatusCode;
use thiserror::Error;

/// A unified error type for the Binance Options client.
#[derive(Debug, Error)]
pub enum BinanceOptionsClientError {
    /// A general HTTP/network error (e.g., request failed or timed out).
    /// This variant captures lower-level issues from Reqwest.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// A non-success HTTP status was returned.
    /// We capture the status code and response body (if any).
    #[error("Received HTTP {code}. Body: {body}")]
    HttpResponse { code: StatusCode, body: String },

    /// Binance API indicates an error in the JSON response body,
    /// e.g. {"code":-1121, "msg":"Invalid symbol."}
    #[error("Binance API error code {code}: {msg}")]
    ApiError { code: i64, msg: String },

    /// JSON (de)serialization error occurred (e.g., malformed JSON).
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    /// Catch-all for unexpected or unclassified errors.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl BinanceOptionsClientError {
    /// Construct an error from an HTTP response that is not successful (e.g., 4XX/5XX).
    /// This helper can parse JSON to check if there's a known Binance "code" / "msg".
    ///
    /// Example usage:
    /// ```rust
    /// use reqwest::Client;
    /// use binance_options_client::error::BinanceOptionsClientError;
    ///
    /// #[tokio::main] // Required for async context in doctests
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new();
    ///     let response = client.get("https://www.binance.com/api/v3/ticker/price?symbol=BTCUSDT").send().await?;
    ///     if !response.status().is_success() {
    ///         let err = BinanceOptionsClientError::from_response(response).await;
    ///         println!("Error: {}", err);
    ///         // Handle the error appropriately in a real application
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn from_response(response: reqwest::Response) -> Self {
        let code = response.status();
        let body = match response.text().await {
            Ok(b) => b,
            Err(e) => return Self::Network(e),
        };

        // Try to parse the standard Binance error: { "code": i64, "msg": String }
        if let Ok(binance_err) = serde_json::from_str::<BinanceApiError>(&body) {
            Self::ApiError {
                code: binance_err.code,
                msg: binance_err.msg,
            }
        } else {
            // If it doesn't match the Binance error structure, fall back to a generic HTTP error
            Self::HttpResponse { code, body }
        }
    }
}

/// Helper struct to parse a Binance error payload if present.
#[derive(Debug, serde::Deserialize)]
struct BinanceApiError {
    code: i64,
    msg: String,
}
