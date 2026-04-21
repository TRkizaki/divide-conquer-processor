# Library Comparison Benchmarks

## Prerequisites
This document assumes familiarity with the sorting and matrix implementations. See [sorting.md](sorting.md) and [matrix.md](matrix.md) for algorithm details.

---

A benchmarking module that compares the project's custom divide-and-conquer implementations against established Rust libraries: the standard library, Rayon, and ndarray.

## Overview

The Library Comparison module provides honest, head-to-head benchmarks between custom implementations and production-quality libraries. This is essential for academic credibility -- showing not just that custom algorithms work, but how they perform relative to well-optimized alternatives.

## Key Results from the JAIT Paper

### Parallel Sorting (1M elements, Table IV)

| Algorithm | Library | Mean Time | Speedup vs Seq Custom |
|---|---|---|---|
| Merge Sort (Parallel) | Custom D&C | 19.1 ms | **6.35×** |
| Quick Sort (Parallel) | Custom D&C | 12.9 ms | **9.36×** |
| Parallel Stable Sort | Rayon | 4.3 ms | 27.91× |
| **Parallel Unstable Sort** | **Rayon** | **4.0 ms** | **30.17×** |

**Finding**: Rayon's `par_sort_unstable()` achieves **~3.2× higher throughput** than our custom parallel quicksort. This gap quantifies the difference between a pedagogical divide-and-conquer implementation and a production-optimized parallel sort combining pdqsort's adaptive partitioning with Rayon's internal optimizations. Transparently reporting this gap allows practitioners to make informed decisions: use production sorts for throughput, use our framework for understanding and research.

### Matrix Multiplication (512×512, Table VI)

| Algorithm | Library | Mean Time | Speedup |
|---|---|---|---|
| Standard O(n³) | Custom | 292.9 ms | 1.00× |
| Strassen O(n^2.807) | Custom | 185.3 ms | 1.58× |
| Cache-Optimized | Custom | 113.1 ms | 2.59× |
| SIMD (AVX2) | Custom | 157.7 ms | 1.86× |
| Parallel | Custom | 17.2 ms | **17.00×** |
| **dot()** | **ndarray / MKL** | **5.7 ms** | **51.60×** |

**Mechanistic gap decomposition**: The ~3× advantage of ndarray/Intel MKL over our best parallel implementation (51.6× vs 17×) is not attributable to a single optimization. Drawing on the Intel MKL architecture [28] and GotoBLAS design [12], it decomposes multiplicatively as:

| Mechanism | Estimated Contribution |
|---|---|
| Multi-level cache blocking (L1/L2/L3) | 1.4–1.6× |
| Micro-kernel SIMD register blocking | 1.3–1.5× |
| NUMA-aware thread scheduling | 1.1–1.2× |
| **Combined** | **≈ 2.4–2.9×** |

Which closely matches the observed ~3× gap, confirming that no "silver bullet" optimization explains production-library performance — rather, it is the coordinated application of cache blocking, micro-kernel design, and thread scheduling.

### Optimization Layer Hierarchy (Section IV.B)

Paper quantifies the impact of individual optimization layers:

1. **Parallelism** (17.00×) — dominates all other optimizations at this problem size
2. **Cache optimization** (2.59×) — outperforms algorithmic complexity reduction (Strassen at 1.58×) at 512×512
3. **Winograd** (2.38×) — reduces multiplications; beats Strassen due to lower overhead at this size
4. **SIMD** (1.86×) — moderate gain, limited by data-layout overhead
5. **Strassen** (1.58×) — asymptotic advantage only meaningful above ~1000×1000

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
