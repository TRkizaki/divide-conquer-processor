# Experimental Methodology

## Overview

This document describes the comprehensive methodology used for benchmarking divide-and-conquer algorithms in the divide-conquer-processor project. The methodology is designed to produce statistically rigorous, reproducible results suitable for academic publication.

## System Specifications

### Hardware Configuration
- **CPU**: 13th Gen Intel(R) Core(TM) i7-13650HX
- **Cores**: 14 physical cores
- **Threads**: 20 logical threads (with hyperthreading)
- **Base Frequency**: 800 MHz
- **Maximum Frequency**: 4900 MHz
- **Memory**: 24 GB DDR4
- **Architecture**: x86_64

### Software Environment
- **Operating System**: Linux 6.12.10-76061203-generic (Pop!_OS 22.04)
- **Rust Version**: 1.89.0 (29483883e 2025-08-04)
- **Compiler Flags**: Default release mode with optimizations (-O3 equivalent)

## Benchmark Design

### 1. Algorithm Categories

#### Sorting Algorithms
- **Merge Sort**: Sequential and parallel implementations
- **Quick Sort**: Sequential and parallel implementations
- **Standard Library Comparisons**: 
  - `std::slice::sort` (stable TimSort)
  - `std::slice::sort_unstable` (introsort)
  - `std::slice::sort_by` (custom comparator)
  - `rayon::par_sort` (parallel stable sort)
  - `rayon::par_sort_unstable` (parallel unstable sort)

#### Matrix Multiplication
- **Standard Algorithm**: O(n³) implementation
- **Strassen Algorithm**: Divide-and-conquer approach

#### Computational Geometry
- **Closest Pair Problem**: Divide-and-conquer solution

### 2. Data Generation

#### Random Data
- **Uniform Distribution**: Integer values in range [0, n]
- **Seed**: Fixed for reproducibility
- **Data Types**: 32-bit signed integers for sorting, floating-point for geometry

#### Matrix Generation
- **Size**: Square matrices of varying dimensions
- **Values**: Random floating-point numbers in range [0.0, 1.0]

#### Point Generation
- **Distribution**: Uniform random points in 2D plane
- **Coordinates**: Floating-point values in range [0.0, 1000.0]

### 3. Measurement Methodology

#### Performance Metrics

**Execution Time**:
- Measured using `std::time::Instant`
- High-resolution timing with nanosecond precision
- Multiple runs to calculate statistical measures

**Memory Usage**:
- Physical memory consumption via `memory-stats` crate
- Measured before and after algorithm execution
- Delta calculation to isolate algorithm-specific usage

#### Statistical Analysis

**Multiple Runs**:
- Default: 10 runs per algorithm/data size combination
- Extended: Up to 50 runs for critical measurements
- Outlier detection and handling

**Statistical Measures**:
- **Mean (μ)**: Average execution time
- **Standard Deviation (σ)**: Measure of variability
- **Median**: Middle value, robust to outliers
- **Min/Max**: Range of observations
- **Confidence Intervals**: 95% confidence level

#### Data Size Ranges

**Standard Benchmarks**:
- Small: 1,000 elements
- Medium: 5,000 - 10,000 elements  
- Large: 25,000 - 50,000 elements

**Extended Scalability**:
- Very Large: 100,000 - 1,000,000 elements
- Memory-limited by available system resources

### 4. Parallel Performance Analysis

#### Thread Scaling
- **Thread Counts**: 1, 2, 4, 8, 14, 20 threads
- **Mapping**: Covers single-thread to full system capacity
- **Load Balancing**: Rayon work-stealing implementation

#### Efficiency Metrics
- **Speedup**: T₁ / Tₚ where T₁ is sequential time, Tₚ is parallel time
- **Efficiency**: Speedup / P where P is number of processors
- **Parallel Efficiency**: Percentage of ideal speedup achieved

### 5. Comparative Analysis

#### Baseline Comparisons
- **Standard Library**: Rust std library implementations
- **Industry Standard**: Rayon parallel implementations
- **Cross-Algorithm**: Performance ratios between different approaches

#### Performance Ratios
- **Speedup vs Sequential**: Parallel implementation improvement
- **Speedup vs Standard Library**: Custom implementation vs std
- **Memory Efficiency**: Memory usage per operation

## Data Collection Protocol

### 1. Environment Preparation
- System idle state verification
- Background process minimization
- CPU frequency scaling disabled during benchmarks
- Memory pre-allocation to reduce allocation overhead

### 2. Execution Protocol
- **Warm-up Runs**: 3 warm-up iterations before measurement
- **Data Isolation**: Fresh data copy for each run
- **Memory Barriers**: Explicit memory barriers between runs
- **Garbage Collection**: Manual cleanup between iterations

