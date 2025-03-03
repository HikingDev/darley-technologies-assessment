use binance_options_client::parser::ParsingStrategy;
use binance_options_client::{BinanceOptionsClient, OptionTicker};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use futures::future::join_all;
use rand::prelude::IndexedRandom;
use std::time::Duration;
use tokio::runtime::Runtime;

fn bench_complete_workflow(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let client = BinanceOptionsClient::new();

    // Benchmark the complete workflow with different request counts
    let mut group = c.benchmark_group("Complete Ticker Workflow");
    group.measurement_time(Duration::from_secs(10));

    for &request_count in &[10, 100, 1000] {
        group.bench_with_input(
            BenchmarkId::new("simulated_requests", request_count),
            &request_count,
            |b, &n| {
                // Fetch data once before benchmarking the processing
                let full_json = rt.block_on(async {
                    client
                        .get_ticker_raw(None)
                        .await
                        .expect("Failed to fetch tickers")
                });

                let tickers = rt.block_on(async {
                    let (tickers, _) = client
                        .parse_ticker_with_metrics(&full_json, Some(ParsingStrategy::Streaming))
                        .expect("Failed to parse tickers");
                    tickers
                });

                b.iter(|| {
                    rt.block_on(async {
                        let mut rng = rand::rng();

                        // Simulate n parallel requests for random tickers
                        let tasks = (0..n).map(|_| {
                            let ticker = tickers
                                .choose(&mut rng)
                                .expect("Ticker pool is empty")
                                .clone();

                            async move {
                                // Simulate processing each ticker
                                black_box(process_ticker(&ticker));
                            }
                        });

                        join_all(tasks).await;
                    })
                });
            },
        );
    }
    group.finish();
}

// Benchmark just the parsing step with different strategies
fn bench_parsing_strategies(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let client = BinanceOptionsClient::new();

    // Fetch the full ticker data once
    let full_json = rt.block_on(async {
        client
            .get_ticker_raw(None)
            .await
            .expect("Failed to fetch tickers")
    });

    // Get a smaller JSON sample for single ticker tests
    let eth_json = rt.block_on(async {
        client
            .get_ticker_raw(Some("ETH-250307-2450-C"))
            .await
            .expect("Failed to fetch ETH ticker")
    });

    let mut group = c.benchmark_group("Parsing Strategies");
    group.measurement_time(Duration::from_secs(10));

    // Benchmark streaming vs direct parsing for both single and all tickers
    group.bench_function("streaming_single_ticker", |b| {
        b.iter(|| {
            let (tickers, _) = client
                .parse_ticker_with_metrics(&eth_json, Some(ParsingStrategy::Streaming))
                .expect("Failed to parse ticker");
            black_box(tickers)
        })
    });

    group.bench_function("direct_single_ticker", |b| {
        b.iter(|| {
            let (tickers, _) = client
                .parse_ticker_with_metrics(&eth_json, Some(ParsingStrategy::Direct))
                .expect("Failed to parse ticker");
            black_box(tickers)
        })
    });

    group.bench_function("streaming_all_tickers", |b| {
        b.iter(|| {
            let (tickers, _) = client
                .parse_ticker_with_metrics(&full_json, Some(ParsingStrategy::Streaming))
                .expect("Failed to parse tickers");
            black_box(tickers)
        })
    });

    group.bench_function("direct_all_tickers", |b| {
        b.iter(|| {
            let (tickers, _) = client
                .parse_ticker_with_metrics(&full_json, Some(ParsingStrategy::Direct))
                .expect("Failed to parse tickers");
            black_box(tickers)
        })
    });

    group.finish();
}

#[inline]
fn process_ticker(ticker: &OptionTicker) -> String {
    format!(
        "{} - {} - {} - {} - {}",
        ticker.symbol, ticker.last_price, ticker.volume, ticker.strike_price, ticker.trade_count
    )
}

fn bench_fetch_ticker_data(c: &mut Criterion) {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let client = BinanceOptionsClient::new();

    let mut group = c.benchmark_group("API Fetch");

    group.bench_function("fetch_all_tickers", |b| {
        b.iter(|| {
            rt.block_on(async {
                black_box(
                    client
                        .get_ticker_raw(None)
                        .await
                        .expect("Failed to fetch tickers"),
                )
            })
        })
    });

    group.bench_function("fetch_single_ticker", |b| {
        b.iter(|| {
            rt.block_on(async {
                black_box(
                    client
                        .get_ticker_raw(Some("ETH-250307-2450-C"))
                        .await
                        .expect("Failed to fetch ETH ticker"),
                )
            })
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_complete_workflow,
    bench_parsing_strategies,
    bench_fetch_ticker_data
);
criterion_main!(benches);
