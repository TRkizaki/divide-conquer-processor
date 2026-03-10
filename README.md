# High-Performance Divide and Conquer Algorithms

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.89.0+-orange.svg)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/TRkizaki/divide-conquer-processor)

A comprehensive implementation and analysis of divide-and-conquer algorithms for large-scale data processing, featuring parallel implementations, extensive benchmarking, and publication-quality performance analysis.

## Overview

This project provides high-performance implementations of fundamental divide-and-conquer algorithms with extensive parallel optimization and rigorous performance analysis. Designed for both educational and research purposes, it includes publication-ready benchmarking tools and comprehensive documentation.

## Key Features

### Algorithm Implementations
- **Sorting Algorithms**: Merge Sort and Quick Sort with sequential and parallel variants (true parallel D&C via `rayon::join()`)
- **Advanced Matrix Operations**: Standard, Strassen, Winograd, Cache-optimized, SIMD, and Parallel algorithms
- **Computational Geometry**: Closest pair problem using divide-and-conquer
- **Performance Optimizations**: Multi-threaded implementations using Rayon with SIMD vectorization

### Comprehensive Benchmarking Framework
- **Performance Benchmarking**: Time and memory analysis with statistical reliability
- **Library Comparison**: Head-to-head benchmarks against Rust std library, Rayon parallel sorts, and ndarray
- **Thread Affinity Analysis**: P-core/E-core separation and core-binding experiments (Intel hybrid architectures)
- **Advanced Hardware Analysis**: Hardware-aware performance analysis (cache, energy, NUMA effects)
- **Cross-Platform Validation**: Statistical validation across data distributions and compiler optimizations
- **Scalability Testing**: Performance analysis across data sizes (1K → 10M elements)
- **Parallel Efficiency**: Thread scaling analysis (1-24+ cores with extended testing)

### Advanced Performance Analysis
- **Hardware Metrics**: L1/L2/L3 cache performance, energy consumption via Intel RAPL
- **Statistical Validation**: ANOVA analysis, confidence intervals, empirical constant analysis
- **Cross-Platform Testing**: Data distribution sensitivity, compiler optimization impact
- **Publication-Quality SVG Figures**: Automated generation of 8 publication-ready charts from benchmark data
- **Publication-Quality Results**: Structured data export in JSON and CSV formats

## Project Structure

```
divide-conquer-processor/
├── src/
│   ├── main.rs                           # CLI interface with advanced commands
│   ├── sorting.rs                        # Divide-and-conquer sorting algorithms
│   ├── matrix.rs                         # Advanced matrix implementations (7+ algorithms)
│   ├── geometry.rs                       # Computational geometry algorithms
│   ├── benchmark.rs                      # Benchmarking framework
│   ├── comprehensive_benchmark.rs        # Publication-quality benchmarking suite
│   ├── advanced_benchmark.rs             # Hardware-aware performance analysis
│   ├── library_comparison.rs             # Benchmarks vs std/Rayon/ndarray
│   ├── thread_affinity.rs                # P-core/E-core affinity experiments
│   ├── cross_platform_validation.rs      # Statistical validation framework
│   ├── publication_figures.rs            # SVG figure generation for papers
│   ├── data_generator.rs                 # Test data generation utilities
│   └── visualization.rs                  # Performance visualization tools
├── Documentation/
│   ├── Core/
│   │   ├── sorting.md                    # Sorting algorithm implementations
│   │   ├── matrix.md                     # Matrix operations
│   │   ├── geometry.md                   # Computational geometry
│   │   ├── benchmark.md                  # Basic benchmarking framework
│   │   ├── comprehensive_benchmark.md    # Publication-quality benchmarking
│   │   ├── library_comparison.md         # Benchmarks vs std/Rayon/ndarray
│   │   ├── thread_affinity.md            # P-core/E-core affinity analysis
│   │   ├── visualization.md              # Performance visualization
│   │   └── rayon.md                      # Parallel processing framework
│   ├── Advanced/
│   │   ├── advanced_benchmark.md         # Hardware-aware benchmarking
│   │   ├── Advanced_matrix.md            # Advanced matrix algorithms
│   │   └── Cross-Platform_Validation.md  # Cross-platform validation
│   └── Research/
│       ├── METHODOLOGY.md                # Detailed experimental methodology
│       ├── REPRODUCIBILITY_GUIDE.md      # Complete reproduction instructions
│       └── REPORT_FOR_PROJECT.md         # Comprehensive project analysis
├── Generated_Data/                        # Benchmark results and validation data
│   ├── Publication_Benchmarks/
│   │   ├── publication_benchmark_full_report.json
│   │   ├── publication_benchmark_detailed_results.csv
│   │   ├── publication_benchmark_scalability.csv
│   │   └── publication_benchmark_parallel_efficiency.csv
│   ├── Library_Comparisons/
│   │   ├── library_comparison.json
│   │   └── library_comparison.csv
│   ├── Affinity_Benchmarks/
│   │   ├── affinity_benchmark_affinity_report.json
│   │   └── affinity_benchmark_affinity_results.csv
│   ├── Advanced_Benchmarks/
│   │   ├── advanced_benchmark_advanced_benchmark.json
│   │   └── advanced_benchmark_advanced_metrics.csv
│   ├── Cross-Platform_validation/
│   │   ├── cross_platform_validation_validation_report.json
│   │   ├── cross_platform_validation_validation_results.csv
│   │   ├── cross_platform_validation_distribution_analysis.csv
│   │   ├── cross_platform_validation_optimization_impact.csv
│   │   └── cross_platform_validation_thread_scaling.csv
│   └── Figures/                           # Publication-quality SVG charts
│       ├── fig2_sorting_speedup.svg
│       ├── fig3_matrix_comparison.svg
│       ├── fig4_thread_scaling.svg
│       ├── fig5_parallel_efficiency.svg
│       ├── fig6_sorting_library_comparison.svg
│       ├── fig7_compiler_optimization.svg
│       └── fig8_distribution_sensitivity.svg
└── ALGORITHM.md                           # Algorithm documentation
```