### 3. Data Validation
- **Correctness Verification**: Algorithm output validation
- **Sorting Verification**: Array sorted order confirmation
- **Stability Testing**: Equal element relative order preservation

## Output Formats

### 1. Structured Data (JSON)
Complete experimental data with:
- System specifications
- Individual run results
- Statistical summaries
- Metadata and timestamps

### 2. Tabular Data (CSV)
Optimized for statistical analysis:
- **Detailed Results**: Per-run measurements
- **Scalability Data**: Performance vs data size
- **Parallel Efficiency**: Performance vs thread count

### 3. Summary Reports
Human-readable performance summaries with:
- Best-performing algorithms
- Statistical significance tests
- Performance recommendations

## Reproducibility Requirements

### 1. Version Control
- **Git Commit Hash**: Exact source code version
- **Dependency Versions**: Locked Cargo.toml dependencies
- **Compiler Version**: Rust toolchain specification

### 2. System Requirements
- **Minimum Memory**: 8GB RAM recommended
- **CPU Cores**: 4+ cores for parallel analysis
- **Disk Space**: 1GB for extended benchmark data

### 3. Execution Commands
```bash
# Standard benchmark
cargo run --release -- publication --runs 10

# Extended scalability analysis  
cargo run --release -- publication --runs 20 --extended

# Quick validation run
cargo run --release -- publication --runs 3
```

## Statistical Significance

### 1. Confidence Intervals
- **Level**: 95% confidence intervals reported
- **Method**: Student's t-distribution for small samples
- **Interpretation**: True mean lies within interval with 95% probability

### 2. Outlier Detection
- **Method**: Interquartile Range (IQR) method
- **Threshold**: Values beyond Q1 - 1.5×IQR or Q3 + 1.5×IQR
- **Handling**: Outliers reported but not excluded from analysis

### 3. Effect Size
- **Cohen's d**: Standardized difference between means
- **Practical Significance**: >10% performance difference threshold
- **Statistical Power**: Minimum 80% power for detection

## Quality Assurance

### 1. Code Quality
- **Compiler Warnings**: Zero warnings in release build
- **Static Analysis**: Clippy lints enforced
- **Testing**: Unit tests for all algorithms
- **Documentation**: Comprehensive inline documentation

### 2. Measurement Validation
- **Timer Resolution**: Nanosecond precision verification
- **Overhead Measurement**: Timing infrastructure overhead quantified
- **Platform Effects**: OS scheduler impact minimization

### 3. Result Validation
- **Cross-Validation**: Multiple measurement methods
- **Sanity Checks**: Performance trends validation
- **Literature Comparison**: Results consistent with published research

## Limitations and Considerations

### 1. Hardware Limitations
- **Single Platform**: Results specific to Intel x86_64 architecture
- **Memory Constraints**: Large datasets limited by available RAM
- **Thermal Throttling**: Potential CPU frequency reduction under load

### 2. Software Limitations
- **Rust-Specific**: Results applicable to Rust implementations
- **Compiler Optimizations**: Performance dependent on Rust compiler
- **System Load**: Background processes may affect measurements

### 3. Statistical Limitations
- **Sample Size**: Limited by computational time constraints
- **Distribution Assumptions**: Normal distribution assumed for statistical tests
- **Independence**: Runs assumed independent (may have cache effects)

## Future Work

### 1. Extended Analysis
- **Cache Performance**: L1/L2/L3 cache miss analysis
- **NUMA Effects**: Multi-socket system performance
- **Energy Consumption**: Power efficiency measurements

### 2. Additional Algorithms
- **Parallel Algorithms**: More divide-and-conquer variants
- **Hybrid Approaches**: Algorithm combination strategies
- **Adaptive Methods**: Runtime algorithm selection

### 3. Platform Expansion
- **ARM Architecture**: Apple M1/M2, ARM server processors
- **GPU Acceleration**: CUDA/OpenCL implementations
- **Distributed Computing**: Multi-node implementations

## References

1. **Cormen, T. H., et al.** (2009). Introduction to Algorithms, Third Edition. MIT Press.
2. **Rayon Documentation**: https://docs.rs/rayon/
3. **Rust Performance Book**: https://nnethercote.github.io/perf-book/
4. **Intel VTune Profiler**: Performance analysis methodology
5. **SPEC CPU Benchmarks**: Industry standard benchmarking practices

## Appendix: Statistical Methods

### Confidence Interval Calculation
For sample mean x̄ with standard deviation s and sample size n:
CI = x̄ ± t_{α/2,n-1} × (s/√n)

### Speedup Calculation
Speedup = T_sequential / T_parallel

### Efficiency Calculation  
Efficiency = Speedup / Number_of_Processors

### Effect Size (Cohen's d)
d = (μ₁ - μ₂) / σ_pooled

Where σ_pooled is the pooled standard deviation of both samples.