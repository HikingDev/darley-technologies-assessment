# Trading Specific Algorithms

## Assignment Task

Review the Binance European Options API documentation at [binance voptions](https://binance-docs.github.io/apidocs/voptions/en/).

Then:

- Retrieve data from the endpoint:
  ```
  GET /eapi/v1/ticker
  ```

- Write a parsing algorithm for the instrument statistics and print the result

- Measure the parsing speed for a single entry and evaluate the algorithmic complexity
of your approach. Document any potential optimizations aimed at achieving low
latency.

## Usage of binance-options-client and Analysis

The `binance-options-client` crate provides a convenient way to interact with the Binance Options API.
It handles the complexities of request signing, error handling, and data serialization/deserialization.

The comprehensive documentation for the `binance-options-client` and findings, analysis, results etc. can be found at
`./binance-options-client/README.md`

the example can be run using the following command:

```
cargo run --example minimal_example
```

===================================================================

**Task Analysis**

## Results Parsing

Single Entry (ETH only):

Time per entry: ~0.313834 ms
Total time: ~0.313834 ms
This relatively high per-entry time is due to fixed initialization overhead
(e.g., setting up the deserializer, and any other one-time costs)
that isn’t amortized when only one entry is processed.

Consideration:
- Try vector of Tickers to see how it affects the runtime.
- Also pre-allocate vector to avoid reallocations during runtime.

```
Instrument Statistics:
=======================
Symbol: ETH-250307-2450-C (Strike: 2450)
Last Price: 12.2 (Change: 0.7429%)
Bid/Ask: 10.4/10.6
24h High/Low: 13/7
Volume: 27.8 contracts (Amount: 232.51)
Trade Count: 9
--------------------------------------------------

Parsing Performance Metrics:
============================
Time per entry: 0.313834 ms
Total time: 0.313834 ms
Entries parsed: 1
```

All Entries (1400 tickers):

Time per entry: ~0.027663 ms
Total time: ~38.728 ms
With a larger batch, the fixed overhead is spread across many entries,
reducing the average time per entry.

```
Instrument Statistics:
=======================
Symbol: BNB-250305-555-P (Strike: 555)
Last Price: 0 (Change: 0%)
Bid/Ask: 0.8/1.7
24h High/Low: 0/0
Volume: 0 contracts (Amount: 0)
Trade Count: 0
--------------------------------------------------
Symbol: BTC-250314-115000-P (Strike: 115000)
Last Price: 18800 (Change: 0%)
Bid/Ask: 24780/0
24h High/Low: 18800/18800
Volume: 0 contracts (Amount: 0)
Trade Count: 0
--------------------------------------------------
Symbol: BTC-251226-120000-P (Strike: 120000)
Last Price: 0 (Change: 0%)
Bid/Ask: 10/0
24h High/Low: 0/0
Volume: 0 contracts (Amount: 0)
Trade Count: 0
--------------------------------------------------
Symbol: BTC-250303-85000-C (Strike: 85000)
Last Price: 1235 (Change: 0.0978%)
Bid/Ask: 1125/1205
24h High/Low: 1680/1115
Volume: 3.42 contracts (Amount: 3975.95)
Trade Count: 16
--------------------------------------------------
Symbol: ETH-250307-2450-C (Strike: 2450)
Last Price: 12.2 (Change: 0.7429%)
Bid/Ask: 10.6/10.8
24h High/Low: 13/7
Volume: 27.8 contracts (Amount: 232.51)
Trade Count: 9
--------------------------------------------------
... and 1395 more instruments

Parsing Performance Metrics:
============================
Time per entry: 0.027663 ms
Total time: 38.728000 ms
Entries parsed: 1400
```

after pre-allocating vector to avoid reallocations during runtime, the total rutime for the 1400 tickers
could be reduced

## Results Async Requests

Benchmarking Complete Ticker Workflow/simulated_requests/10: Warming up for 3.0000 s
thread 'main' panicked at binance-options-client/benches/benchmarks.rs:27:26:
Failed to fetch tickers: ApiError { code: -1003, msg: "Too many requests; current limit of IP(194.230.148.204) is 400 requests per minute. Please use the websocket for live updates to avoid polling the API." }

During our benchmark testing with async requests, we encountered API rate limits that make REST endpoint benchmarking ineffective for high-throughput scenarios.
As shown in the error message above, Binance imposes a strict limit of 400 requests per minute per IP address:

> ApiError { code: -1003, msg: "Too many requests; current limit of IP is 400 requests per minute. Please use the websocket for live updates to avoid polling the API." }

This limitation renders any benchmark attempting to simulate multiple simultaneous clients or high-frequency trading scenarios impractical using REST endpoints.
With only 400 requests per minute (approximately 6-7 requests per second), any realistic trading application would quickly exhaust this quota.

The error message directly points to the recommended solution: WebSockets. For real-time data needs, especially in trading applications where millisecond latency matters, WebSocket connections provide:

1. **Continuous data stream** without repeated polling
2. **Lower overhead** per data update
3. **Reduced latency** compared to REST polling
4. **No rate limiting** for passive listening
5. **Push-based architecture** ideal for market data

Our results confirm that while our parsing implementation is efficient (27.6μs per ticker),
any production system requiring frequent updates should use WebSockets for market data consumption and reserve REST endpoints for account management and trade execution only.

## Serialization/Deserialization Considerations

**Full Document Deserialization:**
- Load the entire JSON payload into memory using `serde_json::from_str`.
- While straightforward, this approach can become memory-intensive with large responses.

**Streaming Deserialization (Chosen):**
- Use `serde_json::Deserializer::from_str` to incrementally parse the JSON stream.
- Note: The Binance Options API returns ticker data as a JSON array rather than a stream of individual JSON objects (e.g., NDJSON). To accommodate this:
       1) Deserialize the top-level JSON into a serde_json::Value to extract the array.
       2) Deserialize each element in the array individually into an OptionTicker using a streaming approach.
