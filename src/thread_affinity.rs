use colored::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::data_generator::DataGenerator;
use crate::sorting;

// ============================================================================
// Core Topology Detection
// ============================================================================

/// Represents the core topology of a hybrid CPU (e.g., Intel 13th gen with P-cores and E-cores)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreTopology {
    pub total_logical_cores: usize,
    pub total_physical_cores: usize,
    /// Performance core IDs (detected or configured)
    pub p_core_ids: Vec<usize>,
    /// Efficiency core IDs (detected or configured)
    pub e_core_ids: Vec<usize>,
    /// Whether hybrid architecture was detected
    pub is_hybrid: bool,
    pub topology_source: String,
}

/// Thread placement strategy for benchmarking
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ThreadPlacement {
    /// Use only performance cores (P-cores)
    PerformanceCoresOnly,
    /// Use only efficiency cores (E-cores)
    EfficiencyCoresOnly,
    /// Use all cores with OS scheduling (default behavior)
    AllCoresDefault,
    /// Use only physical cores (no hyperthreading)
    PhysicalCoresOnly,
    /// Use specific core count with affinity binding
    BoundCores(usize),
}

impl std::fmt::Display for ThreadPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreadPlacement::PerformanceCoresOnly => write!(f, "P-cores only"),
            ThreadPlacement::EfficiencyCoresOnly => write!(f, "E-cores only"),
            ThreadPlacement::AllCoresDefault => write!(f, "All cores (OS scheduled)"),
            ThreadPlacement::PhysicalCoresOnly => write!(f, "Physical cores only (no HT)"),
            ThreadPlacement::BoundCores(n) => write!(f, "Bound to {} cores", n),
        }
    }
}

/// Result of a thread-affinity-aware benchmark run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffinityBenchmarkResult {
    pub algorithm: String,
    pub data_size: usize,
    pub placement: String,
    pub thread_count: usize,
    pub core_ids_used: Vec<usize>,
    pub mean_time_ms: f64,
    pub std_dev_ms: f64,
    pub min_time_ms: f64,
    pub max_time_ms: f64,
    pub speedup_vs_sequential: f64,
    pub efficiency_percent: f64,
    pub individual_runs_ms: Vec<f64>,
}

impl CoreTopology {
    /// Detect core topology from the system.
    /// For Intel 13th gen i7-13650HX:
    ///   - 6 P-cores (cores 0-5, with HT: logical 0-11)
    ///   - 8 E-cores (cores 6-13, logical 12-19)
    pub fn detect() -> Self {
        let total_logical = num_cpus::get();
        let total_physical = num_cpus::get_physical();

        // Try to detect hybrid topology from sysfs
        let (p_cores, e_cores, is_hybrid, source) = Self::detect_from_sysfs()
            .unwrap_or_else(|| Self::default_topology(total_physical, total_logical));

        CoreTopology {
            total_logical_cores: total_logical,
            total_physical_cores: total_physical,
            p_core_ids: p_cores,
            e_core_ids: e_cores,
            is_hybrid,
            topology_source: source,
        }
    }

    /// Create a manually configured topology for Intel 13th gen i7-13650HX
    pub fn intel_13th_gen_13650hx() -> Self {
        // i7-13650HX: 6 P-cores (HT) + 8 E-cores = 14 physical, 20 logical
        // P-cores: physical 0-5, logical 0-11 (with HT pairs)
        // E-cores: physical 6-13, logical 12-19
        CoreTopology {
            total_logical_cores: 20,
            total_physical_cores: 14,
            p_core_ids: (0..12).collect(),       // P-core logical IDs (6 cores * 2 HT)
            e_core_ids: (12..20).collect(),      // E-core logical IDs (8 cores)
            is_hybrid: true,
            topology_source: "Manual: Intel i7-13650HX (6P+8E)".to_string(),
        }
    }

