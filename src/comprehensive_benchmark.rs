use colored::*;
use memory_stats::memory_stats;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use statistical::{mean, standard_deviation};

use crate::sorting;
use crate::data_generator::DataGenerator;
use crate::geometry::{Point, KdTree, LineSegment, convex_hull_graham_scan, closest_pair_brute_force, closest_pair_divide_conquer, find_intersecting_segments};
use crate::matrix::{Matrix, standard_multiply, strassen_multiply, cache_optimized_multiply, parallel_multiply, simd_multiply, winograd_multiply, parallel_winograd_multiply, determinant, inverse, solve_linear_system, lu_decomposition, qr_decomposition, cholesky_decomposition, matrix_rank, condition_number_approx};

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

    /// Comprehensive geometry algorithms benchmarking
    pub fn run_geometry_benchmarks(&mut self, runs: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "🔺 Running Geometry Algorithm Benchmarks".bright_yellow().bold());
        
        let data_sizes = vec![1000, 5000, 10000, 25000, 50000];
        
        for &size in &data_sizes {
            println!("\n{} Data size: {}", "📊".bright_blue(), size.to_string().bright_white().bold());
            
            // Generate test data
            let points = DataGenerator::generate_random_points(size);
            let segments = DataGenerator::generate_random_line_segments(size.min(1000)); // Limit segments for performance
            
            // KdTree vs Brute Force Nearest Neighbor Search
            self.benchmark_kdtree_vs_bruteforce(&points, runs)?;
            
            // Closest Pair Problem: Divide & Conquer vs Brute Force
            self.benchmark_closest_pair_algorithms(&points, runs)?;
            
            // Convex Hull Algorithm
            self.benchmark_convex_hull(&points, runs)?;
            
            // Line Segment Intersection (smaller dataset)
            if segments.len() > 0 {
                self.benchmark_line_segment_intersection(&segments, runs)?;
            }
        }
        
        Ok(())
    }

    fn benchmark_kdtree_vs_bruteforce(&mut self, points: &[Point], runs: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("  {} KdTree vs Brute Force Nearest Neighbor", "🔍".bright_green());
        
        if points.is_empty() {
            return Ok(());
        }
        
        // Build KdTree once
        let tree = KdTree::build(points);
        let query_point = points[0]; // Use first point as query
        
        // Benchmark KdTree search
        let mut kdtree_times = Vec::new();
        let mut memory_usage = None;
        
        for _ in 0..runs {
            let start_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            let start = Instant::now();
            
            let _result = tree.nearest_neighbor(&query_point);
            
            let duration = start.elapsed();
            kdtree_times.push(duration.as_secs_f64() * 1000.0);
            
            let end_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            if let (Some(start_mem), Some(end_mem)) = (start_memory, end_memory) {
                memory_usage = Some(end_mem - start_mem);
            }
        }
        
        // Benchmark brute force search (O(n))
        let mut bruteforce_times = Vec::new();
        
        for _ in 0..runs {
            let start = Instant::now();
            
            let mut min_distance = f64::INFINITY;
            let mut _nearest = points[0];
            for &point in points {
                let distance = query_point.distance_to(&point);
                if distance < min_distance && distance > 0.0 {
                    min_distance = distance;
                    _nearest = point;
                }
            }
            
            let duration = start.elapsed();
            bruteforce_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // Calculate statistics and store results
        self.store_geometry_benchmark_result("KdTree Nearest Neighbor", points.len(), runs, &kdtree_times, memory_usage, false)?;
        self.store_geometry_benchmark_result("Brute Force Nearest Neighbor", points.len(), runs, &bruteforce_times, None, false)?;
        
        let kdtree_mean = mean(&kdtree_times);
        let bruteforce_mean = mean(&bruteforce_times);
        let speedup = bruteforce_mean / kdtree_mean;
        
        println!("    KdTree: {:.3}ms, Brute Force: {:.3}ms, Speedup: {:.1}x", 
                kdtree_mean, bruteforce_mean, speedup);
        
        Ok(())
    }

    fn benchmark_closest_pair_algorithms(&mut self, points: &[Point], runs: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("  {} Closest Pair: Divide & Conquer vs Brute Force", "📏".bright_green());
        
        if points.len() < 2 {
            return Ok(());
        }
        
        // Benchmark divide and conquer approach
        let mut dc_times = Vec::new();
        let mut memory_usage = None;
        
        for _ in 0..runs {
            let start_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            let start = Instant::now();
            
            let _result = closest_pair_divide_conquer(points);
            
            let duration = start.elapsed();
            dc_times.push(duration.as_secs_f64() * 1000.0);
            
            let end_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            if let (Some(start_mem), Some(end_mem)) = (start_memory, end_memory) {
                memory_usage = Some(end_mem - start_mem);
            }
        }
        
        // Benchmark brute force approach (only for smaller datasets to avoid timeout)
        let mut bf_times = Vec::new();
        
        if points.len() <= 10000 {
            for _ in 0..runs {
                let start = Instant::now();
                
                let _result = closest_pair_brute_force(points);
                
                let duration = start.elapsed();
                bf_times.push(duration.as_secs_f64() * 1000.0);
            }
            
            self.store_geometry_benchmark_result("Closest Pair Brute Force", points.len(), runs, &bf_times, None, false)?;
            
            let dc_mean = mean(&dc_times);
            let bf_mean = mean(&bf_times);
            let speedup = bf_mean / dc_mean;
            
            println!("    Divide & Conquer: {:.3}ms, Brute Force: {:.3}ms, Speedup: {:.1}x", 
                    dc_mean, bf_mean, speedup);
        } else {
            let dc_mean = mean(&dc_times);
            println!("    Divide & Conquer: {:.3}ms (Brute Force skipped for large dataset)", dc_mean);
        }
        
        self.store_geometry_benchmark_result("Closest Pair Divide & Conquer", points.len(), runs, &dc_times, memory_usage, false)?;
        
        Ok(())
    }

    fn benchmark_convex_hull(&mut self, points: &[Point], runs: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("  {} Convex Hull (Graham Scan)", "📐".bright_green());
        
        if points.len() < 3 {
            return Ok(());
        }
        
        let mut hull_times = Vec::new();
        let mut memory_usage = None;
        
        for _ in 0..runs {
            let start_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            let start = Instant::now();
            
            let _hull = convex_hull_graham_scan(points);
            
            let duration = start.elapsed();
            hull_times.push(duration.as_secs_f64() * 1000.0);
            
            let end_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            if let (Some(start_mem), Some(end_mem)) = (start_memory, end_memory) {
                memory_usage = Some(end_mem - start_mem);
            }
        }
        
        self.store_geometry_benchmark_result("Convex Hull Graham Scan", points.len(), runs, &hull_times, memory_usage, false)?;
        
        let hull_mean = mean(&hull_times);
        println!("    Convex Hull: {:.3}ms", hull_mean);
        
        Ok(())
    }

    fn benchmark_line_segment_intersection(&mut self, segments: &[LineSegment], runs: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("  {} Line Segment Intersection Detection", "📏".bright_green());
        
        if segments.len() < 2 {
            return Ok(());
        }
        
        let mut intersection_times = Vec::new();
        let mut memory_usage = None;
        
        for _ in 0..runs {
            let start_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            let start = Instant::now();
            
            let _intersections = find_intersecting_segments(segments);
            
            let duration = start.elapsed();
            intersection_times.push(duration.as_secs_f64() * 1000.0);
            
            let end_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            if let (Some(start_mem), Some(end_mem)) = (start_memory, end_memory) {
                memory_usage = Some(end_mem - start_mem);
            }
        }
        
        self.store_geometry_benchmark_result("Line Segment Intersection", segments.len(), runs, &intersection_times, memory_usage, false)?;
        
        let intersection_mean = mean(&intersection_times);
        println!("    Intersection Detection: {:.3}ms", intersection_mean);
        
        Ok(())
    }

    fn store_geometry_benchmark_result(
        &mut self,
        algorithm_name: &str,
        data_size: usize,
        runs: usize,
        times: &[f64],
        memory_usage: Option<f64>,
        parallel: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mean_time = mean(times);
        let std_dev = standard_deviation(times, Some(mean_time));
        let mut times_sorted = times.to_vec();
        times_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let result = DetailedBenchmarkResult {
            algorithm_name: algorithm_name.to_string(),
            data_size,
            runs,
            mean_time_ms: mean_time,
            std_dev_time_ms: std_dev,
            min_time_ms: times_sorted[0],
            max_time_ms: times_sorted[times_sorted.len() - 1],
            median_time_ms: times_sorted[times_sorted.len() / 2],
            mean_memory_mb: memory_usage,
            parallel,
            speedup_vs_sequential: None,
            efficiency: None,
            individual_runs_ms: times.to_vec(),
        };
        
        self.results.push(result);
        Ok(())
    }

    /// Comprehensive matrix algorithms benchmarking
    pub fn run_matrix_benchmarks(&mut self, runs: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n{}", "🔢 Running Matrix Algorithm Benchmarks".bright_yellow().bold());
        
        let matrix_sizes = vec![64, 128, 256, 512]; // Reasonable sizes for comprehensive testing
        
        for &size in &matrix_sizes {
            println!("\n{} Matrix size: {}x{}", "📊".bright_blue(), size.to_string().bright_white().bold(), size.to_string().bright_white().bold());
            
            // Generate test matrices
            let (matrix_a, matrix_b) = DataGenerator::generate_random_matrices(size);
            let identity_matrix = DataGenerator::generate_identity_matrix(size);
            let sparse_matrix = DataGenerator::generate_sparse_matrix(size, 0.1);
            
            // Matrix multiplication algorithms comparison
            self.benchmark_matrix_multiplication(&matrix_a, &matrix_b, runs)?;
            
            // Advanced linear algebra operations
            self.benchmark_matrix_operations(&matrix_a, &identity_matrix, &sparse_matrix, runs)?;
            
            // Linear system solving
            self.benchmark_linear_system_solving(&matrix_a, &matrix_b, runs)?;
            
            // Advanced matrix analysis operations
            self.benchmark_advanced_matrix_operations(&matrix_a, runs)?;
        }
        
        Ok(())
    }

    fn benchmark_matrix_multiplication(&mut self, matrix_a: &Matrix, matrix_b: &Matrix, runs: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("  {} Matrix Multiplication Algorithms", "✖️".bright_green());
        
        let size = matrix_a.size();
        
        // Standard multiplication
        let mut standard_times = Vec::new();
        let mut memory_usage = None;
        
        for _ in 0..runs {
            let start_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            let start = Instant::now();
            
            let _result = standard_multiply(matrix_a, matrix_b)?;
            
            let duration = start.elapsed();
            standard_times.push(duration.as_secs_f64() * 1000.0);
            
            let end_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            if let (Some(start_mem), Some(end_mem)) = (start_memory, end_memory) {
                memory_usage = Some(end_mem - start_mem);
            }
        }
        
        // Cache-optimized multiplication
        let mut cache_opt_times = Vec::new();
        let block_size = 64; // Optimal block size for cache
        
        for _ in 0..runs {
            let start = Instant::now();
            
            let _result = cache_optimized_multiply(matrix_a, matrix_b, block_size)?;
            
            let duration = start.elapsed();
            cache_opt_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // Parallel multiplication
        let mut parallel_times = Vec::new();
        
        for _ in 0..runs {
            let start = Instant::now();
            
            let _result = parallel_multiply(matrix_a, matrix_b)?;
            
            let duration = start.elapsed();
            parallel_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // Strassen multiplication (for comparison)
        let mut strassen_times = Vec::new();
        
        for _ in 0..runs {
            let start = Instant::now();
            
            let _result = strassen_multiply(matrix_a, matrix_b)?;
            
            let duration = start.elapsed();
            strassen_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // SIMD multiplication (x86_64 with AVX2 support)
        let mut simd_times = Vec::new();
        
        for _ in 0..runs {
            let start = Instant::now();
            
            let _result = simd_multiply(matrix_a, matrix_b)?;
            
            let duration = start.elapsed();
            simd_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // Winograd multiplication
        let mut winograd_times = Vec::new();
        
        for _ in 0..runs {
            let start = Instant::now();
            
            let _result = winograd_multiply(matrix_a, matrix_b)?;
            
            let duration = start.elapsed();
            winograd_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // Parallel Winograd multiplication
        let mut parallel_winograd_times = Vec::new();
        
        for _ in 0..runs {
            let start = Instant::now();
            
            let _result = parallel_winograd_multiply(matrix_a, matrix_b)?;
            
            let duration = start.elapsed();
            parallel_winograd_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // Store results
        self.store_geometry_benchmark_result("Standard Matrix Multiply", size * size, runs, &standard_times, memory_usage, false)?;
        self.store_geometry_benchmark_result("Cache-Optimized Multiply", size * size, runs, &cache_opt_times, None, false)?;
        self.store_geometry_benchmark_result("Parallel Matrix Multiply", size * size, runs, &parallel_times, None, true)?;
        self.store_geometry_benchmark_result("Strassen Matrix Multiply", size * size, runs, &strassen_times, None, false)?;
        self.store_geometry_benchmark_result("SIMD Matrix Multiply", size * size, runs, &simd_times, None, false)?;
        self.store_geometry_benchmark_result("Winograd Matrix Multiply", size * size, runs, &winograd_times, None, false)?;
        self.store_geometry_benchmark_result("Parallel Winograd Matrix Multiply", size * size, runs, &parallel_winograd_times, None, true)?;
        
        let standard_mean = mean(&standard_times);
        let cache_opt_mean = mean(&cache_opt_times);
        let parallel_mean = mean(&parallel_times);
        let strassen_mean = mean(&strassen_times);
        let simd_mean = mean(&simd_times);
        let winograd_mean = mean(&winograd_times);
        let parallel_winograd_mean = mean(&parallel_winograd_times);
        
        let cache_speedup = standard_mean / cache_opt_mean;
        let parallel_speedup = standard_mean / parallel_mean;
        let strassen_speedup = standard_mean / strassen_mean;
        let simd_speedup = standard_mean / simd_mean;
        let winograd_speedup = standard_mean / winograd_mean;
        let parallel_winograd_speedup = standard_mean / parallel_winograd_mean;
        
        println!("    Standard: {:.3}ms", standard_mean);
        println!("    Cache-Optimized: {:.3}ms (Speedup: {:.1}x)", cache_opt_mean, cache_speedup);
        println!("    Parallel: {:.3}ms (Speedup: {:.1}x)", parallel_mean, parallel_speedup);
        println!("    Strassen: {:.3}ms (Speedup: {:.1}x)", strassen_mean, strassen_speedup);
        println!("    SIMD: {:.3}ms (Speedup: {:.1}x)", simd_mean, simd_speedup);
        println!("    Winograd: {:.3}ms (Speedup: {:.1}x)", winograd_mean, winograd_speedup);
        println!("    Parallel Winograd: {:.3}ms (Speedup: {:.1}x)", parallel_winograd_mean, parallel_winograd_speedup);
        
        Ok(())
    }

    fn benchmark_matrix_operations(&mut self, matrix_a: &Matrix, identity_matrix: &Matrix, sparse_matrix: &Matrix, runs: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("  {} Advanced Matrix Operations", "🧮".bright_green());
        
        let size = matrix_a.size();
        
        // Matrix addition
        let mut addition_times = Vec::new();
        for _ in 0..runs {
            let start = Instant::now();
            let _result = matrix_a.add(identity_matrix)?;
            let duration = start.elapsed();
            addition_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // Matrix subtraction
        let mut subtraction_times = Vec::new();
        for _ in 0..runs {
            let start = Instant::now();
            let _result = matrix_a.subtract(identity_matrix)?;
            let duration = start.elapsed();
            subtraction_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // Determinant calculation
        let mut determinant_times = Vec::new();
        for _ in 0..runs {
            let start = Instant::now();
            let _det = determinant(matrix_a)?;
            let duration = start.elapsed();
            determinant_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // Matrix inversion (only for well-conditioned matrices)
        let mut inversion_times = Vec::new();
        for _ in 0..runs {
            let start = Instant::now();
            // Use identity matrix for inversion as it's guaranteed to be invertible
            let _inv = inverse(identity_matrix);
            let duration = start.elapsed();
            inversion_times.push(duration.as_secs_f64() * 1000.0);
        }
        
        // Store results
        self.store_geometry_benchmark_result("Matrix Addition", size * size, runs, &addition_times, None, false)?;
        self.store_geometry_benchmark_result("Matrix Subtraction", size * size, runs, &subtraction_times, None, false)?;
        self.store_geometry_benchmark_result("Matrix Determinant", size * size, runs, &determinant_times, None, false)?;
        self.store_geometry_benchmark_result("Matrix Inversion", size * size, runs, &inversion_times, None, false)?;
        
        let addition_mean = mean(&addition_times);
        let subtraction_mean = mean(&subtraction_times);
        let determinant_mean = mean(&determinant_times);
        let inversion_mean = mean(&inversion_times);
        
        println!("    Addition: {:.3}ms", addition_mean);
        println!("    Subtraction: {:.3}ms", subtraction_mean);
        println!("    Determinant: {:.3}ms", determinant_mean);
        println!("    Inversion: {:.3}ms", inversion_mean);
        
        Ok(())
    }

    fn benchmark_linear_system_solving(&mut self, matrix_a: &Matrix, matrix_b: &Matrix, runs: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("  {} Linear System Solving", "⚖️".bright_green());
        
        let size = matrix_a.size();
        
        // Create a well-conditioned system for solving
        let identity = DataGenerator::generate_identity_matrix(size);
        let diagonal = DataGenerator::generate_diagonal_matrix(size);
        
        // Solve Ax = b with identity matrix (guaranteed solution)
        let mut solve_times = Vec::new();
        let mut memory_usage = None;
        
        for _ in 0..runs {
            let start_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            let start = Instant::now();
            
            let _solution = solve_linear_system(&diagonal, &identity);
            
            let duration = start.elapsed();
            solve_times.push(duration.as_secs_f64() * 1000.0);
            
            let end_memory = memory_stats().map(|stats| stats.physical_mem as f64 / 1024.0 / 1024.0);
            if let (Some(start_mem), Some(end_mem)) = (start_memory, end_memory) {
                memory_usage = Some(end_mem - start_mem);
            }
        }
        
        self.store_geometry_benchmark_result("Linear System Solving", size * size, runs, &solve_times, memory_usage, false)?;
        
        let solve_mean = mean(&solve_times);
        println!("    Linear System Solving: {:.3}ms", solve_mean);
        
        Ok(())
    }

    fn benchmark_advanced_matrix_operations(&mut self, matrix: &Matrix, runs: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("  {} Advanced Matrix Analysis", "🔬".bright_green());
        
        let size = matrix.size();
        
        // Matrix norms benchmarking
        let mut frobenius_times = Vec::new();
        let mut one_norm_times = Vec::new();
        let mut infinity_norm_times = Vec::new();
        let mut two_norm_times = Vec::new();
        
        for _ in 0..runs {
            // Frobenius norm
            let start = Instant::now();
            let _norm = matrix.frobenius_norm();
            frobenius_times.push(start.elapsed().as_secs_f64() * 1000.0);
            
            // 1-norm
            let start = Instant::now();
            let _norm = matrix.one_norm();
            one_norm_times.push(start.elapsed().as_secs_f64() * 1000.0);
            
            // Infinity norm
            let start = Instant::now();
            let _norm = matrix.infinity_norm();
            infinity_norm_times.push(start.elapsed().as_secs_f64() * 1000.0);
            
            // 2-norm approximation (fewer iterations for speed)
            let start = Instant::now();
            let _norm = matrix.two_norm_approx(10);
            two_norm_times.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        
        // Matrix decomposition benchmarking
        let mut lu_times = Vec::new();
        let mut qr_times = Vec::new();
        let mut cholesky_times = Vec::new();
        
        for _ in 0..runs {
            // LU decomposition
            let start = Instant::now();
            let _result = lu_decomposition(matrix);
            lu_times.push(start.elapsed().as_secs_f64() * 1000.0);
            
            // QR decomposition
            let start = Instant::now();
            let _result = qr_decomposition(matrix);
            qr_times.push(start.elapsed().as_secs_f64() * 1000.0);
            
            // Cholesky decomposition (only for positive definite matrices)
            // Create a positive definite matrix for testing
            let at = matrix.transpose();
            match standard_multiply(&at, matrix) {
                Ok(positive_def) => {
                    let start = Instant::now();
                    let _result = cholesky_decomposition(&positive_def);
                    cholesky_times.push(start.elapsed().as_secs_f64() * 1000.0);
                }
                Err(_) => cholesky_times.push(0.0),
            }
        }
        
        // Matrix analysis benchmarking
        let mut rank_times = Vec::new();
        let mut condition_times = Vec::new();
        let mut trace_times = Vec::new();
        let mut transpose_times = Vec::new();
        
        for _ in 0..runs {
            // Matrix rank
            let start = Instant::now();
            let _rank = matrix_rank(matrix);
            rank_times.push(start.elapsed().as_secs_f64() * 1000.0);
            
            // Condition number
            let start = Instant::now();
            let _cond = condition_number_approx(matrix, 5);
            condition_times.push(start.elapsed().as_secs_f64() * 1000.0);
            
            // Trace
            let start = Instant::now();
            let _trace = matrix.trace();
            trace_times.push(start.elapsed().as_secs_f64() * 1000.0);
            
            // Transpose
            let start = Instant::now();
            let _transpose = matrix.transpose();
            transpose_times.push(start.elapsed().as_secs_f64() * 1000.0);
        }
        
        // Store all results
        self.store_geometry_benchmark_result("Matrix Frobenius Norm", size * size, runs, &frobenius_times, None, false)?;
        self.store_geometry_benchmark_result("Matrix 1-Norm", size * size, runs, &one_norm_times, None, false)?;
        self.store_geometry_benchmark_result("Matrix Infinity-Norm", size * size, runs, &infinity_norm_times, None, false)?;
        self.store_geometry_benchmark_result("Matrix 2-Norm Approx", size * size, runs, &two_norm_times, None, false)?;
        
        self.store_geometry_benchmark_result("LU Decomposition", size * size, runs, &lu_times, None, false)?;
        self.store_geometry_benchmark_result("QR Decomposition", size * size, runs, &qr_times, None, false)?;
        self.store_geometry_benchmark_result("Cholesky Decomposition", size * size, runs, &cholesky_times, None, false)?;
        
        self.store_geometry_benchmark_result("Matrix Rank", size * size, runs, &rank_times, None, false)?;
        self.store_geometry_benchmark_result("Condition Number", size * size, runs, &condition_times, None, false)?;
        self.store_geometry_benchmark_result("Matrix Trace", size * size, runs, &trace_times, None, false)?;
        self.store_geometry_benchmark_result("Matrix Transpose", size * size, runs, &transpose_times, None, false)?;
        
        // Display results
        println!("    Frobenius Norm: {:.3}ms", mean(&frobenius_times));
        println!("    1-Norm: {:.3}ms", mean(&one_norm_times));
        println!("    Infinity-Norm: {:.3}ms", mean(&infinity_norm_times));
        println!("    2-Norm (approx): {:.3}ms", mean(&two_norm_times));
        println!("    LU Decomposition: {:.3}ms", mean(&lu_times));
        println!("    QR Decomposition: {:.3}ms", mean(&qr_times));
        println!("    Cholesky Decomposition: {:.3}ms", mean(&cholesky_times));
        println!("    Matrix Rank: {:.3}ms", mean(&rank_times));
        println!("    Condition Number: {:.3}ms", mean(&condition_times));
        println!("    Matrix Trace: {:.3}ms", mean(&trace_times));
        println!("    Matrix Transpose: {:.3}ms", mean(&transpose_times));
        
        Ok(())
    }
}