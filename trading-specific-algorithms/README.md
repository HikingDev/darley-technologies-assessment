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
