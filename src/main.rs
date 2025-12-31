use clap::{Parser, Subcommand};
use colored::*;

// Module declarations
mod benchmark;
mod comprehensive_benchmark;
mod advanced_benchmark;
mod cross_platform_validation;
mod data_generator;
mod geometry;
mod matrix;
mod sorting;
mod visualization;

use benchmark::BenchmarkRunner;
use comprehensive_benchmark::ComprehensiveBenchmarkRunner;
use advanced_benchmark::AdvancedBenchmarkRunner;
use cross_platform_validation::CrossPlatformValidator;
use data_generator::DataGenerator;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run sorting algorithms benchmark
    Sort {
        /// Data size
        #[arg(short, long, default_value_t = 10000)]
        size: usize,
        /// Number of runs
        #[arg(short, long, default_value_t = 5)]
        runs: usize,
        /// Enable parallel processing
        #[arg(short, long)]
        parallel: bool,
    },
    /// Run matrix multiplication benchmark
    Matrix {
        /// Matrix size (N x N)
        #[arg(short, long, default_value_t = 512)]
        size: usize,
        /// Use Strassen algorithm
        #[arg(short = 't', long)]
        strassen: bool,
    },
    /// Run closest pair problem benchmark
    Geometry {
        /// Number of points
        #[arg(short, long, default_value_t = 10000)]
        points: usize,
    },
    /// Comprehensive benchmark of all algorithms
    All {
        /// Use small dataset sizes
        #[arg(short, long)]
        small: bool,
    },
    /// Comprehensive benchmark with detailed analysis and publication-ready data
    Publication {
        /// Number of runs per test
        #[arg(short, long, default_value_t = 10)]
        runs: usize,
        /// Include extended scalability analysis
        #[arg(short, long)]
        extended: bool,
    },
    /// Advanced benchmarking with cache, energy, and NUMA analysis
    Advanced {
        /// Number of runs per test
        #[arg(short, long, default_value_t = 5)]
        runs: usize,
        /// Data sizes to test
        #[arg(short, long, default_values_t = vec![1000, 5000, 10000])]
        sizes: Vec<usize>,
    },
    /// Cross-platform validation with statistical analysis
    Validate {
        /// Number of runs per test
        #[arg(short, long, default_value_t = 5)]
        runs: usize,
        /// Include compiler optimization analysis
        #[arg(short, long)]
        optimization: bool,
        /// Include extended thread scaling analysis
        #[arg(short, long)]
        threading: bool,
    },
    /// Generate visualization of results
    Visualize {
        /// Input results file path
        #[arg(short, long)]
        input: String,
        /// Output file path
        #[arg(short, long, default_value = "output.png")]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    println!("{}", "=== Large-Scale Data Processing Application ===".bright_blue().bold());
    
    match &cli.command {
        Commands::Sort { size, runs, parallel } => {
            println!("{}", "Running sorting algorithms benchmark...".green());
            run_sort_benchmark(*size, *runs, *parallel);
        }
        Commands::Matrix { size, strassen } => {
            println!("{}", "Running matrix multiplication benchmark...".green());
            run_matrix_benchmark(*size, *strassen);
        }
        Commands::Geometry { points } => {
            println!("{}", "Running closest pair problem benchmark...".green());
            run_geometry_benchmark(*points);
        }
        Commands::All { small } => {
            println!("{}", "Running comprehensive benchmark...".green());
            run_comprehensive_benchmark(*small);
        }
        Commands::Publication { runs, extended } => {
            println!("{}", "Running publication-quality benchmark...".green());
            run_publication_benchmark(*runs, *extended);
        }
        Commands::Visualize { input, output } => {
            println!("{}", "Generating visualization...".green());
            run_visualization(input, output);
        }
        Commands::Advanced { runs, sizes } => {
            println!("{}", "Running advanced benchmarking analysis...".green());
            run_advanced_benchmark(*runs, sizes);
        }
        Commands::Validate { runs, optimization, threading } => {
            println!("{}", "Running cross-platform validation...".green());
            run_cross_platform_validation(*runs, *optimization, *threading);
        }
    }
}

