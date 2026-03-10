# Library Comparison Benchmarks

## Prerequisites
This document assumes familiarity with the sorting and matrix implementations. See [sorting.md](sorting.md) and [matrix.md](matrix.md) for algorithm details.

---

A benchmarking module that compares the project's custom divide-and-conquer implementations against established Rust libraries: the standard library, Rayon, and ndarray.

## Overview

The Library Comparison module provides honest, head-to-head benchmarks between custom implementations and production-quality libraries. This is essential for academic credibility -- showing not just that custom algorithms work, but how they perform relative to well-optimized alternatives.

## Sorting Comparisons

### Algorithms Benchmarked

| Algorithm | Library | Description |
|-----------|---------|-------------|
| Merge Sort (Sequential) | Custom D&C | Recursive merge sort |
| Merge Sort (Parallel) | Custom D&C | `rayon::join()` parallel D&C |
| Quick Sort (Sequential) | Custom D&C | Median-of-three quicksort |
| Quick Sort (Parallel) | Custom D&C | `rayon::join()` parallel D&C |
| Stable Sort | std library | `slice::sort()` (TimSort) |
| Unstable Sort | std library | `slice::sort_unstable()` (pattern-defeating quicksort) |
| Parallel Stable Sort | Rayon | `par_sort()` |
| Parallel Unstable Sort | Rayon | `par_sort_unstable()` |

### Usage

```bash
cargo run --release -- compare --runs 10
```

```rust
let mut runner = LibraryComparisonRunner::new();
runner.compare_sorting(&[10_000, 100_000, 1_000_000], 10);
runner.calculate_speedups();
runner.save_results("library")?;
```

## Matrix Multiplication Comparisons

### Algorithms Benchmarked

| Algorithm | Library | Description |
|-----------|---------|-------------|
| Standard O(n^3) | Custom | Triple-loop multiplication |
| Strassen O(n^2.807) | Custom | Divide-and-conquer |
| Cache-Optimized | Custom | Block tiling (block_size=32) |
| SIMD (AVX2) | Custom | Vectorized multiplication |
| Winograd | Custom | Reduced multiplication count |
| Parallel | Custom | Rayon row-level parallelism |
| Parallel Winograd | Custom | Multi-threaded Winograd |
| dot() | ndarray | Optimized matrix dot product |

### Usage

```rust
runner.compare_matrix_multiply(&[64, 128, 256, 512], 10);
```

## Result Structure

```rust
pub struct LibraryComparisonResult {
    pub category: String,          // "Sorting" or "Matrix Multiply"
    pub algorithm: String,         // Algorithm name
    pub library: String,           // "Custom D&C", "std library", "Rayon", "ndarray"
    pub data_size: usize,
    pub runs: usize,
    pub mean_time_ms: f64,
    pub std_dev_ms: f64,
    pub min_time_ms: f64,
    pub max_time_ms: f64,
    pub median_time_ms: f64,
    pub speedup_vs_baseline: Option<f64>,  // Relative to first algorithm per category
}
```

## Output Files

Results are saved to `Generated_Data/Library_Comparisons/`:

- **`library_comparison.csv`** - Tabular results with all metrics
- **`library_comparison.json`** - Structured JSON for programmatic analysis

### CSV Format

```csv
Category,Algorithm,Library,DataSize,Runs,MeanTime(ms),StdDev(ms),MinTime(ms),MaxTime(ms),MedianTime(ms),SpeedupVsBaseline
Sorting,Merge Sort (Sequential),Custom D&C,100000,10,15.234,0.456,...
Sorting,Stable Sort,std library,100000,10,12.567,0.321,...
```

## Benchmark Methodology

- **Warm-up runs**: Matrix benchmarks include a warm-up run before timed iterations
- **Multiple runs**: Statistical reliability via configurable run count
- **Fresh data**: Sorting benchmarks clone data for each run to avoid already-sorted input
- **Speedup calculation**: First result per category+size serves as baseline

## Dependencies

```toml
ndarray = "0.16"      # Matrix operation comparison
rayon = "1.8"          # Parallel sort comparison
statistical = "1.0"    # Mean, standard deviation
```

## Related Documentation

- **[sorting.md](sorting.md)** - Custom sorting algorithm implementations
- **[matrix.md](matrix.md)** - Custom matrix implementations
- **[../Advanced/Advanced_matrix.md](../Advanced/Advanced_matrix.md)** - Advanced matrix algorithms
