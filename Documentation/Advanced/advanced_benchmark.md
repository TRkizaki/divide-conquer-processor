# Advanced Benchmarking

## Prerequisites
This document assumes familiarity with basic benchmarking concepts. For an introduction to basic performance measurement, see [benchmark.md](benchmark.md).

---

A comprehensive performance analysis system that goes beyond traditional benchmarking by measuring hardware-level metrics, energy consumption, and algorithmic constants. This module implements Week 2 of the publication-ready benchmarking strategy for academic research.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Metrics Collected](#metrics-collected)
- [API Reference](#api-reference)
- [Performance Analysis](#performance-analysis)
- [Usage Examples](#usage-examples)
- [Academic Applications](#academic-applications)
- [Future Enhancements](#future-enhancements)

## Overview

The Advanced Benchmarking module extends traditional performance measurement with hardware-aware metrics essential for publication-quality computer science research. It provides statistical analysis of algorithmic constants, cache behavior, energy efficiency, and NUMA effects.

**Key Differences from Basic Benchmarking:**
- Hardware-level performance counters (cache, memory bandwidth)
- Energy consumption measurement via Intel RAPL
- Statistical regression analysis with confidence intervals
- NUMA memory locality effects
- Publication-ready data formats

## Features

### Core Capabilities

- **Cache Performance Analysis**: L1/L2/L3 cache miss rates and memory bandwidth measurement
- **Energy Consumption Tracking**: Power usage and energy efficiency metrics via Intel RAPL
- **NUMA Effects Analysis**: Memory locality and cross-node bandwidth measurement
- **Algorithmic Constant Analysis**: Empirical constant calculation with statistical validation
- **Extended Scalability Testing**: Multi-dimensional performance analysis across data sizes

### Advanced Analytics

- Statistical regression analysis with confidence intervals
- Hidden factor identification (cache effects, memory bandwidth limitations)
- Cross-platform performance validation
- Publication-ready data export formats

### Integration Features

- Command-line interface for automated benchmarking
- JSON and CSV export for analysis tools
- Real-time progress monitoring with detailed output
- Comprehensive error handling and fallback mechanisms

## Metrics Collected

### Cache Performance Metrics

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePerformanceMetrics {
    pub l1_cache_misses: Option<u64>,
    pub l2_cache_misses: Option<u64>,
    pub l3_cache_misses: Option<u64>,
    pub cache_references: Option<u64>,
    pub cache_miss_rate: Option<f64>,
    pub instructions_per_cache_miss: Option<f64>,
    pub memory_bandwidth_gb_per_sec: Option<f64>,
}
```

**Key Insights:**
- Cache hierarchy behavior across different data sizes
- Memory bandwidth utilization efficiency
- Cache-aware algorithm performance characteristics

### Energy Consumption Metrics

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyConsumptionMetrics {
    pub package_energy_joules: Option<f64>,
    pub core_energy_joules: Option<f64>,
    pub uncore_energy_joules: Option<f64>,
    pub dram_energy_joules: Option<f64>,
    pub power_consumption_watts: Option<f64>,
    pub energy_efficiency_gflops_per_watt: Option<f64>,
}
```

**Key Insights:**
- Algorithm energy efficiency for green computing research
- Power consumption patterns across different workloads
- Energy-performance trade-offs in algorithmic design

### NUMA Performance Metrics

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumaPerformanceMetrics {
    pub numa_nodes: usize,
    pub local_memory_accesses: Option<u64>,
    pub remote_memory_accesses: Option<u64>,
    pub numa_miss_rate: Option<f64>,
    pub cross_node_bandwidth_gb_per_sec: Option<f64>,
}
```

**Key Insights:**
- Memory locality effects on parallel algorithm performance
- NUMA-aware algorithm design validation
- Cross-node communication overhead analysis

### Algorithmic Constant Analysis

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmicConstantAnalysis {
    pub algorithm_name: String,
    pub theoretical_complexity: String,
    pub empirical_constant: f64,
    pub constant_confidence_interval: (f64, f64),
    pub goodness_of_fit: f64,
    pub hidden_factors: Vec<String>,
}
```

**Key Insights:**
- Empirical validation of theoretical complexity analysis
- Hidden constant factors in Big-O notation
- Statistical confidence in performance measurements

## API Reference

### AdvancedBenchmarkRunner

#### `AdvancedBenchmarkRunner::new() -> Self`

Creates a new advanced benchmark runner with system detection.

```rust
let mut runner = AdvancedBenchmarkRunner::new();
```

#### `analyze_cache_performance<F>(&mut self, algorithm_name: &str, data_size: usize, benchmark_fn: F) -> Result<CachePerformanceMetrics, Box<dyn std::error::Error>>`

Analyzes cache behavior during algorithm execution.

**Parameters:**
- `algorithm_name`: Name for reporting purposes
- `data_size`: Size of data being processed
- `benchmark_fn`: Closure containing the algorithm to benchmark

```rust
let cache_metrics = runner.analyze_cache_performance("Merge Sort", 10000, || {
    let mut data = generate_random_data(10000);
    merge_sort(&mut data);
})?;
```

#### `measure_energy_consumption<F>(&mut self, algorithm_name: &str, benchmark_fn: F) -> Result<EnergyConsumptionMetrics, Box<dyn std::error::Error>>`

Measures energy consumption during algorithm execution.

```rust
let energy_metrics = runner.measure_energy_consumption("Quick Sort", || {
    let mut data = generate_random_data(5000);
    quick_sort(&mut data);
})?;
```

#### `analyze_numa_effects<F>(&mut self, algorithm_name: &str, data_size: usize, benchmark_fn: F) -> Result<NumaPerformanceMetrics, Box<dyn std::error::Error>>`

Analyzes NUMA memory effects on parallel algorithms.

```rust
let numa_metrics = runner.analyze_numa_effects("Parallel Merge Sort", 50000, || {
    let mut data = generate_random_data(50000);
    parallel_merge_sort(&mut data);
})?;
```

#### `extended_scalability_analysis<F>(&mut self, algorithm_name: &str, benchmark_fn: F, data_sizes: &[usize]) -> Result<Vec<AdvancedBenchmarkResult>, Box<dyn std::error::Error>>`

Comprehensive scalability analysis across multiple data sizes with all advanced metrics.

```rust
let benchmark_fn = |size: usize| -> f64 {
    let mut data = generate_random_data(size);
    let start = Instant::now();
    merge_sort(&mut data);
    start.elapsed().as_secs_f64() * 1000.0
};

let results = runner.extended_scalability_analysis(
    "Merge Sort", 
    benchmark_fn, 
    &[1000, 5000, 10000]
)?;
```

#### `save_advanced_results(&self, base_filename: &str) -> Result<(), Box<dyn std::error::Error>>`

Saves comprehensive results in JSON and CSV formats.

```rust
runner.save_advanced_results("experiment_2024")?;
```

**Generated Files:**
- `experiment_2024_advanced_benchmark.json` - Complete structured data
- `experiment_2024_advanced_metrics.csv` - Analysis-ready CSV format

## Performance Analysis

### Complexity Validation

The system performs statistical regression to validate theoretical complexity:

| Algorithm | Theoretical | Empirical Constant | R² Score | Confidence Interval |
|-----------|-------------|-------------------|----------|-------------------|
| Merge Sort | O(n log n) | 0.000011 | 0.9975 | [0.000010, 0.000011] |
| Quick Sort | O(n log n) | 0.000006 | 0.9995 | [0.000006, 0.000006] |
| Matrix Mult | O(n³) | 0.000005 | 0.8850 | [0.000004, 0.000006] |

### Cache Performance Analysis

```
Algorithm: Merge Sort (Data Size: 10000)
[CACHE] Analyzing cache performance for Merge Sort
    Cache miss rate: 0.00%
    Memory bandwidth: 0.08 GB/s

Algorithm: Quick Sort (Data Size: 10000)  
[CACHE] Analyzing cache performance for Quick Sort
    Cache miss rate: 0.00%
    Memory bandwidth: 0.13 GB/s
```

### Energy Efficiency Comparison

```
[ENERGY] Measuring energy consumption for algorithms:
- Quick Sort: Lower power consumption, higher efficiency
- Merge Sort: Moderate power consumption
- Matrix Multiplication: Higher power consumption for compute-intensive operations
```

## Usage Examples

### Command Line Usage

```bash
# Basic advanced benchmarking
cargo run -- advanced --runs 5

# Custom data sizes with detailed analysis
cargo run -- advanced --runs 10 --sizes 1000 5000 10000 25000
```

### Basic Advanced Benchmarking

```rust
use advanced_benchmark::AdvancedBenchmarkRunner;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut runner = AdvancedBenchmarkRunner::new();
    
    // Define benchmark function
    let benchmark_fn = |size: usize| -> f64 {
        let mut data = generate_random_data(size);
        let start = std::time::Instant::now();
        merge_sort(&mut data);
        start.elapsed().as_secs_f64() * 1000.0
    };
    
    // Run comprehensive analysis
    let results = runner.extended_scalability_analysis(
        "Merge Sort",
        benchmark_fn,
        &[1000, 5000, 10000, 25000, 50000]
    )?;
    
    // Save results
    runner.save_advanced_results("merge_sort_analysis")?;
    
    Ok(())
}
```

### Cache-Specific Analysis

```rust
fn analyze_cache_effects() -> Result<(), Box<dyn std::error::Error>> {
    let mut runner = AdvancedBenchmarkRunner::new();
    
    // Test different data sizes to observe cache effects
    let data_sizes = vec![1000, 10000, 100000, 1000000];
    
    for &size in &data_sizes {
        let cache_metrics = runner.analyze_cache_performance(
            "Cache Test",
            size,
            || {
                let data = generate_sequential_data(size);
                process_data_sequentially(&data);
            }
        )?;
        
        println!("Size: {}, Cache Miss Rate: {:.2}%", 
                size, 
                cache_metrics.cache_miss_rate.unwrap_or(0.0) * 100.0);
    }
    
    Ok(())
}
```

### Energy Consumption Study

```rust
fn compare_algorithm_energy_efficiency() -> Result<(), Box<dyn std::error::Error>> {
    let mut runner = AdvancedBenchmarkRunner::new();
    let data_size = 50000;
    
    // Compare energy consumption of different algorithms
    let algorithms = vec![
        ("Merge Sort", |data: &mut Vec<i32>| merge_sort(data)),
        ("Quick Sort", |data: &mut Vec<i32>| quick_sort(data)),
    ];
    
    for (name, sort_fn) in algorithms {
        let energy_metrics = runner.measure_energy_consumption(name, || {
            let mut data = generate_random_data(data_size);
            sort_fn(&mut data);
        })?;
        
        if let Some(power) = energy_metrics.power_consumption_watts {
            println!("{}: {:.2} W average power", name, power);
        }
    }
    
    Ok(())
}
```

## Academic Applications

### Publication-Ready Research

The advanced benchmarking system is designed for academic publications requiring:

#### 1. **Statistical Validation**

```
Empirical Validation Results:
- Algorithm: Merge Sort
- Theoretical Complexity: O(n log n)
- Empirical Constant: 0.000011 ± 0.000001
- Goodness of Fit (R²): 0.9975
- Sample Size: Multiple measurements per data point
- Confidence Level: 95%
```

#### 2. **Hardware-Aware Analysis**

```
[CONSTANTS] Analyzing algorithmic constants for Merge Sort
    Empirical constant: 0.000011
    Confidence interval: [0.000010, 0.000011]
    Goodness of fit (R²): 0.9975

[NUMA] Analyzing NUMA effects for Parallel Merge Sort
    NUMA nodes: 1
    NUMA miss rate: 0.00%
```

#### 3. **Comparative Algorithm Studies**

The system enables rigorous comparison studies suitable for academic papers:

```
Comparative Analysis: Sorting Algorithms
┌─────────────┬─────────────┬──────────────┬─────────────┐
│ Algorithm   │ Time (ms)   │ Cache Miss % │ Bandwidth   │
├─────────────┼─────────────┼──────────────┼─────────────┤
│ Merge Sort  │ 1.021       │ 0.00         │ 0.08 GB/s   │
│ Quick Sort  │ 0.571       │ 0.00         │ 0.13 GB/s   │
└─────────────┴─────────────┴──────────────┴─────────────┘
```

### Research Applications

#### Computer Architecture Research
- Cache-aware algorithm design validation
- Memory hierarchy optimization studies
- Energy-efficient computing research

#### Algorithm Analysis
- Empirical complexity validation
- Hidden constant factor analysis
- Real-world performance vs theoretical predictions

#### Systems Research
- NUMA-aware parallel algorithm evaluation
- Energy consumption optimization
- Cross-platform performance analysis

## Data Formats

### CSV Output Format

```csv
Algorithm,DataSize,ExecutionTime(ms),CacheMissRate(%),MemoryBandwidth(GB/s),PowerConsumption(W),NumaNodes,NumaMissRate(%),EmpiricalConstant,GoodnessOfFit
Merge Sort,1000,0.089,0.00,0.09,0.00,1,0.00,0.000011,0.9975
Merge Sort,5000,0.491,0.00,0.08,0.00,1,0.00,0.000011,0.9975
Quick Sort,1000,0.047,0.00,0.16,0.00,1,0.00,0.000006,0.9995
```

### JSON Output Structure

```json
{
  "algorithm_name": "Merge Sort",
  "data_size": 10000,
  "execution_time_ms": 1.021,
  "cache_metrics": {
    "cache_miss_rate": 0.00,
    "memory_bandwidth_gb_per_sec": 0.08
  },
  "energy_metrics": {
    "power_consumption_watts": 0.00
  },
  "numa_metrics": {
    "numa_nodes": 1,
    "numa_miss_rate": 0.00
  },
  "constant_analysis": {
    "empirical_constant": 0.000011,
    "constant_confidence_interval": [0.000010, 0.000011],
    "goodness_of_fit": 0.9975,
    "hidden_factors": [
      "Cache hierarchy effects",
      "Memory bandwidth limitations",
      "Branch prediction effects"
    ]
  }
}
```

## Error Handling

The system provides comprehensive error handling for various scenarios:

```rust
// Hardware unavailable fallback
if cache_metrics.cache_miss_rate.is_none() {
    println!("Cache performance counters unavailable, using estimates");
}

// Energy measurement fallback
if energy_metrics.power_consumption_watts.is_none() {
    println!("Intel RAPL unavailable, energy measurement skipped");
}

// NUMA detection fallback
if numa_metrics.numa_nodes == 0 {
    numa_metrics.numa_nodes = 1; // Default to single node
}
```

## Future Enhancements

### Planned Features

#### Advanced Hardware Metrics
- **Branch Prediction Analysis**: Miss rates and prediction accuracy
- **TLB Performance**: Translation lookaside buffer hit/miss rates
- **Instruction Pipeline Analysis**: Stall cycles and throughput metrics
- **Vector Unit Utilization**: SIMD instruction usage efficiency

#### Extended Energy Analysis
- **Per-Core Energy Breakdown**: Individual core power consumption
- **Memory Subsystem Energy**: DRAM and cache power consumption
- **Thermal Throttling Detection**: Performance impact of thermal limits

#### Advanced Statistical Analysis
- **Multivariate Regression**: Multiple factor complexity analysis
- **Outlier Detection**: Automatic identification of anomalous measurements
- **Trend Analysis**: Performance evolution over algorithm improvements

#### Distributed Benchmarking
- **Cluster Analysis**: Multi-node performance measurement
- **Network Overhead Measurement**: Communication cost analysis
- **Load Balancing Efficiency**: Work distribution optimization

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
colored = "3.0"
sysinfo = "0.32"
perf-event = "0.4"
libc = "0.2"
statistical = "1.0"
```

## Integration with Basic Benchmarking

For users transitioning from basic to advanced benchmarking:

| Basic Benchmark Feature | Advanced Benchmark Equivalent |
|-------------------------|-------------------------------|
| `benchmark_sort()` | `extended_scalability_analysis()` |
| Execution time | Execution time + Cache + Energy + NUMA |
| Memory usage | Detailed memory bandwidth analysis |
| CSV export | Enhanced CSV with hardware metrics |
| Multiple runs | Statistical analysis with confidence intervals |

**Migration Path:**
1. Start with basic benchmarking to understand algorithm performance
2. Use advanced benchmarking for research and detailed analysis
3. Combine both for comprehensive performance studies

## License

This advanced benchmarking library is provided for educational and research purposes under the MIT license. Suitable for academic publications with proper attribution.

---

**Note**: For basic performance measurement and getting started with benchmarking, see [benchmark.md](benchmark.md). This document focuses on advanced hardware-aware analysis for research and optimization purposes.