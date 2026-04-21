# Experimental Methodology

## Overview

This document describes the methodology used for benchmarking divide-and-conquer algorithms in the divide-conquer-processor project. It is aligned with Section III.E of the accepted JAIT paper *"High-Performance Divide-and-Conquer Algorithms: A Comprehensive Framework with Recursive Parallel Decomposition and Hardware-Aware Optimization"* and reproduces its measurement protocol.

## Experimental Platform

### Hardware Configuration
- **CPU**: 13th Generation Intel® Core™ i7-13650HX (Raptor Lake-HX)
- **P-cores**: 6 physical × Raptor Cove, up to 4.9 GHz, Hyper-Threading enabled (12 logical threads)
- **E-cores**: 8 physical × Gracemont, up to 3.6 GHz, no Hyper-Threading (8 logical threads)
- **Total**: 14 physical / 20 logical cores
- **Cache hierarchy**: 24 MB L3 (shared); per-core L2 of 1.25 MB (P-core) / 2 MB shared per 4 E-cores
- **Memory**: 24 GB DDR4, peak bandwidth ≈ 51.2 GB/s
- **Architecture**: x86_64 with AVX2

### Software Environment
- **Operating System**: Linux 6.12.10 (Pop!_OS 22.04)
- **Rust Toolchain**: rustc 1.89.0 (29483883e 2025-08-04), stable
- **Compiler Flags**: `--release` (equivalent to `-O3`)
- **Rayon**: version 1.8
- **Timing**: `std::time::Instant` (nanosecond precision)
- **Memory**: `memory-stats` 1.1

## Theoretical Framework (Work–Span Model)

Algorithms are characterized using the work–span (W–S) model — see paper Section III.B:

- **Work** W(n): total sequential operations
- **Span** S(n): longest dependent operation chain (critical path)
- **Parallelism** P(n) = W(n) / S(n)
- **Brent–Blumofe–Leiserson bound**: T_p ≤ W(n)/p + O(S(n))
- **Serial fraction** f = S(n) / W(n), with Amdahl's law S_max(p) = 1 / (f + (1 − f)/p)

Per-algorithm analytical results (from the paper):

| Algorithm | W(n) | S(n) | f (theoretical) | f_eff (measured) |
|---|---|---|---|---|
| Parallel Merge Sort | Θ(n log n) | Θ(n) | ≈ 1/log n | **0.13** (at n = 1M) |
| Parallel Quick Sort (med-of-3) | Θ(n log n) | Θ(n) | ≈ 1/log n | **0.11** (at n = 1M) |
| Parallel Strassen | Θ(n^log₇) | Θ(n²) | — | parallelism not bottleneck |

f_eff is fitted to observed S(p) via least-squares regression, capturing real parallelization overhead not modeled by ideal Amdahl.

## Benchmark Execution Protocol

### Sample Sizes and Data Generation

- **Sorting**: 10,000 / 100,000 / 1,000,000 elements; 32-bit signed integers
- **Matrix multiplication**: 64×64 / 128×128 / 256×256 / 512×512; f64 values in [0, 1]
- **Closest pair**: 1,000 / 50,000 / 100,000 points; f64 coordinates in [0, 1000]
- **Seeds are fixed** for reproducibility; fresh data is cloned per run to prevent already-sorted inputs and memory-caching artifacts
- **Distribution sensitivity** suite: random, sorted, reverse-sorted, partially sorted (80%), duplicate-heavy (10 unique values)

### Run Structure

- **Warm-up**: 1 untimed iteration per configuration to prime caches and the JIT-free hot path
- **Timed runs**: 10 runs per configuration (default); 20 for extended mode
- **Explicit memory barriers** between runs for temporal independence

### Statistical Methods

- **Primary metric**: mean execution time μ
- **Variability**: standard deviation σ, coefficient of variation CV = σ / μ
- **Outlier detection**: interquartile range (IQR) method — flagged but not excluded
- **Significance**: one-way ANOVA across distributions (paper reports F = 3.71, p = 0.05, η² = 0.329)
- **Reliability threshold**: all mean CVs observed below 3%, individual CVs below 5.1% — confirming measurement noise does not dominate observed differences

## Hardware-Aware Threshold Optimization

The parallel-to-sequential crossover threshold T is determined empirically by sweeping candidate values {512, 1024, 2048, 4096, 8192, 16384, 32768} and measuring execution time at 1M elements.

The cost function balances two competing factors:

```
C_overhead(T) = c_task × (n / T)           // task-creation overhead
C_idle(T)     = W(T) × max(0, p − n/T) / p  // idle cores below threshold
```

with c_task ≈ 0.5 µs empirically measured for `rayon::join()`.