fn run_sort_benchmark(size: usize, runs: usize, parallel: bool) {
    let mut runner = BenchmarkRunner::new();
    let data = DataGenerator::generate_random_integers(size);
    
    println!("{}", format!("Data size: {}, Number of runs: {}", size, runs).yellow());
    
    if parallel {
        println!("{}", "Running in parallel mode".cyan());
    }
    
    // Benchmark merge sort
    runner.benchmark_sort("Merge Sort", &data, runs, parallel);
    
    // Benchmark quick sort
    runner.benchmark_sort("Quick Sort", &data, runs, parallel);
    
    // Display results
    runner.display_results();
}

fn run_matrix_benchmark(size: usize, strassen: bool) {
    let mut runner = BenchmarkRunner::new();
    let (matrix_a, matrix_b) = DataGenerator::generate_random_matrices(size);
    
    println!("{}", format!("Matrix size: {}x{}", size, size).yellow());
    
    if strassen {
        println!("{}", "Using Strassen algorithm".cyan());
    }
    
    runner.benchmark_matrix_multiply("Matrix Multiplication", &matrix_a, &matrix_b, strassen);
    runner.display_results();
}

fn run_geometry_benchmark(points: usize) {
    let mut runner = BenchmarkRunner::new();
    let point_set = DataGenerator::generate_random_points(points);
    
    println!("{}", format!("Number of points: {}", points).yellow());
    
    runner.benchmark_closest_pair("Closest Pair", &point_set);
    runner.display_results();
}

fn run_comprehensive_benchmark(small: bool) {
    println!("{}", "=== Comprehensive Benchmark ===".bright_magenta().bold());
    
    let sizes = if small {
        vec![100, 500, 1000, 5000]
    } else {
        vec![1000, 5000, 10000, 50000, 100000]
    };
    
    for &size in &sizes {
        println!("{}", format!("\n--- Data size: {} ---", size).bright_yellow());
        
        // Sorting algorithms
        run_sort_benchmark(size, 3, false);
        run_sort_benchmark(size, 3, true);
        
        // Matrix multiplication (adjust size)
        let matrix_size = (size as f64).sqrt() as usize;
        if matrix_size >= 4 {
            run_matrix_benchmark(matrix_size, false);
            run_matrix_benchmark(matrix_size, true);
        }
        
        // Closest pair problem
        run_geometry_benchmark(size);
    }
}

fn run_publication_benchmark(runs: usize, extended: bool) {
    println!("{}", "=== Publication-Quality Comprehensive Benchmark ===".bright_magenta().bold());
    
    let mut runner = ComprehensiveBenchmarkRunner::new();
    
    // Standard data sizes for comprehensive analysis
    let standard_sizes = vec![1000, 5000, 10000, 25000, 50000];
    let extended_sizes = vec![100000, 250000, 500000, 1000000];
    
    // 1. Comprehensive sorting benchmarks with std library comparisons
    let benchmark_sizes = if extended {
        [&standard_sizes[..], &extended_sizes[..]].concat()
    } else {
        standard_sizes.clone()
    };
    
    runner.benchmark_sorting_comprehensive(&benchmark_sizes, runs);
    
    // 2. Calculate speedups
    runner.calculate_speedups();
    
    // 3. Scalability analysis
    println!("\n{}", "=== Scalability Analysis ===".bright_green().bold());
    runner.analyze_scalability("Merge Sort", &standard_sizes, false);
    runner.analyze_scalability("Merge Sort", &standard_sizes, true);
    runner.analyze_scalability("Quick Sort", &standard_sizes, false);
    runner.analyze_scalability("Quick Sort", &standard_sizes, true);
    
    // 4. Parallel efficiency analysis
    println!("\n{}", "=== Parallel Efficiency Analysis ===".bright_green().bold());
    let thread_counts = vec![1, 2, 4, 8, 14, 20]; // Based on system specs
    runner.analyze_parallel_efficiency("Merge Sort", 50000, &thread_counts);
    runner.analyze_parallel_efficiency("Quick Sort", 50000, &thread_counts);
    
    // 5. Geometry algorithms comprehensive benchmarks
    match runner.run_geometry_benchmarks(runs) {
        Ok(_) => println!("\n{}", "✓ Geometry algorithms benchmarked successfully!".bright_green().bold()),
        Err(e) => println!("{}", format!("Error running geometry benchmarks: {}", e).red()),
    }
    
    // 6. Matrix algorithms comprehensive benchmarks
    match runner.run_matrix_benchmarks(runs) {
        Ok(_) => println!("\n{}", "✓ Matrix algorithms benchmarked successfully!".bright_green().bold()),
        Err(e) => println!("{}", format!("Error running matrix benchmarks: {}", e).red()),
    }
    
    // 7. Save comprehensive results
    match runner.save_comprehensive_results("publication_benchmark") {
        Ok(_) => {
            println!("\n{}", "✓ Publication-quality benchmark data generated successfully!".bright_green().bold());
            println!("Generated files:");
            println!("  • publication_benchmark_full_report.json - Complete structured data");
            println!("  • publication_benchmark_detailed_results.csv - Detailed performance metrics");
            println!("  • publication_benchmark_scalability.csv - Scalability analysis data");
            println!("  • publication_benchmark_parallel_efficiency.csv - Parallel efficiency analysis");
        }
        Err(e) => println!("{}", format!("Error saving results: {}", e).red()),
    }
}

