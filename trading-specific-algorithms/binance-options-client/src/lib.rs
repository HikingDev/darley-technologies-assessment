pub mod api;
pub mod error;
pub mod model;
pub mod parser;

// Re-export key types for easy access
pub use api::BinanceOptionsClient;
pub use api::TickerRequest;
pub use error::BinanceOptionsClientError;
pub use model::OptionTicker;
pub use model::ParsingMetrics;

// Initialize logging
pub fn init_logging() {
    env_logger::init();
}