Optimal thresholds determined on the experimental platform:
- **T* ≈ 8192** for merge sort (recursion depth ≈ 7, ≈ 128 tasks)
- **T* ≈ 4096** for quicksort (recursion depth ≈ 8, ≈ 256 tasks)

The optimum achieves a task-count-to-core ratio of ~8–16× — enough load-balancing granularity without excessive scheduling overhead.

## Thread Affinity Protocol

Rayon worker threads are pinned to specific physical cores using `ThreadPoolBuilder::start_handler()` combined with the `core_affinity` crate:

```rust
let pool = ThreadPoolBuilder::new()
    .num_threads(core_ids.len())
    .start_handler(move |thread_index| {
        core_affinity::set_for_current(core_ids[thread_index]);
    })
    .build_global();
```

### Placement Strategies Evaluated

| Placement | Cores | Description |
|---|---|---|
| P-cores only | 12 (6 phys × 2 HT) | Highest per-thread performance |
| E-cores only | 8 | Throughput under power budget |
| Physical only (no HT) | 14 | All physical cores, no SMT |
| All cores (OS-scheduled) | 20 | Default Rayon behavior (baseline) |

P-core / E-core topology is detected automatically from `/sys/devices/system/cpu/cpu*/cpufreq/cpuinfo_max_freq`.

## Comparative Benchmarking Against Production Libraries

To honestly attribute performance contributions, custom implementations are compared head-to-head with production-grade libraries under identical compiler flags (`--release`):

- **Rust standard library**: `slice::sort()` (TimSort, stable), `slice::sort_unstable()` (pdqsort)
- **Rayon**: `par_sort()` (parallel stable), `par_sort_unstable()` (parallel pdqsort)
- **ndarray 0.16**: `dot()` for matrix multiplication (BLAS-backed where available)

Baselines for speedup calculation:
- **Speedup vs sequential custom**: T_seq(custom) / T_par(custom) — avoids inflated numbers by using the *sequential* version of the same algorithm (not the parallel algorithm on 1 thread)
- **Library speedup ratio**: T_custom_parallel / T_library — used for honest production comparisons

## Compiler Optimization Analysis

Each algorithm is compiled at O0, O1, O2, O3 and executed across the same dataset. Paper reports consistent 40–44% improvement from O0→O3 across all three algorithm classes, with matrix multiplication most responsive (43.8%) due to its regular nested-loop structure amenable to unrolling and vectorization.

## Reproducibility

### Version Control
- Git commit hash of source revision is recorded in every output JSON
- `Cargo.lock` is committed for dependency pinning

### Environment Preparation (Linux)
```bash
# Set CPU governor to performance for stable frequency
sudo cpupower frequency-set --governor performance

# Drop page cache before heavy benchmarks
sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches'

# Verify no background load
uptime
```

### Standard Execution
```bash
# Full paper reproduction (10 runs default)
cargo run --release -- publication --runs 10

# Extended scalability analysis
cargo run --release -- publication --runs 20 --extended

# Thread affinity experiments (Table X in paper)
cargo run --release -- affinity --size 1000000 --runs 10 --scaling

# Threshold sweep (Table IX in paper)
cargo run --release -- threshold --size 1000000 --runs 5

# Library comparison (Tables III, IV, VI in paper)
cargo run --release -- compare --runs 10
```

## Output Formats

- **Structured JSON**: system specs, per-run results, statistical summaries, timestamps
- **Tabular CSV**: one row per (algorithm, size, thread count) — suitable for pandas/R/Excel
- **Text summary**: human-readable report printed to stdout

See `REPRODUCIBILITY_GUIDE.md` for exact file layouts.

## Limitations

1. **Single hardware platform**: Results are specific to Intel i7-13650HX; AMD, ARM, and server-grade platforms may differ (paper Section IV.G).
2. **Data types**: Sorting evaluated on 32-bit integers only.
3. **No GPU comparison**: Outside the scope of this CPU-focused study.
4. **Sequential baseline**: Custom sequential implementations prioritize pedagogical clarity, so custom parallel speedups are reported against a ~7–10× slower baseline than std-library sorts — this is disclosed explicitly rather than concealed.

## References

1. Cormen, T. H., Leiserson, C. E., Rivest, R. L., & Stein, C. (2022). *Introduction to Algorithms*, 4th ed. MIT Press.
2. Blumofe, R. D., & Leiserson, C. E. (1999). Scheduling multithreaded computations by work stealing. *J. ACM*, 46(5), 720–748.
3. Matsakis, N. (2024). *Rayon: a data parallelism library for Rust*. https://docs.rs/rayon/
4. Kizaki, T., Capeska Bogatinoska, D., Nandal, A., & Karadimce, A. (2026). High-Performance Divide-and-Conquer Algorithms: A Comprehensive Framework with Recursive Parallel Decomposition and Hardware-Aware Optimization. *Journal of Advances in Information Technology (JAIT)*. Accepted.