fn run_visualization(input: &str, output: &str) {
    match visualization::generate_performance_charts(input, output) {
        Ok(_) => println!("{}", format!("Visualization saved to {}", output).green()),
        Err(e) => println!("{}", format!("Error generating visualization: {}", e).red()),
    }
}

fn run_advanced_benchmark(_runs: usize, sizes: &[usize]) {
    println!("{}", "=== Advanced Benchmarking Analysis ===".bright_magenta().bold());
    println!("{}", "Cache Performance • Energy Consumption • NUMA Effects • Algorithmic Constants".cyan());
    
    let mut runner = AdvancedBenchmarkRunner::new();
    
    // Test different sorting algorithms with advanced metrics
    for algorithm in &["merge_sort", "quick_sort"] {
        println!("\n{}", format!("=== Advanced Analysis: {} ===", algorithm).bright_green().bold());
        
        match algorithm {
            &"merge_sort" => {
                let benchmark_fn = |size: usize| -> f64 {
                    let mut data = DataGenerator::generate_random_integers(size);
                    let start = std::time::Instant::now();
                    crate::sorting::merge_sort(&mut data);
                    start.elapsed().as_secs_f64() * 1000.0
                };
                
                match runner.extended_scalability_analysis("Merge Sort", benchmark_fn, sizes) {
                    Ok(_) => println!("Advanced merge sort analysis completed"),
                    Err(e) => println!("Error in merge sort analysis: {}", e),
                }
            }
            &"quick_sort" => {
                let benchmark_fn = |size: usize| -> f64 {
                    let mut data = DataGenerator::generate_random_integers(size);
                    let start = std::time::Instant::now();
                    crate::sorting::quick_sort(&mut data);
                    start.elapsed().as_secs_f64() * 1000.0
                };
                
                match runner.extended_scalability_analysis("Quick Sort", benchmark_fn, sizes) {
                    Ok(_) => println!("Advanced quick sort analysis completed"),
                    Err(e) => println!("Error in quick sort analysis: {}", e),
                }
            }
            _ => {}
        }
    }
    
    // Test matrix multiplication with advanced metrics
    println!("\n{}", "=== Advanced Analysis: Matrix Multiplication ===".bright_green().bold());
    
    let matrix_benchmark_fn = |size: usize| -> f64 {
        let matrix_size = (size as f64).sqrt() as usize + 1;
        let (matrix_a, matrix_b) = DataGenerator::generate_random_matrices(matrix_size);
        let start = std::time::Instant::now();
        let _ = crate::matrix::standard_multiply(&matrix_a, &matrix_b);
        start.elapsed().as_secs_f64() * 1000.0
    };
    
    let matrix_sizes: Vec<usize> = sizes.iter().map(|&s| (s as f64).sqrt() as usize + 1).collect();
    match runner.extended_scalability_analysis("Matrix Multiplication", matrix_benchmark_fn, &matrix_sizes) {
        Ok(_) => println!("Advanced matrix multiplication analysis completed"),
        Err(e) => println!("Error in matrix analysis: {}", e),
    }
    
    // Save advanced benchmark results
    match runner.save_advanced_results("advanced_benchmark") {
        Ok(_) => {
            println!("\n{}", "Advanced benchmark analysis completed successfully!".bright_green().bold());
            println!("Generated files:");
            println!("  • advanced_benchmark_advanced_benchmark.json - Complete advanced metrics");
            println!("  • advanced_benchmark_advanced_metrics.csv - Cache, energy, NUMA data");
            println!("\n{}", "Analysis includes:".bright_yellow());
            println!("  Cache performance (L1/L2/L3 miss rates, memory bandwidth)");
            println!("  Energy consumption (power usage, energy efficiency)");
            println!("  NUMA effects (memory locality, cross-node bandwidth)");
            println!("  Algorithmic constants (empirical analysis, hidden factors)");
        }
        Err(e) => println!("{}", format!("Error saving advanced results: {}", e).red()),
    }
}

