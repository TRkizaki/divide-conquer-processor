# Reproducibility Guide

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
- **CPU**: 4+ cores recommended for parallel analysis
- **Memory**: 8GB RAM minimum, 16GB+ recommended for extended tests
- **Storage**: 2GB free space for benchmark data and results
- **OS**: Linux, macOS, or Windows (Linux preferred for consistency)

**Optimal Configuration:**
- **CPU**: Multi-core processor (8+ cores ideal)
- **Memory**: 24GB+ RAM for large-scale tests
- **Storage**: SSD for faster I/O during data generation

### Software Dependencies

**Core Requirements:**
```toml
# Rust toolchain
rustc = "1.89.0+" 
cargo = "1.89.0+"

# Key dependencies (automatically installed)
rayon = "1.8"           # Parallel processing
criterion = "0.6.0"     # Benchmarking framework
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

# Comprehensive suite
cargo run --release -- all --small
```

### Expected Results Structure

#### Generated Files

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

**Expected Performance Ranges (Intel i7-13650HX, 50K elements):**
- Merge Sort (Sequential): 15-25ms
- Merge Sort (Parallel): 8-15ms  
- Quick Sort (Sequential): 10-20ms
- Quick Sort (Parallel): 5-12ms
- std::slice::sort_unstable: 8-18ms

*Note: Actual values may vary based on system configuration*

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

### Cross-Platform Considerations

#### Linux (Recommended)
- Most stable and predictable performance
- Better memory management for large datasets
- More accurate timing measurements

#### macOS
- Similar performance to Linux
- May require Xcode command line tools
- Apple Silicon (M1/M2) will show different absolute performance

#### Windows
- Performance may vary due to Windows scheduler
- Requires Visual Studio Build Tools
- Consider using WSL2 for Linux-like environment

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

When using these benchmarks in academic work, please cite:

```bibtex
@software{divide_conquer_processor,
  title={High-Performance Divide and Conquer Algorithms for Large-Scale Data Processing},
  author={TETSUROU KIZAKI},
  year={2025},
  url={https://github.com/TRkizaki/divide-conquer-processor},
  version={0.1.0}
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
2. **Parallel Efficiency**: >60% efficiency with 4 threads for large datasets
3. **Memory Usage**: Linear scaling with data size
4. **Standard Deviation**: <5% for stable measurements

If your results significantly deviate from these patterns, review system configuration and environment setup.