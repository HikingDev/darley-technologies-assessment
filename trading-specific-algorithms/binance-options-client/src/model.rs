use serde::{Deserialize, Serialize};

/// Represents a ticker record returned from the Binance Options API.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct OptionTicker {
    /// The ticker symbol.
    pub symbol: String,
    /// Price change.
    #[serde(rename = "priceChange")]
    pub price_change: String,
    /// Price change percentage.
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: String,
    /// Last traded price.
    #[serde(rename = "lastPrice")]
    pub last_price: String,
    /// Last traded quantity.
    #[serde(rename = "lastQty")]
    pub last_qty: String,
    /// Opening price.
    pub open: String,
    /// Highest price.
    pub high: String,
    /// Lowest price.
    pub low: String,
    /// Trading volume.
    pub volume: String,
    /// Trading amount.
    pub amount: String,
    /// Bid price.
    #[serde(rename = "bidPrice")]
    pub bid_price: String,
    /// Ask price.
    #[serde(rename = "askPrice")]
    pub ask_price: String,
    /// Opening time (timestamp).
    #[serde(rename = "openTime")]
    pub open_time: i64,
    /// Closing time (timestamp).
    #[serde(rename = "closeTime")]
    pub close_time: i64,
    /// First trade ID.
    #[serde(rename = "firstTradeId")]
    pub first_trade_id: i64,
    /// Total number of trades.
    #[serde(rename = "tradeCount")]
    pub trade_count: i64,
    /// Strike price.
    #[serde(rename = "strikePrice")]
    pub strike_price: String,
    /// Exercise price.
    #[serde(rename = "exercisePrice")]
    pub exercise_price: String,
}

/// Contains metrics related to the performance of the JSON parsing process.
#[derive(Debug)]
pub struct ParsingMetrics {
    /// Average time taken per parsed entry in milliseconds.
    pub time_per_entry_ms: f64,
    /// Total number of entries parsed.
    pub entries_parsed: usize,
    /// Total parsing time in milliseconds.
    pub total_time_ms: f64,
}