    /// Try to detect core topology from Linux sysfs
    fn detect_from_sysfs() -> Option<(Vec<usize>, Vec<usize>, bool, String)> {
        // Read CPU frequency scaling to distinguish P-cores from E-cores
        let mut core_freqs: Vec<(usize, u64)> = Vec::new();

        for core_id in 0..num_cpus::get() {
            let freq_path = format!(
                "/sys/devices/system/cpu/cpu{}/cpufreq/cpuinfo_max_freq",
                core_id
            );
            if let Ok(freq_str) = std::fs::read_to_string(&freq_path) {
                if let Ok(freq) = freq_str.trim().parse::<u64>() {
                    core_freqs.push((core_id, freq));
                }
            }
        }

        if core_freqs.is_empty() {
            return None;
        }

        // Find max frequency to identify P-cores
        let max_freq = core_freqs.iter().map(|(_, f)| *f).max()?;
        let min_freq_threshold = max_freq * 80 / 100; // P-cores have >80% of max freq

        let mut p_cores = Vec::new();
        let mut e_cores = Vec::new();

        for (core_id, freq) in &core_freqs {
            if *freq >= min_freq_threshold {
                p_cores.push(*core_id);
            } else {
                e_cores.push(*core_id);
            }
        }

        let is_hybrid = !p_cores.is_empty() && !e_cores.is_empty();

        if is_hybrid {
            Some((
                p_cores,
                e_cores,
                true,
                format!("Detected from sysfs (max_freq={}KHz)", max_freq),
            ))
        } else {
            // All cores have similar frequency — homogeneous architecture
            let all_cores: Vec<usize> = core_freqs.iter().map(|(id, _)| *id).collect();
            Some((
                all_cores,
                Vec::new(),
                false,
                "Detected from sysfs (homogeneous)".to_string(),
            ))
        }
    }

    /// Default topology when sysfs detection fails
    fn default_topology(physical: usize, logical: usize) -> (Vec<usize>, Vec<usize>, bool, String) {
        (
            (0..logical).collect(),
            Vec::new(),
            false,
            format!("Default ({} physical, {} logical)", physical, logical),
        )
    }

    /// Get core IDs for a given placement strategy
    pub fn cores_for_placement(&self, placement: ThreadPlacement) -> Vec<usize> {
        match placement {
            ThreadPlacement::PerformanceCoresOnly => {
                if self.is_hybrid {
                    self.p_core_ids.clone()
                } else {
                    (0..self.total_physical_cores).collect()
                }
            }
            ThreadPlacement::EfficiencyCoresOnly => {
                if self.is_hybrid {
                    self.e_core_ids.clone()
                } else {
                    (0..self.total_physical_cores).collect()
                }
            }
            ThreadPlacement::AllCoresDefault => {
                (0..self.total_logical_cores).collect()
            }
            ThreadPlacement::PhysicalCoresOnly => {
                if self.is_hybrid {
                    // P-cores without HT + E-cores
                    let p_physical: Vec<usize> = self.p_core_ids.iter()
                        .step_by(2) // Take every other P-core ID (skip HT sibling)
                        .cloned()
                        .collect();
                    let mut cores = p_physical;
                    cores.extend(&self.e_core_ids);
                    cores
                } else {
                    (0..self.total_physical_cores).collect()
                }
            }
            ThreadPlacement::BoundCores(n) => {
                (0..n.min(self.total_logical_cores)).collect()
            }
        }
    }

    pub fn print_topology(&self) {
        println!("{}", "=== CPU Core Topology ===".bright_green().bold());
        println!("  Total logical cores:  {}", self.total_logical_cores);
        println!("  Total physical cores: {}", self.total_physical_cores);
        println!("  Hybrid architecture:  {}", if self.is_hybrid { "Yes" } else { "No" });
        println!("  Detection source:     {}", self.topology_source);
        if self.is_hybrid {
            println!("  P-core IDs: {:?} ({} logical)", self.p_core_ids, self.p_core_ids.len());
            println!("  E-core IDs: {:?} ({} logical)", self.e_core_ids, self.e_core_ids.len());
        }
        println!();
    }
}

// ============================================================================
// Affinity-Aware Benchmark Runner
// ============================================================================

