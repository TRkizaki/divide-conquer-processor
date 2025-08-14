# High-Performance Divide and Conquer Algorithms

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.89.0+-orange.svg)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/TRkizaki/divide-conquer-processor)

A comprehensive implementation and analysis of divide-and-conquer algorithms for large-scale data processing, featuring parallel implementations, extensive benchmarking, and publication-quality performance analysis.

## Overview

This project provides high-performance implementations of fundamental divide-and-conquer algorithms with extensive parallel optimization and rigorous performance analysis. Designed for both educational and research purposes, it includes publication-ready benchmarking tools and comprehensive documentation.

## Key Features

### Algorithm Implementations
- **Sorting Algorithms**: Merge Sort and Quick Sort with sequential and parallel variants
- **Matrix Operations**: Standard and Strassen multiplication algorithms
- **Computational Geometry**: Closest pair problem using divide-and-conquer
- **Performance Optimizations**: Multi-threaded implementations using Rayon

### Comprehensive Benchmarking
- **Statistical Analysis**: Multiple runs with confidence intervals and effect sizes
- **Memory Monitoring**: Real-time memory usage tracking and analysis
- **Scalability Testing**: Performance analysis across data sizes (1K → 1M+ elements)
- **Parallel Efficiency**: Thread scaling analysis (1-20+ cores)
- **Comparative Analysis**: Benchmarks against Rust standard library

### Publication-Quality Results
- **Structured Data Export**: JSON and CSV formats for statistical analysis
- **Reproducible Research**: Complete methodology documentation
- **System Specifications**: Detailed hardware/software configuration
- **Statistical Rigor**: Confidence intervals, outlier detection, effect sizes

## Project Structure

```
divide-conquer-processor/
├── src/
│   ├── main.rs                      # CLI interface and command handling
│   ├── sorting.rs                   # Divide-and-conquer sorting algorithms
│   ├── matrix.rs                    # Matrix multiplication implementations
│   ├── geometry.rs                  # Computational geometry algorithms
│   ├── benchmark.rs                 # Basic benchmarking framework
│   ├── comprehensive_benchmark.rs   # Publication-quality benchmarking
│   ├── data_generator.rs           # Test data generation utilities
│   └── visualization.rs            # Performance visualization tools
├── METHODOLOGY.md                   # Detailed experimental methodology
├── REPRODUCIBILITY_GUIDE.md        # Complete reproduction instructions
├── REPORT_FOR_PROJECT.md          # Comprehensive project analysis
└── Publication Data/               # Generated benchmark results
    ├── publication_benchmark_full_report.json
    ├── publication_benchmark_detailed_results.csv
    ├── publication_benchmark_scalability.csv
    └── publication_benchmark_parallel_efficiency.csv
```

## Technology Stack

- **Language**: Rust 1.89.0+ (stable)
- **Parallel Processing**: [Rayon](https://github.com/rayon-rs/rayon) work-stealing framework
- **Benchmarking**: [Criterion](https://github.com/bheisler/criterion.rs) statistical benchmarking
- **Serialization**: [Serde](https://serde.rs/) for structured data export
- **Memory Monitoring**: [memory-stats](https://crates.io/crates/memory-stats) for runtime analysis
- **Statistical Analysis**: Custom implementation with confidence intervals

## Quick Start

### System Requirements

- **Rust**: 1.89.0+ (latest stable recommended)
- **Memory**: 8GB+ RAM (16GB+ for extended benchmarks)
- **CPU**: Multi-core processor (4+ cores recommended)
- **OS**: Linux, macOS, or Windows (Linux preferred for best performance)

### Installation

```bash
# Clone the repository
git clone https://github.com/TRkizaki/divide-conquer-processor
cd divide-conquer-processor

# Build in release mode
cargo build --release
```

### Running Benchmarks

```bash
# Quick validation (3 runs per test)
cargo run --release -- publication --runs 3

# Standard publication benchmark (10 runs)
cargo run --release -- publication --runs 10

# Extended analysis with large datasets
cargo run --release -- publication --runs 20 --extended

# Individual algorithm tests
cargo run --release -- sort --size 50000 --runs 10 --parallel
cargo run --release -- matrix --size 512 --strassen
cargo run --release -- geometry --points 10000
```

### Command Line Interface

```bash
# Available commands
cargo run --release -- --help

# Sort algorithms benchmark
cargo run --release -- sort [OPTIONS]
  --size <SIZE>        Data size [default: 10000]
  --runs <RUNS>        Number of runs [default: 5]
  --parallel           Enable parallel processing

# Matrix multiplication benchmark
cargo run --release -- matrix [OPTIONS]
  --size <SIZE>        Matrix size (N x N) [default: 512]
  --strassen           Use Strassen algorithm

# Computational geometry benchmark
cargo run --release -- geometry [OPTIONS]
  --points <POINTS>    Number of points [default: 10000]

# Comprehensive benchmarks
cargo run --release -- all [--small]
cargo run --release -- publication [--runs <RUNS>] [--extended]
```

## Performance Results

### Key Findings

**Parallel Speedup (50K elements, 14 cores):**
- **Merge Sort**: 18.46x speedup (131.83% efficiency)
- **Quick Sort**: 16.46x speedup (117.59% efficiency)

**Memory Scaling:**
- Linear memory usage with input size
- Efficient memory management in parallel implementations

**Comparative Performance:**
- Competitive with Rust standard library on large datasets
- Superior parallel scaling compared to sequential implementations
- Optimal performance on datasets > 25K elements

## Generated Data Files

After running publication benchmarks, the following files are generated:

- **`publication_benchmark_full_report.json`** - Complete structured dataset
- **`publication_benchmark_detailed_results.csv`** - Statistical performance metrics
- **`publication_benchmark_scalability.csv`** - Performance vs data size analysis
- **`publication_benchmark_parallel_efficiency.csv`** - Thread scaling analysis

## Documentation

### Research Documentation
- **[METHODOLOGY.md](./METHODOLOGY.md)** - Detailed experimental methodology
- **[REPRODUCIBILITY_GUIDE.md](./REPRODUCIBILITY_GUIDE.md)** - Complete reproduction instructions
- **[REPORT_FOR_PROJECT.md](./REPORT_FOR_PROJECT.md)** - Comprehensive project analysis

### Implementation Documentation
- **[benchmark.md](./benchmark.md)** - Benchmarking framework implementation
- **[sorting.md](./sorting.md)** - Divide-and-conquer sorting algorithms
- **[matrix.md](./matrix.md)** - Matrix multiplication implementations
- **[geometry.md](./geometry.md)** - Computational geometry algorithms
- **[visualization.md](./visualization.md)** - Performance visualization tools
- **[rayon.md](./rayon.md)** - Parallel processing with Rayon

## Testing

```bash
# Run unit tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test sorting::tests
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Citation

When using this work in academic research, please cite:

```bibtex
@software{divide_conquer_processor,
  title={High-Performance Divide and Conquer Algorithms for Large-Scale Data Processing},
  author={TETSUROU KIZAKI},
  year={2025},
  url={https://github.com/TRkizaki/divide-conquer-processor},
  version={0.1.0}
}
```

## Contributing

Contributions are welcome! Please read the methodology documentation to understand the performance standards and testing requirements.

## Acknowledgments

- Advanced Algorithms Master's Program 2024-2025
- Rust community for excellent parallel processing tools
- Rayon team for the work-stealing framework