- This method minimizes memory usage by processing one JSON entry at a time while maintaining low latency.

**Asynchronous Parsing:**
- Incorporate async I/O (e.g. using `tokio_serde_json` or `actson`) to process data non-blockingly. Although promising for scalability, this option adds complexity and was postponed for a later stage.
- Although this option is promising for scalability, it adds complexity and is postponed for a later stage.

**Chosen Solution**
For the initial implementation, streaming deserialization was chosen because:

- Provides a low memory footprint by processing JSON entries incrementally.
- Meets low-latency requirements by quickly parsing each instrument statistic.
- Simple to integrate using Rust’s synchronous I/O primitives.


### Potential Optimizations

**Asynchronous I/O:**
Transition to an async model using libraries like `tokio` or `async-std` to further reduce latency in scenarios with high I/O demands.

**Zero-Copy Deserialization:**
Investigate using `serde_json::from_slice or libraries that support zero-copy parsing to reduce CPU overhead during deserialization.

**Parallel Parsing:**
For extremely large datasets, consider splitting the data stream into chunks that can be processed in parallel.

**Profiling & Benchmarking:**
Integrate benchmarking tools (e.g., Criterion) to continuously profile performance and identify any bottlenecks.
Implement multiple strategies and compare their performance.

**Alternative JSON Libraries:**
Evaluate high-performance alternatives such as `simd_json` for potential gains in parsing speed.

### HTTP Client Considerations

**Reqwest (Chosen):**
- Built on Hyper/Tokio with excellent performance
- High-level API with easy JSON handling
- Connection pooling and HTTP/2 support
- Matches Binance's official (Spot) SDK approach (uses Hyper)
- **Asynchronous execution with Tokio:** This client leverages `reqwest` in asynchronous mode, managed by the `tokio` runtime.
  This allows for concurrent handling of multiple API requests, improving performance and responsiveness.

**Hyper:**
- Low-level HTTP implementation with maximum performance
- Requires more manual code but offers finest control
- Excellent for ultra-low-latency requirements
- Achieved ~18k requests/second in benchmarks

**Isahc (libcurl wrapper):**
- Built on battle-tested libcurl C library
- Good for unusual HTTP behaviors or legacy protocols
- Cross-language compatibility
- Benchmarks show lower throughput (~4k req/sec) versus Hyper

**Ureq:**
- Lightweight synchronous client with minimal dependencies
- Very low latency for simple, sequential requests
- Thread-per-request model limits scalability
- Good for low-volume trading bots that prioritize simplicity

**Surf:**
- Runtime-agnostic HTTP client (works with async-std or Tokio)
- Simple, elegant API
- Less performant than Hyper in benchmarks
- Flexibility comes with some overhead

**Actix Web Client (awc):**
- Integrates well with Actix actor model
- Good when already using Actix for other components
- Lower throughput than Hyper/Reqwest in benchmarks

**WebSockets (complementary approach):**
- For real-time data, WebSockets provide a more efficient alternative to polling
- Binance offers comprehensive WebSocket streams that should be leveraged
- Can drastically reduce REST call volume for market data
- [websockets binance api](https://developers.binance.com/docs/derivatives/option/websocket-market-streams)

## Selection Criteria

Our evaluation prioritized:
1. Low latency and high throughput
2. Efficient connection management
3. Memory usage under load
4. Parallelization capabilities
5. Ease of integration with trading workflows

Based on these requirements, Reqwest provides the best balance of performance and usability for our trading application,
with direct Hyper as an alternative when maximum control is needed.

Note:
Futures and Vanilla Options APIs are not supported by https://github.com/binance/binance-spot-connector-rust
But Since there are similarities, it can be leveraged for implementation