pub struct AffinityBenchmarkRunner {
    pub topology: CoreTopology,
    pub results: Vec<AffinityBenchmarkResult>,
}

impl AffinityBenchmarkRunner {
    pub fn new() -> Self {
        let topology = CoreTopology::detect();
        topology.print_topology();
        Self {
            topology,
            results: Vec::new(),
        }
    }

    pub fn with_topology(topology: CoreTopology) -> Self {
        topology.print_topology();
        Self {
            topology,
            results: Vec::new(),
        }
    }

    /// Run a sorting benchmark with specific thread placement.
    pub fn benchmark_sort_with_placement(
        &mut self,
        algorithm: &str,
        data_size: usize,
        placement: ThreadPlacement,
        runs: usize,
    ) {
        let core_ids = self.topology.cores_for_placement(placement);
        let thread_count = core_ids.len();

        println!(
            "{}",
            format!(
                "  {} | {} | {} threads | cores: {:?}",
                algorithm, placement, thread_count,
                if core_ids.len() > 8 { format!("{:?}...", &core_ids[..8]) } else { format!("{:?}", core_ids) }
            ).cyan()
        );

        // Get sequential baseline
        let test_data = DataGenerator::generate_random_integers(data_size);
        let sequential_time = {
            let mut seq_data = test_data.clone();
            let start = Instant::now();
            match algorithm {
                "Merge Sort" => sorting::merge_sort(&mut seq_data),
                "Quick Sort" => sorting::quick_sort(&mut seq_data),
                _ => panic!("Unknown algorithm: {}", algorithm),
            }
            start.elapsed().as_secs_f64() * 1000.0
        };

        // Build thread pool with per-thread CPU affinity binding
        let affinity_cores: Vec<core_affinity::CoreId> = core_ids
            .iter()
            .map(|&id| core_affinity::CoreId { id })
            .collect();

        let cores_for_handler = affinity_cores.clone();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .start_handler(move |thread_index| {
                // Pin each Rayon worker thread to a specific core
                if thread_index < cores_for_handler.len() {
                    let _ = core_affinity::set_for_current(cores_for_handler[thread_index]);
                }
            })
            .build()
            .unwrap();

        let mut times = Vec::new();

        for _run in 0..runs {
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
            times.push(parallel_time);
        }

        let mean_time = statistical::mean(&times);
        let std_dev = if times.len() > 1 {
            statistical::standard_deviation(&times, Some(mean_time))
        } else {
            0.0
        };
        let min_time = times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_time = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let speedup = sequential_time / mean_time;
        let efficiency = (speedup / thread_count as f64) * 100.0;

        println!(
            "    Mean: {:.3}ms | Speedup: {:.2}x | Efficiency: {:.1}%",
            mean_time, speedup, efficiency
        );

        self.results.push(AffinityBenchmarkResult {
            algorithm: algorithm.to_string(),
            data_size,
            placement: format!("{}", placement),
            thread_count,
            core_ids_used: core_ids,
            mean_time_ms: mean_time,
            std_dev_ms: std_dev,
            min_time_ms: min_time,
            max_time_ms: max_time,
            speedup_vs_sequential: speedup,
            efficiency_percent: efficiency,
            individual_runs_ms: times,
        });
    }

    /// Run comprehensive affinity benchmarks across all placement strategies.
    pub fn run_comprehensive_affinity_benchmark(
        &mut self,
        data_size: usize,
        runs: usize,
    ) {
        println!(
            "{}",
            format!("=== Thread Affinity Benchmark (n={}) ===", data_size)
                .bright_magenta()
                .bold()
        );

        let placements = if self.topology.is_hybrid {
            vec![
                ThreadPlacement::PerformanceCoresOnly,
                ThreadPlacement::EfficiencyCoresOnly,
                ThreadPlacement::PhysicalCoresOnly,
                ThreadPlacement::AllCoresDefault,
            ]
        } else {
            vec![
                ThreadPlacement::PhysicalCoresOnly,
                ThreadPlacement::AllCoresDefault,
            ]
        };

        for algorithm in &["Merge Sort", "Quick Sort"] {
            println!(
                "\n{}",
                format!("--- {} ---", algorithm).bright_yellow()
            );
            for &placement in &placements {
                self.benchmark_sort_with_placement(algorithm, data_size, placement, runs);
            }
        }
    }

