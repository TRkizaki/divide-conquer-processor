use colored::*;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use statistical::{mean, standard_deviation};

use crate::data_generator::DataGenerator;
use crate::matrix::{
    cache_optimized_multiply, parallel_multiply, parallel_winograd_multiply, simd_multiply,
    standard_multiply, strassen_multiply, winograd_multiply, Matrix,
};
use crate::sorting;

// ============================================================================
// Result Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryComparisonResult {
    pub category: String,
    pub algorithm: String,
    pub library: String,
    pub data_size: usize,
    pub runs: usize,
    pub mean_time_ms: f64,
    pub std_dev_ms: f64,
    pub min_time_ms: f64,
    pub max_time_ms: f64,
    pub median_time_ms: f64,
    pub speedup_vs_baseline: Option<f64>,
}

pub struct LibraryComparisonRunner {
    pub results: Vec<LibraryComparisonResult>,
}

impl LibraryComparisonRunner {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    // ========================================================================
    // Sorting Comparisons
    // ========================================================================

    /// Compare custom sorting implementations against std library and Rayon.
    pub fn compare_sorting(&mut self, sizes: &[usize], runs: usize) {
        println!(
            "{}",
            "=== Sorting Library Comparison ===".bright_magenta().bold()
        );
        println!(
            "{}",
            "Custom D&C vs std::sort vs Rayon par_sort".cyan()
        );

        for &size in sizes {
            println!(
                "\n{}",
                format!("--- n = {} ---", size).bright_yellow()
            );

            let data = DataGenerator::generate_random_integers(size);

            // Custom sequential merge sort
            self.benchmark_sort("Sorting", "Merge Sort (Sequential)", "Custom D&C", &data, runs, |d| {
                sorting::merge_sort(d);
            });

            // Custom parallel merge sort
            self.benchmark_sort("Sorting", "Merge Sort (Parallel)", "Custom D&C", &data, runs, |d| {
                sorting::parallel_merge_sort(d);
            });

            // Custom sequential quick sort
            self.benchmark_sort("Sorting", "Quick Sort (Sequential)", "Custom D&C", &data, runs, |d| {
                sorting::quick_sort(d);
            });

            // Custom parallel quick sort
            self.benchmark_sort("Sorting", "Quick Sort (Parallel)", "Custom D&C", &data, runs, |d| {
                sorting::parallel_quick_sort(d);
            });

            // std::sort (TimSort)
            self.benchmark_sort("Sorting", "Stable Sort", "std library", &data, runs, |d| {
                d.sort();
            });

            // std::sort_unstable (pattern-defeating quicksort)
            self.benchmark_sort("Sorting", "Unstable Sort", "std library", &data, runs, |d| {
                d.sort_unstable();
            });

            // Rayon parallel stable sort
            {
                use rayon::prelude::*;
                self.benchmark_sort("Sorting", "Parallel Stable Sort", "Rayon", &data, runs, |d| {
                    d.par_sort();
                });
            }

            // Rayon parallel unstable sort
            {
                use rayon::prelude::*;
                self.benchmark_sort(
                    "Sorting",
                    "Parallel Unstable Sort",
                    "Rayon",
                    &data,
                    runs,
                    |d| {
                        d.par_sort_unstable();
                    },
                );
            }
        }
    }

