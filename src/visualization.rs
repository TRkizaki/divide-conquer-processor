use plotters::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::fs;

use crate::benchmark::BenchmarkResult;

/// Generate performance charts from benchmark results
pub fn generate_performance_charts(input_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read benchmark results from JSON file
    let json_data = fs::read_to_string(input_file)?;
    let results: Vec<BenchmarkResult> = serde_json::from_str(&json_data)?;
    
    // Create simple text-based report for now
    let mut report = String::new();
    report.push_str("Performance Analysis Report\n");
    report.push_str("============================\n\n");
    
    for result in &results {
        report.push_str(&format!(
            "Algorithm: {}\nData Size: {}\nExecution Time: {:.2}ms\n\n",
            result.algorithm_name,
            result.data_size,
            result.execution_time.as_secs_f64() * 1000.0
        ));
    }
    
    fs::write(output_file, report)?;
    println!("Performance report generated at {}", output_file);
    
    Ok(())
}

/// Generate detailed performance report
pub fn generate_performance_report(results: &[BenchmarkResult], output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut report = String::new();
    
    report.push_str("# Performance Analysis Report\n\n");
    
    // Summary statistics
    report.push_str("## Summary Statistics\n\n");
    report.push_str(&format!("Total benchmarks: {}\n", results.len()));
    
    let unique_algorithms: std::collections::HashSet<_> = results.iter()
        .map(|r| &r.algorithm_name)
        .collect();
    report.push_str(&format!("Unique algorithms: {}\n", unique_algorithms.len()));
    
    // Best performance analysis
    if let Some(fastest) = results.iter().min_by_key(|r| r.execution_time) {
        report.push_str(&format!(
            "\n**Fastest algorithm**: {} ({:.2}ms for {} elements)\n",
            fastest.algorithm_name,
            fastest.execution_time.as_secs_f64() * 1000.0,
            fastest.data_size
        ));
    }
    
    // Write report to file
    fs::write(output_file, report)?;
    println!("Performance report generated at {}", output_file);
    
    Ok(())
}

/// Generate CSV summary for further analysis
pub fn generate_csv_summary(results: &[BenchmarkResult], output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut csv_content = String::from("Algorithm,DataSize,ExecutionTime(ms),MemoryUsed(MB),Parallel\n");
    
    for result in results {
        csv_content.push_str(&format!(
            "{},{},{:.3},{},{}\n",
            result.algorithm_name,
            result.data_size,
            result.execution_time.as_secs_f64() * 1000.0,
            result.memory_used.map_or("N/A".to_string(), |m| format!("{:.2}", m as f64 / 1024.0 / 1024.0)),
            result.parallel
        ));
    }
    
    fs::write(output_file, csv_content)?;
    Ok(())
}