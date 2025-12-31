# Performance Charts Module

A comprehensive Rust module for generating performance visualizations and reports from benchmark data. This module provides tools to create charts, detailed reports, and CSV summaries from benchmark results.

## Features

- **Multi-chart visualization**: Generate execution time, memory usage, and algorithm comparison charts
- **Automated report generation**: Create detailed performance analysis reports in markdown format
- **CSV export**: Export benchmark data for further analysis in external tools
- **Memory-aware charting**: Handle cases where memory usage data may not be available
- **Algorithm comparison**: Compare different algorithms at specific data sizes

## Dependencies

```toml
[dependencies]
plotters = "0.3"
serde_json = "1.0"
```

## Core Functions

### `generate_performance_charts(input_file, output_file)`

Creates a comprehensive performance visualization with three charts:

**Parameters:**

- `input_file`: Path to JSON file containing benchmark results
- `output_file`: Path where the generated chart image will be saved

**Chart Layout:**

- **Upper Left**: Execution Time vs Data Size (line chart)
- **Upper Right**: Memory Usage vs Data Size (line chart)
- **Bottom**: Algorithm Comparison at most common data size (bar chart)

**Features:**

- Supports up to 6 different algorithms with distinct colors
- Automatically scales axes based on data ranges
- Handles missing memory data gracefully
- Generates 1200x800 pixel bitmap output

### `generate_performance_report(results, output_file)`

Creates a detailed markdown report with performance analysis.

**Report Sections:**

- **Summary Statistics**: Total benchmarks, unique algorithms, data sizes tested
- **Best Performance Analysis**: Identifies fastest algorithm and most memory-efficient algorithm

**Output Format:**

```markdown
# Performance Analysis Report

## Summary Statistics
Total benchmarks: 15
Unique algorithms: 3
Data sizes tested: [100, 500, 1000, 5000, 10000]

## Best Performance Analysis
**Fastest algorithm**: QuickSort (2.45ms for 1000 elements)
**Most memory efficient**: MergeSort (1.2MB for 1000 elements)
```

### `generate_csv_summary(results, output_file)`

Exports benchmark data to CSV format for external analysis tools.

**CSV Structure:**

```csv
Algorithm,DataSize,ExecutionTime(ms),MemoryUsed(MB),Parallel
QuickSort,1000,2.450,1.20,false
MergeSort,1000,3.120,1.15,false
ParallelSort,1000,1.890,2.40,true
```

## Chart Details

### Execution Time Chart

- **X-axis**: Data size (number of elements)
- **Y-axis**: Execution time in milliseconds
- **Series**: One line per algorithm, color-coded
- **Features**: Automatic scaling, sorted data points, legend

### Memory Usage Chart

- **X-axis**: Data size (number of elements)
- **Y-axis**: Memory usage in megabytes
- **Behavior**: Shows “No Data Available” message if no memory data exists
- **Conversion**: Automatically converts bytes to MB for readability

### Algorithm Comparison Chart

- **X-axis**: Algorithm names
- **Y-axis**: Execution time in milliseconds
- **Data**: Uses most common data size across all benchmarks
- **Colors**: Blue for parallel algorithms, red for sequential
- **Format**: Bar chart for easy comparison

## Usage Example

```rust
use crate::charts::{generate_performance_charts, generate_performance_report, generate_csv_summary};
use crate::benchmark::BenchmarkResult;

// Generate all performance outputs
fn create_performance_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "benchmark_results.json";
    
    // Generate visual charts
    generate_performance_charts(input_file, "performance_charts.png")?;
    
    // Read results for report generation
    let json_data = std::fs::read_to_string(input_file)?;
    let results: Vec<BenchmarkResult> = serde_json::from_str(&json_data)?;
    
    // Generate markdown report
    generate_performance_report(&results, "performance_report.md")?;
    
    // Generate CSV for external analysis
    generate_csv_summary(&results, "benchmark_summary.csv")?;
    
    Ok(())
}
```

## Input Data Format

The module expects benchmark results in JSON format with the following structure:

```json
[
  {
    "algorithm_name": "QuickSort",
    "data_size": 1000,
    "execution_time": {"secs": 0, "nanos": 2450000},
    "memory_used": 1258291,
    "parallel": false
  }
]
```

**Required Fields:**

- `algorithm_name`: String identifying the algorithm
- `data_size`: Number of elements processed
- `execution_time`: Duration struct with seconds and nanoseconds
- `parallel`: Boolean indicating if algorithm uses parallelization

**Optional Fields:**

- `memory_used`: Memory consumption in bytes (can be null)

## Error Handling

The module provides comprehensive error handling for:

- **File I/O errors**: Invalid input/output file paths
- **JSON parsing errors**: Malformed benchmark data
- **Chart rendering errors**: Issues with drawing operations
- **Empty data sets**: Graceful handling of missing data

## Color Scheme

The module uses a predefined color palette for consistency:

- Red, Blue, Green, Magenta, Cyan, Black
- Colors cycle automatically for more than 6 algorithms
- Parallel algorithms use blue, sequential use red in comparison charts

## Output Files

1. **Chart Image**: High-resolution bitmap (1200x800) showing all three charts
2. **Markdown Report**: Structured analysis with statistics and insights
3. **CSV Summary**: Raw data export for spreadsheet analysis

## Performance Considerations

- **Memory efficiency**: Processes data in chunks to handle large datasets
- **Scalable rendering**: Automatically adjusts chart scales based on data ranges
- **Robust parsing**: Handles various JSON formats and missing optional fields
- **Color management**: Efficient color cycling for unlimited algorithm count

## Integration

This module is designed to integrate with benchmark systems that produce `BenchmarkResult` structures. It provides a complete solution for performance analysis workflows, from raw data to publication-ready visualizations.