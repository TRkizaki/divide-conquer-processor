# Reproducibility Guide

This guide describes how to reproduce the results reported in the accepted JAIT paper *"High-Performance Divide-and-Conquer Algorithms: A Comprehensive Framework with Recursive Parallel Decomposition and Hardware-Aware Optimization"* (Kizaki, Capeska Bogatinoska, Nandal, Karadimce — 2026).

## Quick Start

To reproduce the publication-quality benchmark results, run:

```bash
# Clone the repository
git clone https://github.com/TRkizaki/divide-conquer-processor
cd divide-conquer-processor

# Build in release mode
cargo build --release

# Run publication benchmark (standard)
cargo run --release -- publication --runs 10

# Run extended benchmark (for scalability analysis)
cargo run --release -- publication --runs 20 --extended
```

## Detailed Reproduction Instructions

### System Requirements

**Minimum Requirements:**
- **CPU**: 4+ physical cores (hybrid P-core/E-core desirable to reproduce Section IV.C thread affinity results)
- **Memory**: 8 GB RAM minimum, 16 GB+ recommended for extended tests
- **Storage**: 2 GB free space for benchmark data and results
- **OS**: Linux (required for reproducing paper's P-core/E-core sysfs detection and affinity results)

**Paper Platform (exact match):**
- **CPU**: Intel Core i7-13650HX (6 P-cores @ 4.9 GHz + 8 E-cores @ 3.6 GHz, 14 physical / 20 logical)
- **Memory**: 24 GB DDR4
- **OS**: Linux 6.12.10 (Pop!_OS 22.04)
- **Rust**: 1.89.0 stable

### Software Dependencies

**Core Requirements:**
```toml
# Rust toolchain
rustc = "1.89.0+"
cargo = "1.89.0+"

# Key dependencies (automatically installed)
rayon = "1.8"           # Parallel processing
criterion = "0.6.0"     # Benchmarking framework
plotters = "0.3"        # SVG figure generation
ndarray = "0.16"        # Library comparison benchmarks
core_affinity = "0.8"   # CPU core binding
memory-stats = "1.1"    # Memory monitoring
statistical = "1.0"     # Statistical analysis
serde = "1.0"          # Data serialization
```

### Installation Steps

1. **Install Rust Toolchain**
   ```bash
   # Via rustup (recommended)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   
   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Clone Repository**
   ```bash
   git clone https://github.com/TRkizaki/divide-conquer-processor
   cd divide-conquer-processor
   
   # Verify exact commit (important for reproducibility)
   git log --oneline -1
   ```

3. **Build Project**
   ```bash
   # Clean build to ensure reproducibility
   cargo clean
   cargo build --release
   
   # Verify build success
   cargo run --release -- --help
   ```

### Benchmark Execution

#### Standard Publication Benchmark

```bash
# Run with default settings (10 runs per test)
cargo run --release -- publication --runs 10

# Expected runtime: 15-30 minutes
# Generated files:
#   - publication_benchmark_full_report.json
#   - publication_benchmark_detailed_results.csv
#   - publication_benchmark_scalability.csv
#   - publication_benchmark_parallel_efficiency.csv
```

#### Extended Scalability Analysis

```bash
# Include larger data sizes (up to 1M elements)
cargo run --release -- publication --runs 20 --extended

# Expected runtime: 2-4 hours
# Requires: 16GB+ RAM for largest datasets
```

#### Quick Validation Run

```bash
# Reduced runs for testing setup
cargo run --release -- publication --runs 3

# Expected runtime: 5-10 minutes
# Useful for: Verifying installation and basic functionality
```

### Individual Benchmark Commands

```bash
# Sorting algorithms only
cargo run --release -- sort --size 50000 --runs 10 --parallel

# Matrix multiplication
cargo run --release -- matrix --size 512 --strassen

# Computational geometry
cargo run --release -- geometry --points 10000

# Library comparison (vs std, Rayon, ndarray)
cargo run --release -- compare --runs 10

# Thread affinity analysis (P-core/E-core)
cargo run --release -- affinity --size 500000 --runs 10 --scaling

# Parallel threshold optimization
cargo run --release -- threshold --size 1000000 --runs 5

# Advanced hardware-aware benchmarking
cargo run --release -- advanced --runs 5 --sizes 1000 5000 10000

# Cross-platform validation
cargo run --release -- validate --runs 3 --optimization --threading

# Generate publication-quality SVG figures
cargo run --release -- figures

# Comprehensive suite
cargo run --release -- all --small
```

### Expected Results Structure

The `publication` command writes its four files to the **project root** (for convenient access). All other commands write to subdirectories of `Generated_Data/`.

#### Generated Files

```
<project root>/
├── publication_benchmark_full_report.json      # from: publication command
├── publication_benchmark_detailed_results.csv
├── publication_benchmark_scalability.csv
└── publication_benchmark_parallel_efficiency.csv

Generated_Data/
├── Library_Comparisons/               # from: compare command
│   ├── library_comparison.json
│   └── library_comparison.csv
├── Affinity_Benchmarks/               # from: affinity command
│   ├── affinity_benchmark_affinity_report.json
│   └── affinity_benchmark_affinity_results.csv
├── Advanced_Benchmarks/               # from: advanced command
│   ├── advanced_benchmark_advanced_benchmark.json
│   └── advanced_benchmark_advanced_metrics.csv
├── Cross-Platform_validation/         # from: validate command
│   ├── cross_platform_validation_validation_report.json
│   ├── cross_platform_validation_validation_results.csv
│   ├── cross_platform_validation_distribution_analysis.csv
│   ├── cross_platform_validation_optimization_impact.csv
│   └── cross_platform_validation_thread_scaling.csv
└── Figures/                           # from: figures command
    └── fig*.svg                       # Publication-quality SVG charts
```

**Full Report (JSON)**:
```
publication_benchmark_full_report.json
├── system_specs: Hardware/software configuration
├── benchmark_date: Execution timestamp
├── methodology: Description of experimental approach
├── detailed_results: Individual algorithm performance
├── scalability_results: Performance vs data size
├── parallel_efficiency_results: Performance vs thread count
└── comparative_analysis: Speedup vs standard library
```

**Detailed Results (CSV)**:
```
Algorithm,DataSize,Runs,MeanTime(ms),StdDev(ms),MinTime(ms),MaxTime(ms),MedianTime(ms),MeanMemory(MB),Parallel,SpeedupVsSequential,Efficiency
Merge Sort,1000,10,0.245,0.012,0.231,0.267,0.244,N/A,false,N/A,N/A
Merge Sort (Parallel),1000,10,0.198,0.015,0.178,0.221,0.196,N/A,true,1.238,0.062
...
```

### Verification Steps

#### 1. System Information Verification

```bash
# Check CPU information
lscpu | grep -E "(Model name|CPU\(s\)|Thread|Core)"

# Check memory
free -h

# Check Rust version
rustc --version

# Check OS
uname -a
```

#### 2. Result Validation

**Basic Sanity Checks:**
- Parallel implementations should show speedup > 1.0 for large datasets
- Standard library comparisons should be within expected ranges
- Memory usage should scale approximately linearly with data size

**Statistical Validation:**
- Standard deviation should be < 10% of mean for stable algorithms
- Confidence intervals should not overlap for significantly different algorithms
- Speedup should plateau at available CPU cores

#### 3. Performance Baselines

**Expected Performance at 1M elements** (Intel i7-13650HX, paper Table III / IV, 10 runs):

| Algorithm | Library | Mean Time | Speedup vs Seq Custom |
|---|---|---|---|
| Merge Sort (Sequential) | Custom D&C | ~121 ms | 1.00× |
| Merge Sort (Parallel) | Custom D&C | ~19 ms | **6.35×** |
| Quick Sort (Sequential) | Custom D&C | ~63 ms | 1.92× |
| Quick Sort (Parallel) | Custom D&C | ~13 ms | **9.36×** |
| Stable Sort | std library | ~17 ms | 6.94× |
| Unstable Sort | std library | ~14 ms | 8.78× |
| Parallel Stable Sort | Rayon | ~4.3 ms | **27.91×** |
| Parallel Unstable Sort | Rayon | ~4.0 ms | **30.17×** |

**Thread Scaling (Merge Sort, 1M elements, paper Table VII):**
| Bound Cores | Mean Time | Speedup | Efficiency |
|---|---|---|---|
| 1 | ~122 ms | 1.04× | 103.6% |
| 2 | ~82 ms | 1.53× | 76.5% |
| 6 | ~35 ms | 3.63× | 60.5% |
| 14 | ~23 ms | **5.47×** | **39.1%** |

**Thread Placement (Quicksort, 1M, paper Table X):**
| Placement | Cores | Mean Time |
|---|---|---|
| **P-cores only** | 12 | **~12.6 ms** |
| E-cores only | 8 | ~16.8 ms |
| Physical only (no HT) | 14 | ~13.4 ms |
| All cores (OS sched) | 20 | ~14.6 ms |

Key reproducibility finding: **P-cores-only outperforms all-core OS scheduling by 14%** for quicksort.

*Note: ±10% variation from thermal, scheduler, and background-load conditions is normal.*

### Troubleshooting

#### Common Issues

**Build Failures:**
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build --release

# Check for conflicting dependencies
cargo tree
```

**Runtime Errors:**
```bash
# Insufficient memory for large datasets
ulimit -v  # Check virtual memory limit

# Permission issues
chmod +x target/release/divide-conquer-processor

# Missing system libraries
sudo apt-get install build-essential  # Ubuntu/Debian
```

**Performance Issues:**
```bash
# Check CPU frequency scaling
cat /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Disable frequency scaling for consistent results
sudo cpupower frequency-set --governor performance

# Check system load
top -n 1 | head -5
```

#### Environment Variables

```bash
# Ensure release mode optimizations
export CARGO_PROFILE_RELEASE_LTO=true
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1

# Set thread pool size explicitly
export RAYON_NUM_THREADS=20

# Control memory allocation
export MALLOC_ARENA_MAX=1
```

### Platform Notes

The paper's results — and the thread-affinity pipeline specifically — require Linux. Automatic P-core / E-core detection reads `/sys/devices/system/cpu/cpu*/cpufreq/cpuinfo_max_freq`, which is Linux-specific.

- **Linux x86_64** (Intel hybrid CPU): full reproducibility.
- **Linux x86_64** (homogeneous CPU, e.g. AMD Ryzen): sorting / matrix / library-comparison results reproducible; affinity results will show uniform placement strategies rather than P-core vs E-core differentiation.
- **Other platforms** (macOS, Windows, ARM): not supported for paper reproduction. The framework may build and run, but sysfs-based topology detection falls back to homogeneous mode and results will not match the paper.

### Data Collection Best Practices

#### System Preparation
```bash
# Close unnecessary applications
# Disable background services
# Ensure stable power supply (laptops: connect to power)

# For servers: disable frequency scaling
sudo cpupower frequency-set --governor performance

# Clear system caches
sudo sh -c 'echo 3 > /proc/sys/vm/drop_caches'  # Linux only
```

#### Multiple Runs for Statistical Significance
```bash
# Run multiple independent sessions
for i in {1..5}; do
    cargo run --release -- publication --runs 10
    mv publication_benchmark_detailed_results.csv results_run_$i.csv
done

# Combine results for meta-analysis
```

### Advanced Configuration

#### Custom Data Sizes
```rust
// Modify src/main.rs for custom benchmark sizes
let custom_sizes = vec![2000, 8000, 32000, 128000];
runner.benchmark_sorting_comprehensive(&custom_sizes, runs);
```

#### Custom Thread Counts
```rust
// Modify parallel efficiency analysis
let custom_threads = vec![1, 3, 6, 12, 24];
runner.analyze_parallel_efficiency("Merge Sort", 50000, &custom_threads);
```

#### Memory Profiling
```bash
# Use valgrind for detailed memory analysis (Linux)
valgrind --tool=massif cargo run --release -- publication --runs 3

# Use perf for CPU profiling
perf record cargo run --release -- publication --runs 3
perf report
```

### Version Control and Checksums

#### Git Commit Verification
```bash
# Verify you're using the correct version
git rev-parse HEAD
# Expected: [specific commit hash will be provided]

# Check for local modifications
git status
# Should show "working tree clean"
```

#### Result Checksums
```bash
# Generate checksums for verification
sha256sum publication_benchmark_*.{json,csv}

# Compare with reference checksums (if provided)
```

### Citation and Attribution

When using these benchmarks in academic work, please cite the accepted JAIT paper:

```bibtex
@article{kizaki2026divideconquer,
  title     = {High-Performance Divide-and-Conquer Algorithms: A Comprehensive
               Framework with Recursive Parallel Decomposition and
               Hardware-Aware Optimization},
  author    = {Kizaki, Tetsurou and Capeska Bogatinoska, Dijana and
               Nandal, Amita and Karadimce, Aleksandar},
  journal   = {Journal of Advances in Information Technology (JAIT)},
  year      = {2026},
  note      = {Accepted},
  url       = {https://github.com/TRkizaki/divide-conquer-processor}
}
```

### Support and Issues

For reproducibility issues:
1. Check this guide thoroughly
2. Verify system requirements
3. Report issues with:
   - System specifications
   - Rust version (`rustc --version`)
   - Exact error messages
   - Generated log files

**Issue Template:**
```
System: [OS, CPU, RAM]
Rust Version: [rustc --version output]
Command: [exact command run]
Error: [error message]
Expected: [what should happen]
Logs: [attach relevant log files]
```

### Performance Validation

Expected performance characteristics that indicate correct reproduction:

1. **Scaling Behavior**: O(n log n) for sorting algorithms
2. **Parallel efficiency at 14 cores**: ~39% for merge sort, ~37% for quicksort (paper Table VII / VIII) — *not* linear; the efficiency ceiling is expected and is a core finding of the paper (Amdahl's-law sequential fraction)
3. **Memory Usage**: Linear scaling with data size
4. **Coefficient of variation**: mean CV < 3%, individual CVs < 5.1% across distributions
5. **Quicksort distribution sensitivity**: custom median-of-three implementation should show ~4–5× worst-to-best ratio; naive first-element pivot variants exhibit catastrophic 1534× degradation on sorted input

If your results significantly deviate from these patterns, review system configuration (CPU governor, background load, thermal throttling) and environment setup.