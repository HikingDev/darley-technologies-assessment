# Implementation Overview

## Serialization/Deserialization Considerations

**Full Document Deserialization:**
Load the entire JSON payload into memory using serde_json::from_str.
While straightforward, this approach can become memory-intensive with large responses.

**Streaming Deserialization (Chosen):**
Use `serde_json::Deserializer::from_str` to incrementally parse the JSON stream.
This method minimizes memory usage by processing one JSON entry at a time, making it ideal for high-frequency or large data sets.

**Asynchronous Parsing:**
Incorporate async I/O (e.g. using `tokio_serde_json` or `actson`) to process data non-blockingly. Although promising for scalability, this option adds complexity and was postponed for a later stage.

**Chosen Solution**
For the initial implementation, streaming deserialization was chosen because:

- It provides a low memory footprint by processing the JSON stream entry by entry.
- It meets the low-latency requirements by parsing each instrument statistic quickly.
- It is simple to integrate using the synchronous I/O primitives available in Rust.


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

**Reqwest (Recommended):**
- Built on Hyper/Tokio with excellent performance
- High-level API with easy JSON handling
- Connection pooling and HTTP/2 support
- Matches Binance's official (Spot) SDK approach (uses Hyper)

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
