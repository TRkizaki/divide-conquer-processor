use colored::*;
use memory_stats::memory_stats;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use statistical::{mean, standard_deviation};

use crate::sorting;
use crate::data_generator::DataGenerator;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSpecs {
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub cpu_max_freq_mhz: f64,
    pub memory_total_gb: f64,
    pub os_version: String,
    pub rust_version: String,
    pub arch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedBenchmarkResult {
    pub algorithm_name: String,
    pub data_size: usize,
    pub runs: usize,
    pub mean_time_ms: f64,
    pub std_dev_time_ms: f64,
    pub min_time_ms: f64,
    pub max_time_ms: f64,
    pub median_time_ms: f64,
    pub mean_memory_mb: Option<f64>,
    pub parallel: bool,
    pub speedup_vs_sequential: Option<f64>,
    pub efficiency: Option<f64>,
    pub individual_runs_ms: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalabilityResult {
    pub algorithm_name: String,
    pub data_sizes: Vec<usize>,
    pub execution_times_ms: Vec<f64>,
    pub memory_usage_mb: Vec<Option<f64>>,
    pub parallel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelEfficiencyResult {
    pub algorithm_name: String,
    pub data_size: usize,
    pub thread_counts: Vec<usize>,
    pub execution_times_ms: Vec<f64>,
    pub speedups: Vec<f64>,
    pub efficiencies: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveBenchmarkReport {
    pub system_specs: SystemSpecs,
    pub benchmark_date: String,
    pub methodology: String,
    pub detailed_results: Vec<DetailedBenchmarkResult>,
    pub scalability_results: Vec<ScalabilityResult>,
    pub parallel_efficiency_results: Vec<ParallelEfficiencyResult>,
    pub comparative_analysis: HashMap<String, f64>, // Algorithm vs std library speedup
}

pub struct ComprehensiveBenchmarkRunner {
    results: Vec<DetailedBenchmarkResult>,
    scalability_results: Vec<ScalabilityResult>,
    parallel_efficiency_results: Vec<ParallelEfficiencyResult>,
    system_specs: SystemSpecs,
}

impl ComprehensiveBenchmarkRunner {
    pub fn new() -> Self {
        let system_specs = Self::collect_system_specs();
        println!("{}", "=== System Specifications ===".bright_green().bold());
        println!("CPU: {} ({} cores, {} threads)", system_specs.cpu_model, system_specs.cpu_cores, system_specs.cpu_threads);
        println!("Memory: {:.1} GB", system_specs.memory_total_gb);
        println!("OS: {}", system_specs.os_version);
        println!("Rust: {}", system_specs.rust_version);
        println!("Architecture: {}", system_specs.arch);
        println!();

        Self {
            results: Vec::new(),
            scalability_results: Vec::new(),
            parallel_efficiency_results: Vec::new(),
            system_specs,
        }
    }

    fn collect_system_specs() -> SystemSpecs {
        SystemSpecs {
            cpu_model: "13th Gen Intel(R) Core(TM) i7-13650HX".to_string(),
            cpu_cores: 14,
            cpu_threads: 20,
            cpu_max_freq_mhz: 4900.0,
            memory_total_gb: 24.0,
            os_version: "Linux 6.12.10-76061203-generic (Pop!_OS)".to_string(),
            rust_version: "1.89.0".to_string(),
            arch: "x86_64".to_string(),
        }
    }

    fn measure_memory() -> Option<usize> {
        memory_stats().map(|stats| stats.physical_mem)
    }

    /// Run comprehensive sorting benchmarks including std library comparisons
    pub fn benchmark_sorting_comprehensive(&mut self, data_sizes: &[usize], runs: usize) {
        println!("{}", "=== Comprehensive Sorting Benchmarks ===".bright_green().bold());
        
        for &size in data_sizes {
            println!("{}", format!("\nTesting data size: {}", size).bright_yellow());
            
            let test_data = DataGenerator::generate_random_integers(size);
            
            // Benchmark our implementations
            self.benchmark_sort_algorithm("Merge Sort", &test_data, runs, false);
            self.benchmark_sort_algorithm("Merge Sort", &test_data, runs, true);
            self.benchmark_sort_algorithm("Quick Sort", &test_data, runs, false);
            self.benchmark_sort_algorithm("Quick Sort", &test_data, runs, true);
            
            // Benchmark std library implementations
            self.benchmark_std_sort("std::slice::sort", &test_data, runs);
            self.benchmark_std_sort("std::slice::sort_unstable", &test_data, runs);
            self.benchmark_std_sort("std::slice::sort_by", &test_data, runs);
            
            // Benchmark rayon parallel sort
            self.benchmark_rayon_sort("rayon::par_sort", &test_data, runs);
            self.benchmark_rayon_sort("rayon::par_sort_unstable", &test_data, runs);
        }
    }

    fn benchmark_sort_algorithm(&mut self, algorithm: &str, data: &[i32], runs: usize, parallel: bool) {
        println!("{}", format!("  Testing {} ({})...", algorithm, if parallel { "Parallel" } else { "Sequential" }).cyan());
        
        let mut times = Vec::new();
        let mut memory_usages = Vec::new();
        
        for run in 0..runs {
            let mut test_data = data.to_vec();
            
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
                _ => panic!("Unknown algorithm: {}", algorithm),
            }
            
            let elapsed = start.elapsed();
            times.push(elapsed.as_secs_f64() * 1000.0);
            
            if let (Some(before), Some(after)) = (memory_before, Self::measure_memory()) {
                if after > before {
                    memory_usages.push((after - before) as f64 / 1024.0 / 1024.0);
                }
            }
            
            if run % (runs / 10).max(1) == 0 {
                print!(".");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
            }
        }
        
        println!();
        
        let mean_time = mean(&times);
        let std_dev = standard_deviation(&times, Some(mean_time));
        let mut sorted_times = times.clone();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let result = DetailedBenchmarkResult {
            algorithm_name: format!("{}{}", algorithm, if parallel { " (Parallel)" } else { "" }),
            data_size: data.len(),
            runs,
            mean_time_ms: mean_time,
            std_dev_time_ms: std_dev,
            min_time_ms: sorted_times[0],
            max_time_ms: sorted_times[sorted_times.len() - 1],
            median_time_ms: sorted_times[sorted_times.len() / 2],
            mean_memory_mb: if memory_usages.is_empty() { None } else { Some(mean(&memory_usages)) },
            parallel,
            speedup_vs_sequential: None,
            efficiency: None,
            individual_runs_ms: times,
        };
        
        self.results.push(result);
        
        println!(
            "    Mean: {:.2}ms ± {:.2}ms, Median: {:.2}ms, Range: {:.2}-{:.2}ms",
            mean_time, std_dev, sorted_times[sorted_times.len() / 2], sorted_times[0], sorted_times[sorted_times.len() - 1]
        );
    }

    fn benchmark_std_sort(&mut self, algorithm: &str, data: &[i32], runs: usize) {
        println!("{}", format!("  Testing {}...", algorithm).cyan());
        
        let mut times = Vec::new();
        
        for run in 0..runs {
            let mut test_data = data.to_vec();
            
            let start = Instant::now();
            
            match algorithm {
                "std::slice::sort" => test_data.sort(),
                "std::slice::sort_unstable" => test_data.sort_unstable(),
                "std::slice::sort_by" => test_data.sort_by(|a, b| a.cmp(b)),
                _ => panic!("Unknown std algorithm: {}", algorithm),
            }
            
            let elapsed = start.elapsed();
            times.push(elapsed.as_secs_f64() * 1000.0);
            
            if run % (runs / 10).max(1) == 0 {
                print!(".");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
            }
        }
        
        println!();
        
        let mean_time = mean(&times);
        let std_dev = standard_deviation(&times, Some(mean_time));
        let mut sorted_times = times.clone();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let result = DetailedBenchmarkResult {
            algorithm_name: algorithm.to_string(),
            data_size: data.len(),
            runs,
            mean_time_ms: mean_time,
            std_dev_time_ms: std_dev,
            min_time_ms: sorted_times[0],
            max_time_ms: sorted_times[sorted_times.len() - 1],
            median_time_ms: sorted_times[sorted_times.len() / 2],
            mean_memory_mb: None,
            parallel: false,
            speedup_vs_sequential: None,
            efficiency: None,
            individual_runs_ms: times,
        };
        
        self.results.push(result);
        
        println!(
            "    Mean: {:.2}ms ± {:.2}ms, Median: {:.2}ms",
            mean_time, std_dev, sorted_times[sorted_times.len() / 2]
        );
    }

    fn benchmark_rayon_sort(&mut self, algorithm: &str, data: &[i32], runs: usize) {
        println!("{}", format!("  Testing {}...", algorithm).cyan());
        
        let mut times = Vec::new();
        
        for run in 0..runs {
            let mut test_data = data.to_vec();
            
            let start = Instant::now();
            
            match algorithm {
                "rayon::par_sort" => test_data.par_sort(),
                "rayon::par_sort_unstable" => test_data.par_sort_unstable(),
                _ => panic!("Unknown rayon algorithm: {}", algorithm),
            }
            
            let elapsed = start.elapsed();
            times.push(elapsed.as_secs_f64() * 1000.0);
            
            if run % (runs / 10).max(1) == 0 {
                print!(".");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
            }
        }
        
        println!();
        
        let mean_time = mean(&times);
        let std_dev = standard_deviation(&times, Some(mean_time));
        let mut sorted_times = times.clone();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let result = DetailedBenchmarkResult {
            algorithm_name: algorithm.to_string(),
            data_size: data.len(),
            runs,
            mean_time_ms: mean_time,
            std_dev_time_ms: std_dev,
            min_time_ms: sorted_times[0],
            max_time_ms: sorted_times[sorted_times.len() - 1],
            median_time_ms: sorted_times[sorted_times.len() / 2],
            mean_memory_mb: None,
            parallel: true,
            speedup_vs_sequential: None,
            efficiency: None,
            individual_runs_ms: times,
        };
        
        self.results.push(result);
        
        println!(
            "    Mean: {:.2}ms ± {:.2}ms, Median: {:.2}ms",
            mean_time, std_dev, sorted_times[sorted_times.len() / 2]
        );
    }

    /// Perform scalability analysis across different data sizes
    pub fn analyze_scalability(&mut self, algorithm: &str, size_range: &[usize], parallel: bool) {
        println!("{}", format!("=== Scalability Analysis: {} ===", algorithm).bright_green().bold());
        
        let mut times = Vec::new();
        let mut memory_usages = Vec::new();
        
        for &size in size_range {
            println!("{}", format!("Testing size: {}", size).bright_yellow());
            
            let test_data = DataGenerator::generate_random_integers(size);
            let mut test_data_copy = test_data.clone();
            
            let memory_before = Self::measure_memory();
            let start = Instant::now();
            
            match algorithm {
                "Merge Sort" => {
                    if parallel {
                        sorting::parallel_merge_sort(&mut test_data_copy);
                    } else {
                        sorting::merge_sort(&mut test_data_copy);
                    }
                }
                "Quick Sort" => {
                    if parallel {
                        sorting::parallel_quick_sort(&mut test_data_copy);
                    } else {
                        sorting::quick_sort(&mut test_data_copy);
                    }
                }
                _ => panic!("Unknown algorithm: {}", algorithm),
            }
            
            let elapsed = start.elapsed();
            times.push(elapsed.as_secs_f64() * 1000.0);
            
            let memory_usage = memory_before
                .zip(Self::measure_memory())
                .and_then(|(before, after)| {
                    if after > before {
                        Some((after - before) as f64 / 1024.0 / 1024.0)
                    } else {
                        None
                    }
                });
            memory_usages.push(memory_usage);
            
            println!("  Time: {:.2}ms", elapsed.as_secs_f64() * 1000.0);
        }
        
        let result = ScalabilityResult {
            algorithm_name: format!("{}{}", algorithm, if parallel { " (Parallel)" } else { "" }),
            data_sizes: size_range.to_vec(),
            execution_times_ms: times,
            memory_usage_mb: memory_usages,
            parallel,
        };
        
        self.scalability_results.push(result);
    }

    /// Analyze parallel efficiency across different thread counts
    pub fn analyze_parallel_efficiency(&mut self, algorithm: &str, data_size: usize, thread_counts: &[usize]) {
        println!("{}", format!("=== Parallel Efficiency Analysis: {} ===", algorithm).bright_green().bold());
        
        let test_data = DataGenerator::generate_random_integers(data_size);
        
        // First get sequential baseline
        let mut sequential_data = test_data.clone();
        let sequential_start = Instant::now();
        
        match algorithm {
            "Merge Sort" => sorting::merge_sort(&mut sequential_data),
            "Quick Sort" => sorting::quick_sort(&mut sequential_data),
            _ => panic!("Unknown algorithm: {}", algorithm),
        }
        
        let sequential_time = sequential_start.elapsed().as_secs_f64() * 1000.0;
        println!("Sequential baseline: {:.2}ms", sequential_time);
        
        let mut times = Vec::new();
        let mut speedups = Vec::new();
        let mut efficiencies = Vec::new();
        
        for &thread_count in thread_counts {
            println!("{}", format!("Testing with {} threads", thread_count).bright_yellow());
            
            // Set thread pool size
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread_count)
                .build()
                .unwrap();
            
            let parallel_time = pool.install(|| {
                let mut parallel_data = test_data.clone();
                let start = Instant::now();
                
                match algorithm {
                    "Merge Sort" => sorting::parallel_merge_sort(&mut parallel_data),
                    "Quick Sort" => sorting::parallel_quick_sort(&mut parallel_data),
                    _ => panic!("Unknown algorithm: {}", algorithm),
                }
                
                start.elapsed().as_secs_f64() * 1000.0
            });
            
            let speedup = sequential_time / parallel_time;
            let efficiency = speedup / thread_count as f64;
            
            times.push(parallel_time);
            speedups.push(speedup);
            efficiencies.push(efficiency);
            
            println!("  Time: {:.2}ms, Speedup: {:.2}x, Efficiency: {:.2}%", 
                    parallel_time, speedup, efficiency * 100.0);
        }
        
        let result = ParallelEfficiencyResult {
            algorithm_name: algorithm.to_string(),
            data_size,
            thread_counts: thread_counts.to_vec(),
            execution_times_ms: times,
            speedups,
            efficiencies,
        };
        
        self.parallel_efficiency_results.push(result);
    }

    /// Calculate speedups relative to sequential implementations
    pub fn calculate_speedups(&mut self) {
        let mut sequential_baselines: HashMap<String, f64> = HashMap::new();
        
        // Find sequential baselines
        for result in &self.results {
            if !result.parallel {
                let base_name = result.algorithm_name.clone();
                sequential_baselines.insert(
                    format!("{}_{}", base_name, result.data_size),
                    result.mean_time_ms
                );
            }
        }
        
        // Update parallel results with speedups
        for result in &mut self.results {
            if result.parallel {
                let base_name = result.algorithm_name.replace(" (Parallel)", "");
                let key = format!("{}_{}", base_name, result.data_size);
                
                if let Some(&sequential_time) = sequential_baselines.get(&key) {
                    let speedup = sequential_time / result.mean_time_ms;
                    let efficiency = speedup / self.system_specs.cpu_threads as f64;
                    result.speedup_vs_sequential = Some(speedup);
                    result.efficiency = Some(efficiency);
                }
            }
        }
    }

    /// Generate comprehensive report
    pub fn generate_report(&self) -> ComprehensiveBenchmarkReport {
        let mut comparative_analysis = HashMap::new();
        
        // Calculate speedups vs std library
        for result in &self.results {
            if result.algorithm_name.contains("std::") {
                continue; // Skip std library results for comparison base
            }
            
            // Find corresponding std library result
            let std_result = self.results.iter()
                .find(|r| r.algorithm_name == "std::slice::sort_unstable" && r.data_size == result.data_size);
            
            if let Some(std_result) = std_result {
                let speedup = std_result.mean_time_ms / result.mean_time_ms;
                comparative_analysis.insert(
                    format!("{}_vs_std_size_{}", result.algorithm_name, result.data_size),
                    speedup
                );
            }
        }
        
        ComprehensiveBenchmarkReport {
            system_specs: self.system_specs.clone(),
            benchmark_date: format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()),
            methodology: "Multiple runs with statistical analysis, memory monitoring, and comparative benchmarking".to_string(),
            detailed_results: self.results.clone(),
            scalability_results: self.scalability_results.clone(),
            parallel_efficiency_results: self.parallel_efficiency_results.clone(),
            comparative_analysis,
        }
    }

    /// Save comprehensive results in multiple formats
    pub fn save_comprehensive_results(&self, base_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let report = self.generate_report();
        
        // Save full JSON report
        let json = serde_json::to_string_pretty(&report)?;
        std::fs::write(format!("{}_full_report.json", base_filename), json)?;
        
        // Save detailed CSV
        self.save_detailed_csv(&format!("{}_detailed_results.csv", base_filename))?;
        
        // Save scalability CSV
        self.save_scalability_csv(&format!("{}_scalability.csv", base_filename))?;
        
        // Save parallel efficiency CSV
        self.save_parallel_efficiency_csv(&format!("{}_parallel_efficiency.csv", base_filename))?;
        
        println!("{}", "Comprehensive results saved successfully!".bright_green().bold());
        Ok(())
    }

    fn save_detailed_csv(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_content = String::from(
            "Algorithm,DataSize,Runs,MeanTime(ms),StdDev(ms),MinTime(ms),MaxTime(ms),MedianTime(ms),MeanMemory(MB),Parallel,SpeedupVsSequential,Efficiency\n"
        );
        
        for result in &self.results {
            csv_content.push_str(&format!(
                "{},{},{},{:.3},{:.3},{:.3},{:.3},{:.3},{},{},{},{}\n",
                result.algorithm_name,
                result.data_size,
                result.runs,
                result.mean_time_ms,
                result.std_dev_time_ms,
                result.min_time_ms,
                result.max_time_ms,
                result.median_time_ms,
                result.mean_memory_mb.map_or("N/A".to_string(), |m| format!("{:.2}", m)),
                result.parallel,
                result.speedup_vs_sequential.map_or("N/A".to_string(), |s| format!("{:.3}", s)),
                result.efficiency.map_or("N/A".to_string(), |e| format!("{:.3}", e))
            ));
        }
        
        std::fs::write(filename, csv_content)?;
        Ok(())
    }

    fn save_scalability_csv(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_content = String::from("Algorithm,DataSize,ExecutionTime(ms),MemoryUsage(MB),Parallel\n");
        
        for result in &self.scalability_results {
            for (i, &size) in result.data_sizes.iter().enumerate() {
                csv_content.push_str(&format!(
                    "{},{},{:.3},{},{}\n",
                    result.algorithm_name,
                    size,
                    result.execution_times_ms[i],
                    result.memory_usage_mb[i].map_or("N/A".to_string(), |m| format!("{:.2}", m)),
                    result.parallel
                ));
            }
        }
        
        std::fs::write(filename, csv_content)?;
        Ok(())
    }

    fn save_parallel_efficiency_csv(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_content = String::from("Algorithm,DataSize,ThreadCount,ExecutionTime(ms),Speedup,Efficiency\n");
        
        for result in &self.parallel_efficiency_results {
            for (i, &thread_count) in result.thread_counts.iter().enumerate() {
                csv_content.push_str(&format!(
                    "{},{},{},{:.3},{:.3},{:.3}\n",
                    result.algorithm_name,
                    result.data_size,
                    thread_count,
                    result.execution_times_ms[i],
                    result.speedups[i],
                    result.efficiencies[i]
                ));
            }
        }
        
        std::fs::write(filename, csv_content)?;
        Ok(())
    }
}