## Technology Stack

- **Language**: Rust 1.89.0+ (stable)
- **CLI Framework**: [clap](https://crates.io/crates/clap) with derive macros
- **Parallel Processing**: [Rayon](https://github.com/rayon-rs/rayon) work-stealing framework
- **Thread Affinity**: [core_affinity](https://crates.io/crates/core_affinity) for CPU core binding
- **Chart Generation**: [plotters](https://crates.io/crates/plotters) for publication-quality SVG figures
- **Library Comparison**: [ndarray](https://crates.io/crates/ndarray) for matrix operation benchmarks
- **Hardware Monitoring**: [sysinfo](https://crates.io/crates/sysinfo) for system metrics
- **Performance Counters**: [perf-event](https://crates.io/crates/perf-event) for cache analysis
- **SIMD Optimization**: AVX2 vectorization for x86_64 architectures
- **Serialization**: [Serde](https://serde.rs/) for structured data export (JSON + CSV)
- **Memory Monitoring**: [memory-stats](https://crates.io/crates/memory-stats) for runtime analysis
- **Statistical Analysis**: [statistical](https://crates.io/crates/statistical) + custom ANOVA with confidence intervals
- **Benchmarking**: [Criterion](https://crates.io/crates/criterion) with HTML reports
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

# Library comparison (vs std, Rayon, ndarray)
cargo run --release -- compare --runs 10

# Thread affinity analysis (P-core/E-core)
cargo run --release -- affinity --size 500000 --runs 10 --scaling

# Advanced hardware-aware benchmarking
cargo run --release -- advanced --runs 5 --sizes 1000 5000 10000

# Cross-platform validation
cargo run --release -- validate --runs 3 --optimization --threading

# Publication-quality comprehensive benchmarks
cargo run --release -- publication --runs 10
cargo run --release -- publication --runs 20 --extended

# Generate publication-quality SVG figures
cargo run --release -- figures
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

# Library comparison benchmarks
cargo run --release -- compare [OPTIONS]
  --runs <RUNS>        Number of runs [default: 5]

# Thread affinity analysis
cargo run --release -- affinity [OPTIONS]
  --size <SIZE>        Data size [default: 500000]
  --runs <RUNS>        Number of runs [default: 5]
  --scaling            Enable thread scaling analysis

# Parallel threshold discovery
cargo run --release -- threshold [OPTIONS]
  --size <SIZE>        Data size [default: 1000000]
  --runs <RUNS>        Number of runs [default: 5]

# Advanced performance analysis
cargo run --release -- advanced [OPTIONS]
  --runs <RUNS>        Number of runs [default: 3]
  --sizes <SIZES>      Data sizes to test [default: 1000 5000 10000]
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

# Publication-quality SVG figure generation
cargo run --release -- figures
```

## Performance Results

### Key Findings

**Parallel Speedup (50K elements, optimal 8 threads):**
- **Merge Sort**: 14.8x speedup (185% efficiency)
- **Quick Sort**: 10.2x speedup (127% efficiency)

**Note**: Tested on Intel i7-13650HX (6 P-cores + 8 E-cores, 20 logical threads). Optimal performance achieved at 8 threads.

**Thread Affinity (P-core vs E-core):**
- P-core-only configurations show highest per-thread performance
- E-core-only scaling effective for throughput workloads
- Rayon worker threads bound to cores via `start_handler` for reproducible results

**Library Comparison:**
- Sorting benchmarked against `std::sort`, `std::sort_unstable`, and Rayon parallel sorts
- Matrix operations benchmarked against ndarray
- Honest comparison showing where custom implementations excel and where std library wins

**Memory Scaling:**
- Linear memory usage with input size
- Efficient memory management in parallel implementations

## Generated Data Files

All benchmark data is exported to `Generated_Data/` in JSON and CSV formats:

### Publication Benchmarks (`Generated_Data/Publication_Benchmarks/`)
- **`publication_benchmark_full_report.json`** - Complete structured dataset
- **`publication_benchmark_detailed_results.csv`** - Statistical performance metrics
- **`publication_benchmark_scalability.csv`** - Performance vs data size analysis
- **`publication_benchmark_parallel_efficiency.csv`** - Thread scaling analysis

### Library Comparisons (`Generated_Data/Library_Comparisons/`)
- **`library_comparison.json`** - Structured comparison results
- **`library_comparison.csv`** - Sorting and matrix performance vs std/Rayon/ndarray

### Thread Affinity Benchmarks (`Generated_Data/Affinity_Benchmarks/`)
- **`affinity_benchmark_affinity_report.json`** - P-core/E-core analysis report
- **`affinity_benchmark_affinity_results.csv`** - Core-specific performance data

### Advanced Performance Analysis (`Generated_Data/Advanced_Benchmarks/`)
- **`advanced_benchmark_advanced_benchmark.json`** - Hardware-aware metrics
- **`advanced_benchmark_advanced_metrics.csv`** - Cache, energy, and NUMA data

### Cross-Platform Validation (`Generated_Data/Cross-Platform_validation/`)
- **`cross_platform_validation_validation_report.json`** - Comprehensive validation report
- **`cross_platform_validation_validation_results.csv`** - Statistical validation data
- **`cross_platform_validation_distribution_analysis.csv`** - Data distribution performance
- **`cross_platform_validation_optimization_impact.csv`** - Compiler optimization analysis
- **`cross_platform_validation_thread_scaling.csv`** - Extended threading analysis

### Publication Figures (`Generated_Data/Figures/`)
Generated via `cargo run --release -- figures`:
- **`fig2_sorting_speedup.svg`** - Parallel speedup vs data size
- **`fig3_matrix_comparison.svg`** - Matrix algorithm comparison bar chart
- **`fig4_thread_scaling.svg`** - Thread scaling with core affinity
- **`fig5_parallel_efficiency.svg`** - Efficiency percentage vs thread count
- **`fig6_sorting_library_comparison.svg`** - Custom vs std vs Rayon
- **`fig7_compiler_optimization.svg`** - Optimization level (O0-O3) impact
- **`fig8_distribution_sensitivity.svg`** - Data distribution sensitivity

## Documentation

### Research Documentation
- **[METHODOLOGY.md](./Documentation/Research/METHODOLOGY.md)** - Detailed experimental methodology
- **[REPRODUCIBILITY_GUIDE.md](./Documentation/Research/REPRODUCIBILITY_GUIDE.md)** - Complete reproduction instructions
- **[REPORT_FOR_PROJECT.md](./Documentation/Research/REPORT_FOR_PROJECT.md)** - Comprehensive project analysis

### Implementation Documentation

#### Core Implementations
- **[sorting.md](./Documentation/Core/sorting.md)** - Divide-and-conquer sorting algorithms
- **[matrix.md](./Documentation/Core/matrix.md)** - Matrix operations and API
- **[geometry.md](./Documentation/Core/geometry.md)** - Computational geometry algorithms
- **[benchmark.md](./Documentation/Core/benchmark.md)** - Basic benchmarking framework
- **[comprehensive_benchmark.md](./Documentation/Core/comprehensive_benchmark.md)** - Publication-quality benchmarking suite
- **[library_comparison.md](./Documentation/Core/library_comparison.md)** - Benchmarks vs std/Rayon/ndarray
- **[thread_affinity.md](./Documentation/Core/thread_affinity.md)** - P-core/E-core affinity experiments
- **[visualization.md](./Documentation/Core/visualization.md)** - Performance visualization tools
- **[rayon.md](./Documentation/Core/rayon.md)** - Parallel processing with Rayon

#### Advanced Performance Analysis
- **[advanced_benchmark.md](./Documentation/Advanced/advanced_benchmark.md)** - Hardware-aware performance analysis
- **[Advanced_matrix.md](./Documentation/Advanced/Advanced_matrix.md)** - Advanced matrix algorithms (Strassen, SIMD, etc.)
- **[Cross-Platform_Validation.md](./Documentation/Advanced/Cross-Platform_Validation.md)** - Cross-platform validation framework

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
  year={2025--2026},
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
