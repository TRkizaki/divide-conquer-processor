use colored::*;
use memory_stats::memory_stats;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::geometry::Point;
use crate::matrix::Matrix;
use crate::sorting;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub algorithm_name: String,
    pub data_size: usize,
    pub execution_time: Duration,
    pub memory_used: Option<usize>,
    pub parallel: bool,
}

pub struct BenchmarkRunner {
    results: Vec<BenchmarkResult>,
}

impl BenchmarkRunner {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Measure memory usage
    fn measure_memory() -> Option<usize> {
        memory_stats().map(|stats| stats.physical_mem)
    }

    /// Benchmark sorting algorithms
    pub fn benchmark_sort(&mut self, algorithm: &str, data: &[i32], runs: usize, parallel: bool) {
        let mut total_time = Duration::new(0, 0);
        let mut memory_usage = None;
        
        println!("{}", format!("  Testing {}...", algorithm).cyan());
        
        for run in 0..runs {
            let mut test_data = data.to_vec();
            
            // Start memory measurement
            let memory_before = Self::measure_memory();
            
            let start = Instant::now();
            
            match algorithm {
                "Merge Sort" => {
                    if parallel {
                        sorting::parallel_merge_sort(&mut test_data);
                    } else {
                        sorting::merge_sort(&mut test_data);
                    }
                }
                "Quick Sort" => {
                    if parallel {
                        sorting::parallel_quick_sort(&mut test_data);
                    } else {
                        sorting::quick_sort(&mut test_data);
                    }
                }
                _ => panic!("Unknown sorting algorithm: {}", algorithm),
            }
            
            let elapsed = start.elapsed();
            total_time += elapsed;
            
            // End memory measurement
            if let (Some(before), Some(after)) = (memory_before, Self::measure_memory()) {
                if after > before {
                    memory_usage = Some(after - before);
                }
            }
            
            print!(".");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
        
        println!();
        
        let avg_time = total_time / runs as u32;
        
        let result = BenchmarkResult {
            algorithm_name: format!("{}{}", algorithm, if parallel { " (Parallel)" } else { "" }),
            data_size: data.len(),
            execution_time: avg_time,
            memory_used: memory_usage,
            parallel,
        };
        
        self.results.push(result);
        
        println!(
            "    {}: {:.2}ms",
            if parallel { "Parallel" } else { "Sequential" },
            avg_time.as_secs_f64() * 1000.0
        );
    }

    /// Benchmark matrix multiplication
    pub fn benchmark_matrix_multiply(
        &mut self,
        algorithm: &str,
        matrix_a: &Matrix,
        matrix_b: &Matrix,
        use_strassen: bool,
    ) {
        println!("{}", format!("  Testing {}...", algorithm).cyan());
        
        let memory_before = Self::measure_memory();
        let start = Instant::now();
        
        let _result = if use_strassen {
            crate::matrix::strassen_multiply(matrix_a, matrix_b)
        } else {
            crate::matrix::standard_multiply(matrix_a, matrix_b)
        };
        
        let elapsed = start.elapsed();
        let memory_usage = memory_before
            .zip(Self::measure_memory())
            .and_then(|(before, after)| if after > before { Some(after - before) } else { None });
        
        let result = BenchmarkResult {
            algorithm_name: format!(
                "{}{}",
                algorithm,
                if use_strassen { " (Strassen)" } else { " (Standard)" }
            ),
            data_size: matrix_a.size(),
            execution_time: elapsed,
            memory_used: memory_usage,
            parallel: false,
        };
        
        self.results.push(result);
        
        println!(
            "    {}: {:.2}ms",
            if use_strassen { "Strassen" } else { "Standard" },
            elapsed.as_secs_f64() * 1000.0
        );
    }

    /// Benchmark closest pair problem
    pub fn benchmark_closest_pair(&mut self, algorithm: &str, points: &[Point]) {
        println!("{}", format!("  Testing {}...", algorithm).cyan());
        
        let memory_before = Self::measure_memory();
        let start = Instant::now();
        
        let _result = crate::geometry::closest_pair_divide_conquer(points);
        
        let elapsed = start.elapsed();
        let memory_usage = memory_before
            .zip(Self::measure_memory())
            .and_then(|(before, after)| if after > before { Some(after - before) } else { None });
        
        let result = BenchmarkResult {
            algorithm_name: algorithm.to_string(),
            data_size: points.len(),
            execution_time: elapsed,
            memory_used: memory_usage,
            parallel: false,
        };
        
        self.results.push(result);
        
        println!(
            "    Divide & Conquer: {:.2}ms",
            elapsed.as_secs_f64() * 1000.0
        );
    }

    /// Display benchmark results
    pub fn display_results(&self) {
        if self.results.is_empty() {
            println!("{}", "No benchmark results available".yellow());
            return;
        }
        
        println!("\n{}", "=== Benchmark Results ===".bright_green().bold());
        
        // Group results by algorithm
        let mut grouped_results = HashMap::new();
        for result in &self.results {
            grouped_results
                .entry(result.algorithm_name.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }
        
        for (algorithm, results) in grouped_results {
            println!("\n{}", format!("--- {} ---", algorithm).bright_yellow());
            
            for result in results {
                println!(
                    "Data size: {}, Execution time: {:.2}ms{}",
                    result.data_size,
                    result.execution_time.as_secs_f64() * 1000.0,
                    if let Some(mem) = result.memory_used {
                        format!(", Memory usage: {:.2}MB", mem as f64 / 1024.0 / 1024.0)
                    } else {
                        String::new()
                    }
                );
            }
        }
        
        // Display best performance
        if let Some(fastest) = self.results.iter().min_by_key(|r| r.execution_time) {
            println!(
                "\n{}: {} ({:.2}ms)",
                "Best Performance".bright_green().bold(),
                fastest.algorithm_name,
                fastest.execution_time.as_secs_f64() * 1000.0
            );
        }
    }

    /// Save results as JSON
    pub fn save_results(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.results)?;
        std::fs::write(filename, json)?;
        Ok(())
    }

    /// Save results as CSV
    pub fn save_results_csv(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_content = String::from("Algorithm,DataSize,ExecutionTime(ms),MemoryUsed(MB),Parallel\n");
        
        for result in &self.results {
            csv_content.push_str(&format!(
                "{},{},{:.3},{},{}\n",
                result.algorithm_name,
                result.data_size,
                result.execution_time.as_secs_f64() * 1000.0,
                result.memory_used.map_or("N/A".to_string(), |m| format!("{:.2}", m as f64 / 1024.0 / 1024.0)),
                result.parallel
            ));
        }
        
        std::fs::write(filename, csv_content)?;
        Ok(())
    }

    /// Get benchmark results
    pub fn get_results(&self) -> &[BenchmarkResult] {
        &self.results
    }
}
        &self.results
    }
}
        &self.results
    }
}
    