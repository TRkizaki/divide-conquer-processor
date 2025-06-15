# Rust Benchmark Runner

A comprehensive benchmarking framework for measuring the performance of various algorithms including sorting, matrix multiplication, and computational geometry operations.

## Overview

This Rust module provides a robust benchmarking system that measures execution time and memory usage for different algorithms. It supports both sequential and parallel implementations and offers multiple output formats for analysis.

## Features

- **Multi-Algorithm Support**: Benchmarks sorting algorithms, matrix operations, and geometric computations
- **Parallel Processing**: Supports benchmarking of parallel algorithm implementations
- **Memory Tracking**: Monitors memory usage during algorithm execution
- **Multiple Output Formats**: Results can be exported as JSON or CSV
- **Colored Console Output**: Enhanced readability with colored terminal output
- **Statistical Analysis**: Calculates averages and identifies best-performing algorithms

## Dependencies

```toml
[dependencies]
colored = "*"           # Terminal color output
memory-stats = "*"      # Memory usage monitoring
serde = { version = "*", features = ["derive"] }  # Serialization
```

## Core Components

### BenchmarkResult Structure

```rust
pub struct BenchmarkResult {
    pub algorithm_name: String,
    pub data_size: usize,
    pub execution_time: Duration,
    pub memory_used: Option<usize>,
    pub parallel: bool,
}
```

Stores the results of a single benchmark run, including:

- Algorithm name and configuration
- Input data size
- Execution time
- Memory usage (if measurable)
- Whether parallel processing was used

### BenchmarkRunner

The main benchmarking engine that orchestrates test execution and result collection.

## Supported Algorithms

### Sorting Algorithms

- **Merge Sort**: Both sequential and parallel implementations
- **Quick Sort**: Both sequential and parallel implementations

The benchmark runner executes multiple runs of each algorithm to calculate average performance.

### Matrix Multiplication

- **Standard Multiplication**: Traditional O(n³) algorithm
- **Strassen’s Algorithm**: Optimized divide-and-conquer approach

### Computational Geometry

- **Closest Pair Problem**: Divide-and-conquer solution for finding the closest pair of points

## Key Methods

### `benchmark_sort()`

```rust
pub fn benchmark_sort(&mut self, algorithm: &str, data: &[i32], runs: usize, parallel: bool)
```

Benchmarks sorting algorithms with multiple runs for statistical accuracy. Measures both execution time and memory usage.

### `benchmark_matrix_multiply()`

```rust
pub fn benchmark_matrix_multiply(&mut self, algorithm: &str, matrix_a: &Matrix, matrix_b: &Matrix, use_strassen: bool)
```

Benchmarks matrix multiplication algorithms, supporting both standard and Strassen implementations.

### `benchmark_closest_pair()`

```rust
pub fn benchmark_closest_pair(&mut self, algorithm: &str, points: &[Point])
```

Benchmarks the closest pair problem using divide-and-conquer approach.

## Memory Measurement

The framework uses the `memory-stats` crate to monitor physical memory usage:

- Measures memory before and after algorithm execution
- Calculates the difference to determine algorithm-specific memory usage
- Handles cases where memory measurement may not be available

## Output and Analysis

### Console Display

- Real-time progress indicators during benchmarking
- Colored output for enhanced readability
- Grouped results by algorithm type
- Automatic identification of best-performing algorithms

### Export Formats

**JSON Export**:

```rust
pub fn save_results(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>>
```

**CSV Export**:

```rust
pub fn save_results_csv(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>>
```

The CSV format includes columns for:

- Algorithm name
- Data size
- Execution time (milliseconds)
- Memory usage (megabytes)
- Parallel processing flag

## Usage Example

```rust
let mut runner = BenchmarkRunner::new();

// Benchmark sorting algorithms
let test_data = vec![/* your data */];
runner.benchmark_sort("Merge Sort", &test_data, 10, false);
runner.benchmark_sort("Merge Sort", &test_data, 10, true);

// Display results
runner.display_results();

// Save results
runner.save_results_csv("benchmark_results.csv")?;
```

## Performance Analysis Features

- **Average Calculation**: Multiple runs provide statistical reliability
- **Best Performance Identification**: Automatically highlights the fastest algorithm
- **Memory Efficiency Tracking**: Monitors memory consumption patterns
- **Comparative Analysis**: Groups results for easy algorithm comparison

## Error Handling

The framework includes robust error handling for:

- Unknown algorithm names (panics with descriptive messages)
- File I/O operations during result export
- Memory measurement failures (gracefully handles unavailable measurements)

## Thread Safety

The benchmark runner is designed for single-threaded use but can benchmark multi-threaded algorithms safely through the parallel algorithm implementations.

This benchmarking framework provides a comprehensive solution for algorithm performance analysis, offering detailed insights into execution time, memory usage, and comparative performance across different implementations and configurations.