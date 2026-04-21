# Comprehensive (Publication) Benchmark Suite

## Prerequisites
This document assumes familiarity with basic benchmarking. See [benchmark.md](benchmark.md) for the foundational framework.

---

A publication-quality benchmarking suite that performs systematic performance analysis across all algorithm categories: sorting, matrix multiplication, computational geometry, and linear algebra. This is the primary tool for generating results suitable for academic papers.

## Results Summary (JAIT Paper)

This module produced the core measurements reported in the accepted paper. Headline numbers (Intel i7-13650HX, 1M elements, 10 runs):

| Category | Result |
|---|---|
| **Parallel merge sort** | 6.35× speedup over sequential custom |
| **Parallel quick sort** (median-of-three) | 9.36× speedup over sequential custom |
| **Efficiency at 14 cores** | ~39% merge sort, ~37% quicksort (Amdahl ceiling) |
| **Optimal sequential threshold** | T* ≈ 8192 (merge sort), T* ≈ 4096 (quicksort) |
| **Measurement reliability** | mean CV < 3%, individual CVs < 5.1% |
| **ANOVA across distributions** | F = 3.71, p = 0.05, η² = 0.329 |

For the comparative numbers against Rayon/std/ndarray/MKL, see [library_comparison.md](library_comparison.md). For placement-strategy analysis on the hybrid architecture, see [thread_affinity.md](thread_affinity.md).

## Overview

The comprehensive benchmark module (`comprehensive_benchmark.rs`) orchestrates large-scale experiments with:

- Multi-run statistical analysis with mean, std dev, median, min/max
- Scalability testing across data sizes (10K to 10M elements)
- Parallel efficiency analysis with thread scaling
- Memory usage tracking
- System specification capture for reproducibility
- Structured JSON and CSV output

## CLI Usage

```bash
# Standard publication benchmark (10 runs)
cargo run --release -- publication --runs 10

# Extended benchmark (larger data sizes)
cargo run --release -- publication --runs 20 --extended

# Quick validation (3 runs)
cargo run --release -- publication --runs 3
```

## What Gets Benchmarked

### Sorting Algorithms (Sequential & Parallel)
- Merge Sort, Quick Sort
- Parallel Merge Sort, Parallel Quick Sort (via `rayon::join()`)
- Standard library comparisons: `sort()`, `sort_unstable()`

### Matrix Multiplication (7 algorithms)
- Standard O(n^3)
- Strassen O(n^2.807)
- Cache-optimized (block tiling)
- SIMD (AVX2)
- Winograd
- Parallel (Rayon)
- Parallel Winograd

### Computational Geometry
- Closest pair: brute force vs divide-and-conquer
- Convex hull (Graham scan)
- K-d tree construction and nearest-neighbor queries
- Line segment intersection detection

### Linear Algebra
- Determinant, inverse, LU/QR/Cholesky decomposition
- Linear system solving
- Matrix rank and condition number

## Key Structures

### SystemSpecs
Captures full hardware/software environment:
```rust
pub struct SystemSpecs {
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub cpu_max_freq_mhz: f64,
    pub memory_total_gb: f64,
    pub os_version: String,
    pub rust_version: String,
    pub arch: String,
}
```

### DetailedBenchmarkResult
Per-algorithm results with full statistics:
```rust
pub struct DetailedBenchmarkResult {
    pub algorithm_name: String,
    pub data_size: usize,
    pub runs: usize,
    pub mean_time_ms: f64,
    pub std_dev_time_ms: f64,
    pub min_time_ms: f64,
    pub max_time_ms: f64,
    pub median_time_ms: f64,
    pub mean_memory_mb: Option<f64>,
    pub parallel: bool,
    pub speedup_vs_sequential: Option<f64>,
    pub efficiency: Option<f64>,
    pub individual_runs_ms: Vec<f64>,
}
```

### ScalabilityResult & ParallelEfficiencyResult
Tracks performance across data sizes and thread counts for generating scaling charts.

## Analysis Types

### Sorting Comprehensive
Benchmarks all sorting algorithms at specified data sizes with configurable runs.

### Scalability Analysis
Tests algorithms across a range of sizes (default: 10K, 50K, 100K, 500K, 1M; extended adds 5M, 10M) to validate O(n log n) scaling.

### Parallel Efficiency Analysis
Measures speedup and efficiency across thread counts (1, 2, 4, 6, 8, 12, 14, 20 threads) to identify optimal parallelism.

### Matrix Comprehensive
Benchmarks all 7 matrix multiplication algorithms at sizes 64, 128, 256, 512.

### Geometry Comprehensive
Benchmarks closest pair, convex hull, K-d tree, and line segment intersection across point/segment counts.

### Linear Algebra Comprehensive
Benchmarks decomposition and solving operations at matrix sizes 64, 128, 256, 512.

## Output Files

Results are saved to the project root:

- **`publication_benchmark_full_report.json`** - Complete structured report including system specs, methodology, and all results
- **`publication_benchmark_detailed_results.csv`** - Per-algorithm performance metrics
- **`publication_benchmark_scalability.csv`** - Data size vs performance
- **`publication_benchmark_parallel_efficiency.csv`** - Thread count vs speedup/efficiency

### Full Report JSON Structure

```json
{
  "system_specs": { ... },
  "benchmark_date": "2025-...",
  "methodology": "...",
  "detailed_results": [ ... ],
  "scalability_results": [ ... ],
  "parallel_efficiency_results": [ ... ],
  "geometry_results": [ ... ],
  "matrix_results": [ ... ],
  "linear_algebra_results": [ ... ]
}
```

## Dependencies

```toml
rayon = "1.8"
memory-stats = "1.1"
statistical = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
colored = "3.0"
chrono = { version = "0.4", features = ["serde"] }
```

## Related Documentation

- **[benchmark.md](benchmark.md)** - Basic benchmarking framework
- **[../Advanced/advanced_benchmark.md](../Advanced/advanced_benchmark.md)** - Hardware-aware analysis (cache, energy, NUMA)
- **[library_comparison.md](library_comparison.md)** - Comparisons against std/Rayon/ndarray
- **[../Advanced/Cross-Platform_Validation.md](../Advanced/Cross-Platform_Validation.md)** - Statistical validation framework