    fn benchmark_sort<F>(
        &mut self,
        category: &str,
        algorithm: &str,
        library: &str,
        data: &[i32],
        runs: usize,
        sort_fn: F,
    ) where
        F: Fn(&mut Vec<i32>),
    {
        let mut times = Vec::new();

        for _ in 0..runs {
            let mut test_data = data.to_vec();
            let start = Instant::now();
            sort_fn(&mut test_data);
            times.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        let result = Self::compute_result(category, algorithm, library, data.len(), runs, &times);
        println!(
            "  {:>30} [{:<12}]: {:.3}ms +/- {:.3}ms",
            algorithm, library, result.mean_time_ms, result.std_dev_ms
        );
        self.results.push(result);
    }

    // ========================================================================
    // Matrix Multiplication Comparisons
    // ========================================================================

    /// Compare custom matrix multiplication against ndarray.
    pub fn compare_matrix_multiply(&mut self, sizes: &[usize], runs: usize) {
        println!(
            "\n{}",
            "=== Matrix Multiplication Library Comparison ==="
                .bright_magenta()
                .bold()
        );
        println!(
            "{}",
            "Custom implementations vs ndarray".cyan()
        );

        for &size in sizes {
            println!(
                "\n{}",
                format!("--- {}x{} matrices ---", size, size).bright_yellow()
            );

            let (mat_a, mat_b) = DataGenerator::generate_random_matrices(size);

            // Custom standard O(n^3)
            self.benchmark_matrix("Matrix Multiply", "Standard O(n^3)", "Custom", size, runs, || {
                let _ = standard_multiply(&mat_a, &mat_b).unwrap();
            });

            // Custom Strassen
            self.benchmark_matrix("Matrix Multiply", "Strassen O(n^2.807)", "Custom", size, runs, || {
                let _ = strassen_multiply(&mat_a, &mat_b).unwrap();
            });

            // Custom cache-optimized (block_size = 32 for L1 cache fitting)
            self.benchmark_matrix("Matrix Multiply", "Cache-Optimized", "Custom", size, runs, || {
                let _ = cache_optimized_multiply(&mat_a, &mat_b, 32).unwrap();
            });

            // Custom SIMD
            self.benchmark_matrix("Matrix Multiply", "SIMD (AVX2)", "Custom", size, runs, || {
                let _ = simd_multiply(&mat_a, &mat_b).unwrap();
            });

            // Custom Winograd
            self.benchmark_matrix("Matrix Multiply", "Winograd", "Custom", size, runs, || {
                let _ = winograd_multiply(&mat_a, &mat_b).unwrap();
            });

            // Custom parallel
            self.benchmark_matrix("Matrix Multiply", "Parallel", "Custom", size, runs, || {
                let _ = parallel_multiply(&mat_a, &mat_b).unwrap();
            });

            // Custom parallel Winograd
            self.benchmark_matrix("Matrix Multiply", "Parallel Winograd", "Custom", size, runs, || {
                let _ = parallel_winograd_multiply(&mat_a, &mat_b).unwrap();
            });

            // ndarray matrix multiplication
            let nd_a = Self::matrix_to_ndarray(&mat_a);
            let nd_b = Self::matrix_to_ndarray(&mat_b);
            self.benchmark_matrix("Matrix Multiply", "dot()", "ndarray", size, runs, || {
                let _ = nd_a.dot(&nd_b);
            });
        }
    }

    fn benchmark_matrix<F>(
        &mut self,
        category: &str,
        algorithm: &str,
        library: &str,
        size: usize,
        runs: usize,
        bench_fn: F,
    ) where
        F: Fn(),
    {
        let mut times = Vec::new();

        // Warm-up run
        bench_fn();

        for _ in 0..runs {
            let start = Instant::now();
            bench_fn();
            times.push(start.elapsed().as_secs_f64() * 1000.0);
        }

        let result = Self::compute_result(category, algorithm, library, size, runs, &times);
        println!(
            "  {:>30} [{:<12}]: {:.3}ms +/- {:.3}ms",
            algorithm, library, result.mean_time_ms, result.std_dev_ms
        );
        self.results.push(result);
    }

    /// Convert our Matrix type to ndarray Array2 for comparison
    fn matrix_to_ndarray(matrix: &Matrix) -> Array2<f64> {
        let rows = matrix.rows();
        let cols = matrix.cols();
        let mut arr = Array2::<f64>::zeros((rows, cols));
        for i in 0..rows {
            for j in 0..cols {
                arr[[i, j]] = matrix.get(i, j);
            }
        }
        arr
    }

    // ========================================================================
    // Utilities
    // ========================================================================

    fn compute_result(
        category: &str,
        algorithm: &str,
        library: &str,
        data_size: usize,
        runs: usize,
        times: &[f64],
    ) -> LibraryComparisonResult {
        let mean_time = mean(times);
        let std_dev = if times.len() > 1 {
            standard_deviation(times, Some(mean_time))
        } else {
            0.0
        };
        let mut sorted = times.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        LibraryComparisonResult {
            category: category.to_string(),
            algorithm: algorithm.to_string(),
            library: library.to_string(),
            data_size,
            runs,
            mean_time_ms: mean_time,
            std_dev_ms: std_dev,
            min_time_ms: sorted[0],
            max_time_ms: sorted[sorted.len() - 1],
            median_time_ms: sorted[sorted.len() / 2],
            speedup_vs_baseline: None,
        }
    }

    /// Calculate speedups relative to baseline (first result per category+size)
    pub fn calculate_speedups(&mut self) {
        use std::collections::HashMap;
        let mut baselines: HashMap<(String, usize), f64> = HashMap::new();

        // First pass: find baselines (Custom Standard for each size)
        for r in &self.results {
            let key = (r.category.clone(), r.data_size);
            if !baselines.contains_key(&key) {
                baselines.insert(key, r.mean_time_ms);
            }
        }

        // Second pass: calculate speedups
        for r in &mut self.results {
            let key = (r.category.clone(), r.data_size);
            if let Some(&baseline) = baselines.get(&key) {
                r.speedup_vs_baseline = Some(baseline / r.mean_time_ms);
            }
        }
    }

    /// Save results to CSV and JSON
    pub fn save_results(&self, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = "Generated_Data/Library_Comparisons";
        std::fs::create_dir_all(output_dir)?;

        // CSV
        let csv_path = format!("{}/{}_comparison.csv", output_dir, prefix);
        let mut csv = String::from(
            "Category,Algorithm,Library,DataSize,Runs,MeanTime(ms),StdDev(ms),\
             MinTime(ms),MaxTime(ms),MedianTime(ms),SpeedupVsBaseline\n",
        );

        for r in &self.results {
            csv.push_str(&format!(
                "{},{},{},{},{},{:.4},{:.4},{:.4},{:.4},{:.4},{}\n",
                r.category,
                r.algorithm,
                r.library,
                r.data_size,
                r.runs,
                r.mean_time_ms,
                r.std_dev_ms,
                r.min_time_ms,
                r.max_time_ms,
                r.median_time_ms,
                r.speedup_vs_baseline
                    .map(|s| format!("{:.2}", s))
                    .unwrap_or_else(|| "N/A".to_string()),
            ));
        }
        std::fs::write(&csv_path, csv)?;
        println!("{}", format!("Saved: {}", csv_path).green());

        // JSON
        let json_path = format!("{}/{}_comparison.json", output_dir, prefix);
        let json = serde_json::to_string_pretty(&self.results)?;
        std::fs::write(&json_path, json)?;
        println!("{}", format!("Saved: {}", json_path).green());

        Ok(())
    }
}