fn run_cross_platform_validation(runs: usize, include_optimization: bool, include_threading: bool) {
    println!("{}", "=== Week 3: Cross-Platform Validation ===".bright_magenta().bold());
    println!("{}", "Data Distributions • Compiler Optimization • Thread Scaling • Statistical Analysis".cyan());
    
    let mut validator = CrossPlatformValidator::new();
    
    // 1. Data Distribution Validation
    println!("\n{} Starting data distribution validation...", "[PHASE 1]".bright_blue());
    match validator.validate_data_distributions(runs) {
        Ok(_) => println!("{} Data distribution validation completed", "[SUCCESS]".bright_green()),
        Err(e) => println!("{} Error in data distribution validation: {}", "[ERROR]".red(), e),
    }
    
    // 2. Compiler Optimization Analysis (optional)
    if include_optimization {
        println!("\n{} Starting compiler optimization analysis...", "[PHASE 2]".bright_blue());
        match validator.validate_compiler_optimizations(runs) {
            Ok(_) => println!("{} Compiler optimization analysis completed", "[SUCCESS]".bright_green()),
            Err(e) => println!("{} Error in optimization analysis: {}", "[ERROR]".red(), e),
        }
    }
    
    // 3. Extended Thread Scaling Analysis (optional)
    let thread_scaling_results = if include_threading {
        println!("\n{} Starting extended thread scaling analysis...", "[PHASE 3]".bright_blue());
        match validator.validate_thread_scaling(runs) {
            Ok(results) => {
                println!("{} Thread scaling analysis completed", "[SUCCESS]".bright_green());
                results
            }
            Err(e) => {
                println!("{} Error in thread scaling analysis: {}", "[ERROR]".red(), e);
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };
    
    // 4. Statistical Significance Testing
    println!("\n{} Performing statistical analysis...", "[PHASE 4]".bright_blue());
    let anova_results = match validator.perform_statistical_analysis() {
        Ok(results) => {
            println!("{} Statistical analysis completed", "[SUCCESS]".bright_green());
            Some(results)
        }
        Err(e) => {
            println!("{} Error in statistical analysis: {}", "[ERROR]".red(), e);
            None
        }
    };
    
    // 5. Generate Comprehensive Report
    println!("\n{} Generating validation report...", "[REPORT]".bright_yellow());
    let report = validator.generate_validation_report(thread_scaling_results, anova_results);
    
    // 6. Save Results
    match validator.save_validation_results(&report, "cross_platform_validation") {
        Ok(_) => {
            println!("\n{} Cross-platform validation completed successfully!", "[COMPLETED]".bright_green().bold());
            println!("Generated files:");
            println!("  • cross_platform_validation_validation_report.json - Complete validation data");
            println!("  • cross_platform_validation_validation_results.csv - Detailed test results");
            println!("  • cross_platform_validation_distribution_analysis.csv - Data distribution effects");
            if include_optimization {
                println!("  • cross_platform_validation_optimization_impact.csv - Compiler optimization impact");
            }
            if include_threading {
                println!("  • cross_platform_validation_thread_scaling.csv - Thread scaling analysis");
            }
            println!("\n{} Validation includes:", "[ANALYSIS]".bright_yellow());
            println!("  Data distribution testing (random, sorted, reverse, partial, duplicates)");
            if include_optimization {
                println!("  Compiler optimization impact analysis (O0, O1, O2, O3)");
            }
            if include_threading {
                println!("  Extended thread scaling beyond available cores");
            }
            println!("  Statistical significance testing with ANOVA");
            println!("  Publication-ready cross-platform validation data");
        }
        Err(e) => println!("{} Error saving validation results: {}", "[ERROR]".red(), e),
    }
}
