# Thread Affinity and Core Topology Analysis

## Prerequisites
This document assumes familiarity with parallel sorting implementations. See [sorting.md](sorting.md) and [rayon.md](rayon.md) for background.

---

A benchmarking module for analyzing parallel algorithm performance under different thread placement strategies on hybrid CPU architectures (e.g., Intel 13th gen with P-cores and E-cores).

## Overview

Modern CPUs often feature heterogeneous core architectures with performance cores (P-cores) and efficiency cores (E-cores). This module provides tools to:

- Detect CPU core topology automatically via Linux sysfs
- Bind Rayon worker threads to specific cores using `core_affinity`
- Benchmark parallel algorithms under different placement strategies
- Find optimal parallel-to-sequential thresholds

## Core Topology Detection

```rust
pub struct CoreTopology {
    pub total_logical_cores: usize,
    pub total_physical_cores: usize,
    pub p_core_ids: Vec<usize>,     // Performance core IDs
    pub e_core_ids: Vec<usize>,     // Efficiency core IDs
    pub is_hybrid: bool,
    pub topology_source: String,
}
```

### Detection Methods

1. **Automatic (sysfs)**: Reads `/sys/devices/system/cpu/cpu*/cpufreq/cpuinfo_max_freq` to distinguish P-cores (higher max frequency) from E-cores
2. **Manual configuration**: `CoreTopology::intel_13th_gen_13650hx()` for known hardware
3. **Default fallback**: Treats all cores as homogeneous when sysfs is unavailable

### Example: Intel i7-13650HX

```
P-cores: 6 physical (12 logical with HT), IDs 0-11
E-cores: 8 physical (8 logical, no HT), IDs 12-19
Total:   14 physical, 20 logical
```

## Thread Placement Strategies

```rust
pub enum ThreadPlacement {
    PerformanceCoresOnly,   // P-cores only (highest per-thread perf)
    EfficiencyCoresOnly,    // E-cores only (throughput workloads)
    AllCoresDefault,        // OS-scheduled (default Rayon behavior)
    PhysicalCoresOnly,      // No hyperthreading (P-core primaries + E-cores)
    BoundCores(usize),      // Specific core count with affinity binding
}
```

## Affinity-Aware Benchmarking

The runner creates a dedicated Rayon thread pool per placement strategy, using `start_handler` to pin each worker thread:

```rust
let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(thread_count)
    .start_handler(move |thread_index| {
        if thread_index < cores.len() {
            let _ = core_affinity::set_for_current(cores[thread_index]);
        }
    })
    .build()
    .unwrap();
```

### Usage

```bash
# Comprehensive affinity benchmark
cargo run --release -- affinity --size 500000 --runs 10

# With thread scaling analysis
cargo run --release -- affinity --size 500000 --runs 10 --scaling
```

```rust
let mut runner = AffinityBenchmarkRunner::new();

// Test all placement strategies
runner.run_comprehensive_affinity_benchmark(500_000, 10);

// Thread scaling with specific counts
runner.run_scaling_with_affinity(
    "Merge Sort", 500_000,
    &[1, 2, 4, 6, 8, 12, 14, 20], 10
);

runner.save_results("affinity_benchmark")?;
```

## Result Structure

```rust
pub struct AffinityBenchmarkResult {
    pub algorithm: String,
    pub data_size: usize,
    pub placement: String,
    pub thread_count: usize,
    pub core_ids_used: Vec<usize>,
    pub mean_time_ms: f64,
    pub std_dev_ms: f64,
    pub min_time_ms: f64,
    pub max_time_ms: f64,
    pub speedup_vs_sequential: f64,
    pub efficiency_percent: f64,
    pub individual_runs_ms: Vec<f64>,
}
```

## Threshold Optimization

The module also includes a threshold optimizer that finds the optimal crossover point between parallel and sequential execution:

```bash
cargo run --release -- threshold --size 1000000 --runs 5
```

```rust
pub fn optimize_threshold(algorithm: &str, data_size: usize, runs: usize) -> Vec<(usize, f64)>
```

Tests thresholds: 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536

## Output Files

Results are saved to `Generated_Data/Affinity_Benchmarks/`:

- **`affinity_benchmark_affinity_results.csv`** - Per-run results with core IDs
- **`affinity_benchmark_affinity_report.json`** - Full report including topology info

### CSV Format

```csv
Algorithm,DataSize,Placement,ThreadCount,MeanTime(ms),StdDev(ms),MinTime(ms),MaxTime(ms),Speedup,Efficiency(%),CoreIDs
Merge Sort,500000,P-cores only,12,8.234,0.156,...,3.45,28.8,"[0,1,2,...,11]"
```

## Dependencies

```toml
core_affinity = "0.8"   # CPU core binding
num_cpus = "1.16"        # CPU count detection
rayon = "1.8"            # Configurable thread pools
statistical = "1.0"      # Statistical analysis
```

## Related Documentation

- **[sorting.md](sorting.md)** - Parallel sorting with configurable thresholds
- **[rayon.md](rayon.md)** - Rayon `start_handler` for thread pinning
- **[../Advanced/advanced_benchmark.md](../Advanced/advanced_benchmark.md)** - Hardware-aware performance analysis