    /// Run thread scaling analysis with affinity control.
    /// Tests specific thread counts while binding to cores.
    pub fn run_scaling_with_affinity(
        &mut self,
        algorithm: &str,
        data_size: usize,
        thread_counts: &[usize],
        runs: usize,
    ) {
        println!(
            "{}",
            format!(
                "=== Thread Scaling with Affinity: {} (n={}) ===",
                algorithm, data_size
            )
            .bright_green()
            .bold()
        );

        for &count in thread_counts {
            let placement = ThreadPlacement::BoundCores(count);
            self.benchmark_sort_with_placement(algorithm, data_size, placement, runs);
        }
    }

    /// Save results to CSV
    pub fn save_results(&self, prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output_dir = "Generated_Data/Affinity_Benchmarks";
        std::fs::create_dir_all(output_dir)?;

        // Save CSV
        let csv_path = format!("{}/{}_affinity_results.csv", output_dir, prefix);
        let mut csv_content = String::from(
            "Algorithm,DataSize,Placement,ThreadCount,MeanTime(ms),StdDev(ms),\
             MinTime(ms),MaxTime(ms),Speedup,Efficiency(%),CoreIDs\n",
        );

        for r in &self.results {
            csv_content.push_str(&format!(
                "{},{},{},{},{:.4},{:.4},{:.4},{:.4},{:.2},{:.1},\"{:?}\"\n",
                r.algorithm,
                r.data_size,
                r.placement,
                r.thread_count,
                r.mean_time_ms,
                r.std_dev_ms,
                r.min_time_ms,
                r.max_time_ms,
                r.speedup_vs_sequential,
                r.efficiency_percent,
                r.core_ids_used
            ));
        }

        std::fs::write(&csv_path, csv_content)?;
        println!("{}", format!("Saved: {}", csv_path).green());

        // Save JSON
        let json_path = format!("{}/{}_affinity_report.json", output_dir, prefix);
        let report = serde_json::json!({
            "topology": self.topology,
            "results": self.results,
        });
        std::fs::write(&json_path, serde_json::to_string_pretty(&report)?)?;
        println!("{}", format!("Saved: {}", json_path).green());

        Ok(())
    }
}

// ============================================================================
// Threshold Optimization
// ============================================================================

/// Find the optimal parallel threshold for a given algorithm and data size
/// by testing multiple threshold values.
pub fn optimize_threshold(
    algorithm: &str,
    data_size: usize,
    runs: usize,
) -> Vec<(usize, f64)> {
    println!(
        "{}",
        format!(
            "=== Threshold Optimization: {} (n={}) ===",
            algorithm, data_size
        )
        .bright_green()
        .bold()
    );

    let test_data = DataGenerator::generate_random_integers(data_size);
    let thresholds = vec![256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536];
    let mut results = Vec::new();

    for &threshold in &thresholds {
        if threshold >= data_size {
            continue;
        }

        let mut times = Vec::new();
        for _ in 0..runs {
            let time = match algorithm {
                "Merge Sort" => sorting::benchmark_merge_sort_threshold(&test_data, threshold),
                "Quick Sort" => sorting::benchmark_quick_sort_threshold(&test_data, threshold),
                _ => panic!("Unknown algorithm: {}", algorithm),
            };
            times.push(time);
        }

        let mean_time = statistical::mean(&times);
        println!(
            "  Threshold {:>6}: {:.3}ms",
            threshold, mean_time
        );
        results.push((threshold, mean_time));
    }

    if let Some((best_threshold, best_time)) = results
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    {
        println!(
            "{}",
            format!(
                "  Optimal threshold: {} ({:.3}ms)",
                best_threshold, best_time
            )
            .bright_green()
        );
    }

    results
}
