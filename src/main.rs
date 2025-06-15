use clap::{Parser, Subcommand};
use colored::*;

// Module declarations
mod benchmark;
mod data_generator;
mod geometry;
mod matrix;
mod sorting;
mod visualization;

use benchmark::BenchmarkRunner;
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
        Commands::Visualize { input, output } => {
            println!("{}", "Generating visualization...".green());
            run_visualization(input, output);
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

fn run_visualization(input: &str, output: &str) {
    match visualization::generate_performance_charts(input, output) {
        Ok(_) => println!("{}", format!("Visualization saved to {}", output).green()),
        Err(e) => println!("{}", format!("Error generating visualization: {}", e).red()),
    }
}
