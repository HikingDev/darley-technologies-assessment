use binance_options_client::BinanceOptionsClient;
use binance_options_client::OptionTicker;
use binance_options_client::parser::ParsingStrategy;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = BinanceOptionsClient::new();

    //let json_data = client.get_ticker_raw("ETH-250307-2450-C".into()).await?; //only ETH Instrument
    let json_data = client.get_ticker_raw(None).await?;

    let (parsed_tickers, metrics) =
        client.parse_ticker_with_metrics(&json_data, Some(ParsingStrategy::Streaming))?;

    println!("\nInstrument Statistics:");
    println!("=======================");
    for (i, ticker) in parsed_tickers.iter().enumerate() {
        if i > 4 {
            println!("... and {} more instruments", parsed_tickers.len() - 5);
            break;
        }
        print_ticker_details(ticker);
        println!("--------------------------------------------------");
    }

    println!("\nParsing Performance Metrics:");
    println!("============================");
    println!("Time per entry: {:.6} ms", metrics.time_per_entry_ms);
    println!("Total time: {:.6} ms", metrics.total_time_ms);
    println!("Entries parsed: {}", metrics.entries_parsed);

    Ok(())
}

fn print_ticker_details(ticker: &OptionTicker) {
    println!(
        "Symbol: {} (Strike: {})",
        ticker.symbol, ticker.strike_price
    );
    println!(
        "Last Price: {} (Change: {}%)",
        ticker.last_price, ticker.price_change_percent
    );
    println!("Bid/Ask: {}/{}", ticker.bid_price, ticker.ask_price);
    println!("24h High/Low: {}/{}", ticker.high, ticker.low);
    println!(
        "Volume: {} contracts (Amount: {})",
        ticker.volume, ticker.amount
    );
    println!("Trade Count: {}", ticker.trade_count);
}
