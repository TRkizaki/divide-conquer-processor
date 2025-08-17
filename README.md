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
- **Advanced Matrix Operations**: Standard, Strassen, Winograd, Cache-optimized, SIMD, and Parallel algorithms
- **Computational Geometry**: Closest pair problem using divide-and-conquer
- **Performance Optimizations**: Multi-threaded implementations using Rayon with SIMD vectorization

### Comprehensive Benchmarking Framework
- **Performance Benchmarking**: Time and memory analysis with statistical reliability
- **Advanced Hardware Analysis**: Hardware-aware performance analysis (cache, energy, NUMA effects)
- **Cross-Platform Validation**: Statistical validation across data distributions and compiler optimizations
- **Scalability Testing**: Performance analysis across data sizes (1K → 1M+ elements)
- **Parallel Efficiency**: Thread scaling analysis (1-24+ cores with extended testing)

### Advanced Performance Analysis
- **Hardware Metrics**: L1/L2/L3 cache performance, energy consumption via Intel RAPL
- **Statistical Validation**: ANOVA analysis, confidence intervals, empirical constant analysis
- **Cross-Platform Testing**: Data distribution sensitivity, compiler optimization impact
- **Publication-Quality Results**: Structured data export with comprehensive statistical measures

## Project Structure

```
divide-conquer-processor/
├── src/
│   ├── main.rs                           # CLI interface with advanced commands
│   ├── sorting.rs                        # Divide-and-conquer sorting algorithms
│   ├── matrix.rs                         # Advanced matrix implementations (7+ algorithms)
│   ├── geometry.rs                       # Computational geometry algorithms
│   ├── benchmark.rs                      # Benchmarking framework
│   ├── advanced_benchmark.rs             # Hardware-aware performance analysis
│   ├── cross_platform_validation.rs     # Statistical validation framework
│   ├── comprehensive_benchmark.rs        # Comprehensive benchmarking
│   ├── data_generator.rs                # Test data generation utilities
│   └── visualization.rs                 # Performance visualization tools
├── Documentation/
│   ├── Core/
│   │   ├── benchmark.md                  # Benchmarking framework
│   │   ├── matrix.md                     # Matrix operations
│   │   ├── sorting.md                    # Sorting algorithm implementations
│   │   ├── geometry.md                   # Computational geometry
│   │   ├── visualization.md              # Performance visualization
│   │   └── rayon.md                      # Parallel processing framework
│   ├── Advanced/
│   │   ├── advanced_benchmark.md         # Hardware-aware benchmarking
│   │   ├── Advanced_matrix.md            # Advanced matrix algorithms
│   │   └── Cross-Platform_Validation.md # Cross-platform validation
│   └── Research/
│       ├── METHODOLOGY.md               # Detailed experimental methodology
│       ├── REPRODUCIBILITY_GUIDE.md     # Complete reproduction instructions
│       └── REPORT_FOR_PROJECT.md        # Comprehensive project analysis
└── Generated Data/                       # Benchmark results and validation data
    ├── Publication Benchmarks/
    │   ├── publication_benchmark_full_report.json
    │   ├── publication_benchmark_detailed_results.csv
    │   ├── publication_benchmark_scalability.csv
    │   └── publication_benchmark_parallel_efficiency.csv
    ├── Advanced Benchmarks/
    │   ├── advanced_benchmark_advanced_benchmark.json
    │   └── advanced_benchmark_advanced_metrics.csv
    └── Cross-Platform Validation/
        ├── cross_platform_validation_validation_report.json
        ├── cross_platform_validation_validation_results.csv
        ├── cross_platform_validation_distribution_analysis.csv
        ├── cross_platform_validation_optimization_impact.csv
        └── cross_platform_validation_thread_scaling.csv
```

## Technology Stack

