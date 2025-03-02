# binance-options-client-cli

A command-line interface for interacting with the binance-options-client.

## Features (Planned)

*   **Ticker Selection:** Specify the desired ticker symbol to fetch data for.
*   **Request Interval Configuration:** Customize the interval at which data is requested from the Binance Options API.  This allows users to balance data freshness with API usage limits.

## Usage (Planned)

```bash
# Example usage (not yet implemented)
binance-options-client --ticker "BTC-250303-85000-C" --interval 5s
```

## Configuration

Currently, the CLI relies on the backend configuration.
Future versions will allow overriding certain parameters via command-line arguments or a configuration file.
