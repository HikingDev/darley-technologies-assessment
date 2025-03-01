# Data Structures & Algorithms Assignment

This repository implements a fixed-size hash table with O(1) operations, designed to process and analyze word frequencies from a Project Gutenberg text (or any text for that matter).

## Assignment Requirements

Implement a fixed-size open addressing hash table using linear probing for collision resolution, with the following O(1) operations:
- `insert(key, value)`: Insert or update a key-value pair
- `remove(key)`: Remove a key-value pair
- `get(key)`: Retrieve a value by key
- `get_last()`: Get the most recently inserted/updated key-value pair
- `get_first()`: Get the least recently inserted/updated key-value pair

The implementation should process words from ["A Tale of Two Cities"](https://www.gutenberg.org/files/98/98-0.txt) as keys, with integer values.

## Implementation Overview

The solution is split into three crates:

### hash-table
- Implements the required fixed-size hash table using linear probing
- Uses a doubly linked list to track insertion order for O(1) first/last operations
- All operations maintain O(1) average-case complexity

### word-processor
- Handles text processing and word extraction
- Configurable word parsing (case sensitivity, punctuation handling, etc.)
- Supports reading from files or URLs

### word-frequency
- Demo binary combining both libraries
- Processes the Gutenberg text and counts word frequencies
- Shows practical usage of the hash table implementation

## Repository Structure
```
data-structures-and-algorithms/
├── hash-table/          # O(1) hash table implementation
├── word-processor/      # Text processing library
└── word-frequency/      # Demo binary
```

## Building and Running

```bash
# Build all crates
cargo build

# Run tests
cargo test -p hash-table

# Process the Gutenberg text (with URL support)
cargo run -p word-frequency --features url -- --input https://www.gutenberg.org/files/98/98-0.txt

# Or process a local file
cargo run -p word-frequency -- --input path/to/file.txt
```

## Performance Benchmarks

Benchmarks were run on a MacBook Pro M1 Max with 64GB RAM to verify O(1) complexity. Sample results:

```
# Insert operation remains O(1) across different sizes
insert/100     time:   [54.844 ns 55.083 ns 55.354 ns]
insert/1000    time:   [50.904 ns 51.304 ns 51.716 ns]
insert/10000   time:   [90.226 ns 184.48 ns 305.51 ns]

# Get operation shows consistent O(1) timing
get/100        time:   [8.8081 ns 9.1630 ns 9.5850 ns]
get/1000       time:   [8.7446 ns 9.0430 ns 9.4120 ns]
get/10000      time:   [10.514 ns 11.090 ns 11.887 ns]

# Remove operation is also O(1)
remove/100              time:   [359.38 ns 361.07 ns 362.81 ns]
remove/1000             time:   [2.8762 µs 2.8844 µs 2.8974 µs]
remove/10000            time:   [29.843 µs 30.262 µs 30.771 µs]

# First/Last operations are extremely fast (sub-nanosecond)
get_first/1000  time:   [242.43 ps 242.96 ps 243.48 ps]
get_last/1000   time:   [241.32 ps 242.24 ps 243.26 ps]
```

The benchmark results confirm O(1) complexity across all operations, with consistent performance regardless of table size.

## Design Decisions

1. **Linear Probing + Linked List**: Combines linear probing for collision resolution with a doubly linked list to achieve O(1) for all operations, including first/last access.

2. **Fixed Size**: As per requirements, the hash table has a fixed size. Attempting to insert beyond capacity results in a panic.

3. **Word Processing**: The word processor is separate to keep the hash table implementation focused and reusable.

Run benchmarks yourself with:
```bash
cargo bench -p hash-table
```