- **Language**: Rust 1.89.0+ (stable)
- **Parallel Processing**: [Rayon](https://github.com/rayon-rs/rayon) work-stealing framework
- **Hardware Monitoring**: [sysinfo](https://crates.io/crates/sysinfo) for system metrics
- **Performance Counters**: [perf-event](https://crates.io/crates/perf-event) for cache analysis
- **SIMD Optimization**: AVX2 vectorization for x86_64 architectures
- **Serialization**: [Serde](https://serde.rs/) for structured data export
- **Memory Monitoring**: [memory-stats](https://crates.io/crates/memory-stats) for runtime analysis
- **Statistical Analysis**: Custom ANOVA implementation with confidence intervals
- **Energy Monitoring**: Intel RAPL for power consumption measurement

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
# Algorithm benchmarks
cargo run --release -- sort --size 50000 --runs 10 --parallel
cargo run --release -- matrix --size 512 --strassen
cargo run --release -- geometry --points 10000

# Advanced hardware-aware benchmarking
cargo run --release -- advanced --runs 5 --cache --energy --numa

# Cross-platform validation
cargo run --release -- validate --runs 3 --optimization --threading

# Publication-quality comprehensive benchmarks
cargo run --release -- publication --runs 10
cargo run --release -- publication --runs 20 --extended

# Quick validation (3 runs per test)
cargo run --release -- publication --runs 3
```

### Command Line Interface

```bash
# Available commands
cargo run --release -- --help

# Algorithm benchmarks
cargo run --release -- sort [OPTIONS]
  --size <SIZE>        Data size [default: 10000]
  --runs <RUNS>        Number of runs [default: 5]
  --parallel           Enable parallel processing

cargo run --release -- matrix [OPTIONS]
  --size <SIZE>        Matrix size (N x N) [default: 512]
  --strassen           Use Strassen algorithm

cargo run --release -- geometry [OPTIONS]
  --points <POINTS>    Number of points [default: 10000]

# Advanced performance analysis
cargo run --release -- advanced [OPTIONS]
  --runs <RUNS>        Number of runs [default: 3]
  --cache              Enable cache performance analysis
  --energy             Enable energy consumption monitoring
  --numa               Enable NUMA effects analysis

# Cross-platform validation
cargo run --release -- validate [OPTIONS]
  --runs <RUNS>        Number of runs per test [default: 2]
  --optimization       Enable compiler optimization analysis
  --threading          Enable extended thread scaling analysis

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

The framework generates comprehensive benchmark data in multiple formats:

### Publication Benchmarks
- **`publication_benchmark_full_report.json`** - Complete structured dataset
- **`publication_benchmark_detailed_results.csv`** - Statistical performance metrics
- **`publication_benchmark_scalability.csv`** - Performance vs data size analysis
- **`publication_benchmark_parallel_efficiency.csv`** - Thread scaling analysis

### Advanced Performance Analysis
- **`advanced_benchmark_advanced_benchmark.json`** - Hardware-aware metrics
- **`advanced_benchmark_advanced_metrics.csv`** - Cache, energy, and NUMA data

### Cross-Platform Validation Results
- **`cross_platform_validation_validation_report.json`** - Comprehensive validation report
- **`cross_platform_validation_validation_results.csv`** - Statistical validation data
- **`cross_platform_validation_distribution_analysis.csv`** - Data distribution performance
- **`cross_platform_validation_optimization_impact.csv`** - Compiler optimization analysis
- **`cross_platform_validation_thread_scaling.csv`** - Extended threading analysis

## Documentation

### Research Documentation
- **[METHODOLOGY.md](./METHODOLOGY.md)** - Detailed experimental methodology
- **[REPRODUCIBILITY_GUIDE.md](./REPRODUCIBILITY_GUIDE.md)** - Complete reproduction instructions
- **[REPORT_FOR_PROJECT.md](./REPORT_FOR_PROJECT.md)** - Comprehensive project analysis

### Implementation Documentation

#### Core Implementations
- **[benchmark.md](./benchmark.md)** - Benchmarking framework
- **[sorting.md](./sorting.md)** - Divide-and-conquer sorting algorithms
- **[matrix.md](./matrix.md)** - Matrix operations and API
- **[geometry.md](./geometry.md)** - Computational geometry algorithms
- **[visualization.md](./visualization.md)** - Performance visualization tools
- **[rayon.md](./rayon.md)** - Parallel processing with Rayon

#### Advanced Performance Analysis
- **[advanced_benchmark.md](./advanced_benchmark.md)** - Hardware-aware performance analysis
- **[Advanced_matrix.md](./Advanced_matrix.md)** - Advanced matrix algorithms (Strassen, SIMD, etc.)
- **[Cross-Platform_Validation.md](./Cross-Platform_Validation.md)** - Cross-platform validation framework

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
