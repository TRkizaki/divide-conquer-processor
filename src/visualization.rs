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
    
    // Create the output file
    let root = BitMapBackend::new(output_file, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    
    // Split the drawing area into multiple charts
    let (upper, lower) = root.split_evenly((2, 1));
    let (execution_chart, memory_chart) = upper.split_evenly((1, 2));
    
    // Generate execution time chart
    draw_execution_time_chart(execution_chart, &results)?;
    
    // Generate memory usage chart
    draw_memory_usage_chart(memory_chart, &results)?;
    
    // Generate algorithm comparison chart
    draw_algorithm_comparison_chart(lower, &results)?;
    
    root.present()?;
    println!("Performance charts generated successfully at {}", output_file);
    
    Ok(())
}

fn draw_execution_time_chart(
    drawing_area: DrawingArea<BitMapBackend, plotters::coord::Shift>,
    results: &[BenchmarkResult],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Execution Time vs Data Size", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(
            0usize..results.iter().map(|r| r.data_size).max().unwrap_or(1000),
            0f64..results.iter().map(|r| r.execution_time.as_secs_f64() * 1000.0).fold(0.0, f64::max),
        )?;

    chart.configure_mesh()
        .x_desc("Data Size")
        .y_desc("Execution Time (ms)")
        .draw()?;

    // Group results by algorithm
    let mut algorithm_data: HashMap<String, Vec<(usize, f64)>> = HashMap::new();
    
    for result in results {
        let time_ms = result.execution_time.as_secs_f64() * 1000.0;
        algorithm_data
            .entry(result.algorithm_name.clone())
            .or_insert_with(Vec::new)
            .push((result.data_size, time_ms));
    }

    let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];
    let mut color_idx = 0;

    for (algorithm, data) in algorithm_data.iter_mut() {
        data.sort_by_key(|&(size, _)| size);
        
        chart
            .draw_series(LineSeries::new(data.iter().cloned(), colors[color_idx % colors.len()]))?
            .label(algorithm)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], colors[color_idx % colors.len()]));
        
        color_idx += 1;
    }

    chart.configure_series_labels().draw()?;
    
    Ok(())
}

fn draw_memory_usage_chart(
    drawing_area: DrawingArea<BitMapBackend, plotters::coord::Shift>,
    results: &[BenchmarkResult],
) -> Result<(), Box<dyn std::error::Error>> {
    // Filter results that have memory usage data
    let memory_results: Vec<_> = results.iter()
        .filter(|r| r.memory_used.is_some())
        .collect();
    
    if memory_results.is_empty() {
        // Draw empty chart with message
        let mut chart = ChartBuilder::on(&drawing_area)
            .caption("Memory Usage (No Data Available)", ("sans-serif", 30))
            .margin(5)
            .build_cartesian_2d(0..1, 0..1)?;
        
        chart.draw_series(std::iter::once(Text::new("No memory usage data available", (0, 0), ("sans-serif", 20))))?;
        return Ok(());
    }

    let max_memory = memory_results.iter()
        .map(|r| r.memory_used.unwrap() as f64 / 1024.0 / 1024.0)
        .fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption("Memory Usage vs Data Size", ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(
            0usize..memory_results.iter().map(|r| r.data_size).max().unwrap_or(1000),
            0f64..max_memory,
        )?;

    chart.configure_mesh()
        .x_desc("Data Size")
        .y_desc("Memory Usage (MB)")
        .draw()?;

    // Group memory results by algorithm
    let mut algorithm_memory: HashMap<String, Vec<(usize, f64)>> = HashMap::new();
    
    for result in memory_results {
        if let Some(memory) = result.memory_used {
            let memory_mb = memory as f64 / 1024.0 / 1024.0;
            algorithm_memory
                .entry(result.algorithm_name.clone())
                .or_insert_with(Vec::new)
                .push((result.data_size, memory_mb));
        }
    }

    let colors = [&RED, &BLUE, &GREEN, &MAGENTA, &CYAN, &BLACK];
    let mut color_idx = 0;

    for (algorithm, data) in algorithm_memory.iter_mut() {
        data.sort_by_key(|&(size, _)| size);
        
        chart
            .draw_series(LineSeries::new(data.iter().cloned(), colors[color_idx % colors.len()]))?
            .label(algorithm)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], colors[color_idx % colors.len()]));
        
        color_idx += 1;
    }

    chart.configure_series_labels().draw()?;
    
    Ok(())
}

fn draw_algorithm_comparison_chart(
    drawing_area: DrawingArea<BitMapBackend, plotters::coord::Shift>,
    results: &[BenchmarkResult],
) -> Result<(), Box<dyn std::error::Error>> {
    // Group results by data size and find common sizes
    let mut size_groups: HashMap<usize, Vec<&BenchmarkResult>> = HashMap::new();
    
    for result in results {
        size_groups
            .entry(result.data_size)
            .or_insert_with(Vec::new)
            .push(result);
    }
    
    // Find the most common data size for comparison
    let comparison_size = size_groups.iter()
        .max_by_key(|(_, results)| results.len())
        .map(|(size, _)| *size)
        .unwrap_or(1000);
    
    let comparison_results = size_groups.get(&comparison_size).unwrap_or(&Vec::new());
    
    if comparison_results.is_empty() {
        let mut chart = ChartBuilder::on(&drawing_area)
            .caption("Algorithm Comparison (No Data Available)", ("sans-serif", 30))
            .margin(5)
            .build_cartesian_2d(0..1, 0..1)?;
        
        chart.draw_series(std::iter::once(Text::new("No comparison data available", (0, 0), ("sans-serif", 20))))?;
        return Ok(());
    }

    let max_time = comparison_results.iter()
        .map(|r| r.execution_time.as_secs_f64() * 1000.0)
        .fold(0.0, f64::max);

    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(&format!("Algorithm Comparison (Data Size: {})", comparison_size), ("sans-serif", 30))
        .margin(5)
        .x_label_area_size(60)
        .y_label_area_size(50)
        .build_cartesian_2d(
            0..comparison_results.len(),
            0f64..max_time,
        )?;

    chart.configure_mesh()
        .x_desc("Algorithms")
        .y_desc("Execution Time (ms)")
        .x_label_formatter(&|x| {
            comparison_results.get(*x)
                .map(|r| r.algorithm_name.clone())
                .unwrap_or_else(|| "".to_string())
        })
        .draw()?;

    // Draw bars for each algorithm
    let bar_width = 0.8;
    
    for (i, result) in comparison_results.iter().enumerate() {
        let time_ms = result.execution_time.as_secs_f64() * 1000.0;
        let color = if result.parallel { &BLUE } else { &RED };
        
        chart.draw_series(Rectangle::new([(i, 0.0), (i, time_ms)], color.filled()))?
            .label(&result.algorithm_name);
    }

    chart.configure_series_labels().draw()?;
    
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
    
    let data_sizes: std::collections::HashSet<_> = results.iter()
        .map(|r| r.data_size)
        .collect();
    report.push_str(&format!("Data sizes tested: {:?}\n\n", {
        let mut sizes: Vec<_> = data_sizes.into_iter().collect();
        sizes.sort();
        sizes
    }));
    
    // Best performance analysis
    report.push_str("## Best Performance Analysis\n\n");
    
    if let Some(fastest) = results.iter().min_by_key(|r| r.execution_time) {
        report.push_str(&format!(
            "**Fastest algorithm**: {} ({:.2}ms for {} elements)\n",
            fastest.algorithm_name,
            fastest.execution_time.as_secs_f64() * 1000.0,
            fastest.data_size
        ));
    }
    
    if let Some(most_memory_efficient) = results.iter()
        .filter(|r| r.memory_used.is_some())
        .min_by_key(|r| r.memory_used.unwrap()) {
        report.push_str(&format!(
            "**Most memory efficient**: {} ({:.2}MB for {} elements)\n",
            most_memory_efficient.algorithm_name,
            most_memory_efficient.memory_used.unwrap() as f64 / 1024.0 / 1024.0,
            most_memory_efficient.data_size
        ));
    }
    
    // Algorithm-specific analysis
    report.push_str("\n## Algorithm-Specific Analysis\n\n");
    
    let mut algorithm_groups: HashMap<String, Vec<&BenchmarkResult>> = HashMap::new();
    for result in results {
        algorithm_groups
            .entry(result.algorithm_name.clone())
            .or_insert_with(Vec::new)
            .push(result);
    }
    
    for (algorithm, results) in algorithm_groups {
        report.push_str(&format!("### {}\n\n", algorithm));
        
        let avg_time = results.iter()
            .map(|r| r.execution_time.as_secs_f64() * 1000.0)
            .sum::<f64>() / results.len() as f64;
        
        report.push_str(&format!("- Average execution time: {:.2}ms\n", avg_time));
        report.push_str(&format!("- Number of tests: {}\n", results.len()));
        
        if let Some(memory_results) = results.iter()
            .filter(|r| r.memory_used.is_some())
            .map(|r| r.memory_used.unwrap() as f64 / 1024.0 / 1024.0)
            .collect::<Vec<_>>()
            .get(0) {
            let avg_memory = results.iter()
                .filter_map(|r| r.memory_used)
                .map(|m| m as f64 / 1024.0 / 1024.0)
                .sum::<f64>() / results.iter().filter(|r| r.memory_used.is_some()).count() as f64;
            report.push_str(&format!("- Average memory usage: {:.2}MB\n", avg_memory));
        }
        
        report.push_str("\n");
    }
    
    // Scaling analysis
    report.push_str("## Scaling Analysis\n\n");
    
    // Group by data size and analyze scaling
    let mut size_groups: HashMap<usize, Vec<&BenchmarkResult>> = HashMap::new();
    for result in results {
        size_groups
            .entry(result.data_size)
            .or_insert_with(Vec::new)
            .push(result);
    }
    
    let mut sizes: Vec<_> = size_groups.keys().cloned().collect();
    sizes.sort();
    
    for algorithm in unique_algorithms {
        let algorithm_results: Vec<_> = results.iter()
            .filter(|r| r.algorithm_name == *algorithm)
            .collect();
        
        if algorithm_results.len() >= 2 {
            report.push_str(&format!("### {} Scaling\n\n", algorithm));
            
            for size in &sizes {
                if let Some(result) = algorithm_results.iter().find(|r| r.data_size == *size) {
                    report.push_str(&format!(
                        "- Size {}: {:.2}ms\n",
                        size,
                        result.execution_time.as_secs_f64() * 1000.0
                    ));
                }
            }
            report.push_str("\n");
        }
    }
    
    // Write report to file
    fs::write(output_file, report)?;
    println!("Performance report generated at {}", output_file);
    
    Ok(())
}

/// Generate CSV summary for further analysis
pub fn generate_csv_summary(results: &[BenchmarkResult], output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut csv_content = String::from("Algorithm,DataSize,ExecutionTime(ms),MemoryUsed(MB),Parallel,Category\n");