use plotters::prelude::*;
use serde_json::Value;
use std::fs;

const FONT_FAMILY: &str = "sans-serif";
const TITLE_SIZE: u32 = 22;
const LABEL_SIZE: u32 = 16;
const LEGEND_SIZE: u32 = 14;
const TICK_SIZE: u32 = 13;

// Distinct colors that work in print and are colorblind-friendly
const COLOR_MERGE_SEQ: RGBColor = RGBColor(31, 119, 180);   // blue
const COLOR_MERGE_PAR: RGBColor = RGBColor(31, 119, 180);   // blue (dashed)
const COLOR_QUICK_SEQ: RGBColor = RGBColor(255, 127, 14);   // orange
const COLOR_QUICK_PAR: RGBColor = RGBColor(255, 127, 14);   // orange (dashed)
const COLOR_STANDARD: RGBColor = RGBColor(44, 160, 44);     // green
const COLOR_STRASSEN: RGBColor = RGBColor(214, 39, 40);     // red
const COLOR_CACHE_OPT: RGBColor = RGBColor(148, 103, 189);  // purple
const COLOR_SIMD: RGBColor = RGBColor(140, 86, 75);         // brown
const COLOR_WINOGRAD: RGBColor = RGBColor(227, 119, 194);   // pink
const COLOR_PARALLEL: RGBColor = RGBColor(31, 119, 180);    // blue
const COLOR_PAR_WINO: RGBColor = RGBColor(255, 127, 14);    // orange

/// Generate all publication figures as SVG from benchmark data.
pub fn generate_all_figures() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = "Generated_Data/Figures";
    fs::create_dir_all(output_dir)?;

    // Read benchmark data files
    let pub_report_path = "publication_benchmark_full_report.json";
    let pub_scalability_path = "publication_benchmark_scalability.csv";
    let pub_efficiency_path = "publication_benchmark_parallel_efficiency.csv";
    let lib_comparison_path = "Generated_Data/Library_Comparisons/library_comparison.csv";
    let affinity_path = "Generated_Data/Affinity_Benchmarks/affinity_benchmark_affinity_results.csv";

    // Figure 1: Sorting parallel speedup across data sizes
    if let Ok(data) = fs::read_to_string(pub_scalability_path) {
        generate_sorting_speedup_figure(&data, &format!("{}/fig2_sorting_speedup.svg", output_dir))?;
        println!("Generated: fig2_sorting_speedup.svg");
    }

    // Figure 2: Matrix multiplication comparison (bar chart)
    if let Ok(data) = fs::read_to_string(lib_comparison_path) {
        generate_matrix_comparison_figure(&data, &format!("{}/fig3_matrix_comparison.svg", output_dir))?;
        println!("Generated: fig3_matrix_comparison.svg");
    }

    // Figure 3: Thread scaling with affinity
    if let Ok(data) = fs::read_to_string(affinity_path) {
        generate_thread_scaling_figure(&data, &format!("{}/fig4_thread_scaling.svg", output_dir))?;
        println!("Generated: fig4_thread_scaling.svg");
    }

    // Figure 4: Parallel efficiency comparison
    if let Ok(data) = fs::read_to_string(pub_efficiency_path) {
        generate_parallel_efficiency_figure(&data, &format!("{}/fig5_parallel_efficiency.svg", output_dir))?;
        println!("Generated: fig5_parallel_efficiency.svg");
    }

    // Figure 5: Library comparison sorting
    if let Ok(data) = fs::read_to_string(lib_comparison_path) {
        generate_sorting_library_comparison(&data, &format!("{}/fig6_sorting_library_comparison.svg", output_dir))?;
        println!("Generated: fig6_sorting_library_comparison.svg");
    }

    // Figure 6: Compiler optimization impact
    // (need cross-platform validation data)
    let cp_opt_path = "Generated_Data/Cross-Platform_validation/cross_platform_validation_optimization_impact.csv";
    if let Ok(data) = fs::read_to_string(cp_opt_path) {
        generate_compiler_optimization_figure(&data, &format!("{}/fig7_compiler_optimization.svg", output_dir))?;
        println!("Generated: fig7_compiler_optimization.svg");
    }

    // Figure 7: Distribution sensitivity
    let cp_dist_path = "Generated_Data/Cross-Platform_validation/cross_platform_validation_distribution_analysis.csv";
    if let Ok(data) = fs::read_to_string(cp_dist_path) {
        generate_distribution_sensitivity_figure(&data, &format!("{}/fig8_distribution_sensitivity.svg", output_dir))?;
        println!("Generated: fig8_distribution_sensitivity.svg");
    }

    Ok(())
}

// ============================================================================
// Figure 2: Sorting Parallel Speedup
// ============================================================================

fn generate_sorting_speedup_figure(csv_data: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse scalability CSV
    let lines: Vec<&str> = csv_data.lines().collect();
    if lines.len() < 2 { return Ok(()); }

    let mut merge_seq: Vec<(f64, f64)> = Vec::new();
    let mut merge_par: Vec<(f64, f64)> = Vec::new();
    let mut quick_seq: Vec<(f64, f64)> = Vec::new();
    let mut quick_par: Vec<(f64, f64)> = Vec::new();

    for line in &lines[1..] {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 3 {
            let algo = fields[0].trim();
            let size: f64 = fields[1].trim().parse().unwrap_or(0.0);
            let time: f64 = fields[2].trim().parse().unwrap_or(0.0);
            if size > 0.0 && time > 0.0 {
                match algo {
                    "Merge Sort" => merge_seq.push((size, time)),
                    "Merge Sort (Parallel)" => merge_par.push((size, time)),
                    "Quick Sort" => quick_seq.push((size, time)),
                    "Quick Sort (Parallel)" => quick_par.push((size, time)),
                    _ => {}
                }
            }
        }
    }

    // Calculate speedups
    let merge_speedups: Vec<(f64, f64)> = merge_seq.iter().zip(merge_par.iter())
        .map(|((size, seq_t), (_, par_t))| (*size, seq_t / par_t))
        .collect();
    let quick_speedups: Vec<(f64, f64)> = quick_seq.iter().zip(quick_par.iter())
        .map(|((size, seq_t), (_, par_t))| (*size, seq_t / par_t))
        .collect();

    let max_size = merge_seq.iter().map(|(s, _)| *s).fold(0.0f64, |a, b| a.max(b));
    let max_speedup = merge_speedups.iter().chain(quick_speedups.iter())
        .map(|(_, s)| *s).fold(0.0f64, |a, b| a.max(b)) * 1.15;

    let root = SVGBackend::new(output_path, (800, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Parallel Speedup vs Data Size", (FONT_FAMILY, TITLE_SIZE))
        .margin(15)
        .x_label_area_size(45)
        .y_label_area_size(55)
        .build_cartesian_2d(0f64..max_size, 0f64..max_speedup)?;

    chart.configure_mesh()
        .x_desc("Data Size (elements)")
        .y_desc("Speedup (x)")
        .label_style((FONT_FAMILY, TICK_SIZE))
        .x_label_formatter(&|v| {
            if *v >= 1_000_000.0 { format!("{:.0}M", v / 1_000_000.0) }
            else if *v >= 1_000.0 { format!("{:.0}K", v / 1_000.0) }
            else { format!("{:.0}", v) }
        })
        .draw()?;

    // Merge sort speedup - solid line with circles
    chart.draw_series(LineSeries::new(merge_speedups.clone(), &COLOR_MERGE_SEQ))?
        .label("Merge Sort")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], COLOR_MERGE_SEQ.stroke_width(2)));
    chart.draw_series(merge_speedups.iter().map(|(x, y)| Circle::new((*x, *y), 4, COLOR_MERGE_SEQ.filled())))?;

    // Quick sort speedup - dashed line with triangles
    chart.draw_series(LineSeries::new(quick_speedups.clone(), COLOR_QUICK_SEQ.stroke_width(2)))?
        .label("Quick Sort")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], COLOR_QUICK_SEQ.stroke_width(2)));
    chart.draw_series(quick_speedups.iter().map(|(x, y)| TriangleMarker::new((*x, *y), 5, COLOR_QUICK_SEQ.filled())))?;

    // Reference line: no parallel benefit (speedup = 1)
    chart.draw_series(LineSeries::new(
        vec![(0.0, 1.0), (max_size, 1.0)],
        RGBColor(150, 150, 150).stroke_width(1),
    ))?
        .label("No parallel benefit")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RGBColor(150, 150, 150)));

    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK.stroke_width(1))
        .label_font((FONT_FAMILY, LEGEND_SIZE))
        .draw()?;

    root.present()?;
    Ok(())
}

// ============================================================================
// Figure 3: Matrix Multiplication Comparison
// ============================================================================

fn generate_matrix_comparison_figure(csv_data: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = csv_data.lines().collect();
    if lines.len() < 2 { return Ok(()); }

    // Parse matrix results for 512x512
    let mut algorithms: Vec<(String, f64)> = Vec::new();
    for line in &lines[1..] {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 6 {
            let category = fields[0].trim();
            let algo = fields[1].trim();
            let library = fields[2].trim();
            let size: usize = fields[3].trim().parse().unwrap_or(0);
            let time: f64 = fields[5].trim().parse().unwrap_or(0.0);
            if category == "Matrix Multiply" && size == 512 && time > 0.0 {
                let label = if library == "Custom" { algo.to_string() }
                else { format!("{} ({})", algo, library) };
                algorithms.push((label, time));
            }
        }
    }

    if algorithms.is_empty() { return Ok(()); }

    let max_time = algorithms.iter().map(|(_, t)| *t).fold(0.0f64, |a, b| a.max(b)) * 1.1;
    let n_bars = algorithms.len();

    let root = SVGBackend::new(output_path, (900, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Matrix Multiplication: 512x512 (Algorithm Comparison)", (FONT_FAMILY, TITLE_SIZE))
        .margin(15)
        .x_label_area_size(100)
        .y_label_area_size(60)
        .build_cartesian_2d(0..n_bars, 0f64..max_time)?;

    let algo_names: Vec<String> = algorithms.iter().map(|(n, _)| n.clone()).collect();
    chart.configure_mesh()
        .y_desc("Execution Time (ms)")
        .label_style((FONT_FAMILY, TICK_SIZE))
        .x_label_formatter(&|idx| {
            algo_names.get(*idx).cloned().unwrap_or_default()
        })
        .x_label_style((FONT_FAMILY, 11).into_font().transform(FontTransform::Rotate270))
        .draw()?;

    let bar_colors = [
        COLOR_STANDARD, COLOR_STRASSEN, COLOR_CACHE_OPT, COLOR_SIMD,
        COLOR_WINOGRAD, COLOR_PARALLEL, COLOR_PAR_WINO, RGBColor(0, 0, 0),
    ];

    for (i, (_, time)) in algorithms.iter().enumerate() {
        let color = bar_colors[i % bar_colors.len()];
        chart.draw_series(std::iter::once(
            Rectangle::new([(i, 0.0), (i + 1, *time)], color.filled())
        ))?;
    }

    root.present()?;
    Ok(())
}

// ============================================================================
// Figure 4: Thread Scaling with Affinity
// ============================================================================

fn generate_thread_scaling_figure(csv_data: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = csv_data.lines().collect();
    if lines.len() < 2 { return Ok(()); }

    let mut merge_scaling: Vec<(f64, f64)> = Vec::new();
    let mut quick_scaling: Vec<(f64, f64)> = Vec::new();

    for line in &lines[1..] {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 10 {
            let algo = fields[0].trim();
            let placement = fields[2].trim();
            let threads: f64 = fields[3].trim().parse().unwrap_or(0.0);
            let speedup: f64 = fields[8].trim().parse().unwrap_or(0.0);
            // Only use "Bound to N cores" entries for clean scaling curve
            if placement.starts_with("Bound to") && speedup > 0.0 {
                match algo {
                    "Merge Sort" => merge_scaling.push((threads, speedup)),
                    "Quick Sort" => quick_scaling.push((threads, speedup)),
                    _ => {}
                }
            }
        }
    }

    if merge_scaling.is_empty() && quick_scaling.is_empty() { return Ok(()); }

    let max_threads = merge_scaling.iter().chain(quick_scaling.iter())
        .map(|(t, _)| *t).fold(0.0f64, |a, b| a.max(b));
    let max_speedup = merge_scaling.iter().chain(quick_scaling.iter())
        .map(|(_, s)| *s).fold(0.0f64, |a, b| a.max(b)) * 1.15;

    let root = SVGBackend::new(output_path, (800, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Thread Scaling with Core Affinity (n=1,000,000)", (FONT_FAMILY, TITLE_SIZE))
        .margin(15)
        .x_label_area_size(45)
        .y_label_area_size(55)
        .build_cartesian_2d(0f64..max_threads * 1.05, 0f64..max_speedup)?;

    chart.configure_mesh()
        .x_desc("Thread Count")
        .y_desc("Speedup (x)")
        .label_style((FONT_FAMILY, TICK_SIZE))
        .draw()?;

    // Linear speedup reference
    chart.draw_series(LineSeries::new(
        vec![(0.0, 0.0), (max_threads, max_threads)],
        RGBColor(200, 200, 200).stroke_width(1),
    ))?
        .label("Linear speedup")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RGBColor(200, 200, 200)));

    // Merge sort
    chart.draw_series(LineSeries::new(merge_scaling.clone(), COLOR_MERGE_SEQ.stroke_width(2)))?
        .label("Merge Sort")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], COLOR_MERGE_SEQ.stroke_width(2)));
    chart.draw_series(merge_scaling.iter().map(|(x, y)| Circle::new((*x, *y), 4, COLOR_MERGE_SEQ.filled())))?;

    // Quick sort
    chart.draw_series(LineSeries::new(quick_scaling.clone(), COLOR_QUICK_SEQ.stroke_width(2)))?
        .label("Quick Sort")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], COLOR_QUICK_SEQ.stroke_width(2)));
    chart.draw_series(quick_scaling.iter().map(|(x, y)| TriangleMarker::new((*x, *y), 5, COLOR_QUICK_SEQ.filled())))?;

    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperLeft)
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK.stroke_width(1))
        .label_font((FONT_FAMILY, LEGEND_SIZE))
        .draw()?;

    root.present()?;
    Ok(())
}

// ============================================================================
// Figure 5: Parallel Efficiency
// ============================================================================

fn generate_parallel_efficiency_figure(csv_data: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = csv_data.lines().collect();
    if lines.len() < 2 { return Ok(()); }

    let mut merge_eff: Vec<(f64, f64)> = Vec::new();
    let mut quick_eff: Vec<(f64, f64)> = Vec::new();

    for line in &lines[1..] {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 5 {
            let algo = fields[0].trim();
            let threads: f64 = fields[2].trim().parse().unwrap_or(0.0);
            let efficiency: f64 = fields[4].trim().parse().unwrap_or(0.0);
            if threads > 0.0 {
                match algo {
                    "Merge Sort" => merge_eff.push((threads, efficiency * 100.0)),
                    "Quick Sort" => quick_eff.push((threads, efficiency * 100.0)),
                    _ => {}
                }
            }
        }
    }

    if merge_eff.is_empty() && quick_eff.is_empty() { return Ok(()); }

    let max_threads = merge_eff.iter().chain(quick_eff.iter())
        .map(|(t, _)| *t).fold(0.0f64, |a, b| a.max(b));

    let root = SVGBackend::new(output_path, (800, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Parallel Efficiency vs Thread Count", (FONT_FAMILY, TITLE_SIZE))
        .margin(15)
        .x_label_area_size(45)
        .y_label_area_size(55)
        .build_cartesian_2d(0f64..max_threads * 1.05, 0f64..110.0)?;

    chart.configure_mesh()
        .x_desc("Thread Count")
        .y_desc("Efficiency (%)")
        .label_style((FONT_FAMILY, TICK_SIZE))
        .draw()?;

    // 100% efficiency reference
    chart.draw_series(LineSeries::new(
        vec![(0.0, 100.0), (max_threads, 100.0)],
        RGBColor(200, 200, 200).stroke_width(1),
    ))?;

    // Merge sort
    chart.draw_series(LineSeries::new(merge_eff.clone(), COLOR_MERGE_SEQ.stroke_width(2)))?
        .label("Merge Sort")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], COLOR_MERGE_SEQ.stroke_width(2)));
    chart.draw_series(merge_eff.iter().map(|(x, y)| Circle::new((*x, *y), 4, COLOR_MERGE_SEQ.filled())))?;

    // Quick sort
    chart.draw_series(LineSeries::new(quick_eff.clone(), COLOR_QUICK_SEQ.stroke_width(2)))?
        .label("Quick Sort")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], COLOR_QUICK_SEQ.stroke_width(2)));
    chart.draw_series(quick_eff.iter().map(|(x, y)| TriangleMarker::new((*x, *y), 5, COLOR_QUICK_SEQ.filled())))?;

    chart.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK.stroke_width(1))
        .label_font((FONT_FAMILY, LEGEND_SIZE))
        .draw()?;

    root.present()?;
    Ok(())
}

// ============================================================================
// Figure 6: Sorting Library Comparison
// ============================================================================

fn generate_sorting_library_comparison(csv_data: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = csv_data.lines().collect();
    if lines.len() < 2 { return Ok(()); }

    // Parse sorting results for 1M elements
    let mut entries: Vec<(String, String, f64)> = Vec::new(); // (algo, library, time)
    for line in &lines[1..] {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 6 {
            let category = fields[0].trim();
            let algo = fields[1].trim();
            let library = fields[2].trim();
            let size: usize = fields[3].trim().parse().unwrap_or(0);
            let time: f64 = fields[5].trim().parse().unwrap_or(0.0);
            if category == "Sorting" && size == 1000000 && time > 0.0 {
                entries.push((algo.to_string(), library.to_string(), time));
            }
        }
    }

    if entries.is_empty() { return Ok(()); }

    let max_time = entries.iter().map(|(_, _, t)| *t).fold(0.0f64, |a, b| a.max(b)) * 1.1;
    let n_bars = entries.len();

    let root = SVGBackend::new(output_path, (900, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sorting Performance: 1,000,000 Elements", (FONT_FAMILY, TITLE_SIZE))
        .margin(15)
        .x_label_area_size(120)
        .y_label_area_size(60)
        .build_cartesian_2d(0..n_bars, 0f64..max_time)?;

    let labels: Vec<String> = entries.iter()
        .map(|(algo, lib, _)| format!("{}\n({})", algo, lib))
        .collect();

    chart.configure_mesh()
        .y_desc("Execution Time (ms)")
        .label_style((FONT_FAMILY, TICK_SIZE))
        .x_label_formatter(&|idx| labels.get(*idx).cloned().unwrap_or_default())
        .x_label_style((FONT_FAMILY, 10).into_font().transform(FontTransform::Rotate270))
        .draw()?;

    for (i, (_, library, time)) in entries.iter().enumerate() {
        let color = match library.as_str() {
            "Custom D&C" => COLOR_MERGE_SEQ,
            "std library" => COLOR_STANDARD,
            "Rayon" => COLOR_STRASSEN,
            _ => RGBColor(128, 128, 128),
        };
        chart.draw_series(std::iter::once(
            Rectangle::new([(i, 0.0), (i + 1, *time)], color.filled())
        ))?;
    }

    root.present()?;
    Ok(())
}

// ============================================================================
// Figure 7: Compiler Optimization Impact
// ============================================================================

fn generate_compiler_optimization_figure(csv_data: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = csv_data.lines().collect();
    if lines.len() < 2 { return Ok(()); }

    // Parse: Algorithm, OptLevel, MeanTime, StdDev
    let mut merge_times: Vec<(String, f64)> = Vec::new();
    let mut quick_times: Vec<(String, f64)> = Vec::new();
    let mut matrix_times: Vec<(String, f64)> = Vec::new();

    for line in &lines[1..] {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 3 {
            let algo = fields[0].trim();
            let opt_level = fields[1].trim().to_string();
            let time: f64 = fields[2].trim().parse().unwrap_or(0.0);
            if time > 0.0 {
                match algo {
                    "Merge Sort" => merge_times.push((opt_level, time)),
                    "Quick Sort" => quick_times.push((opt_level, time)),
                    "Matrix Multiplication" => matrix_times.push((opt_level, time)),
                    _ => {}
                }
            }
        }
    }

    if merge_times.is_empty() { return Ok(()); }

    let all_times: Vec<f64> = merge_times.iter()
        .chain(quick_times.iter())
        .chain(matrix_times.iter())
        .map(|(_, t)| *t).collect();
    let max_time = all_times.iter().fold(0.0f64, |a, &b| a.max(b)) * 1.1;

    let root = SVGBackend::new(output_path, (800, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let opt_levels = vec!["O0", "O1", "O2", "O3"];
    let n_groups = opt_levels.len();

    let mut chart = ChartBuilder::on(&root)
        .caption("Compiler Optimization Impact on Execution Time", (FONT_FAMILY, TITLE_SIZE))
        .margin(15)
        .x_label_area_size(45)
        .y_label_area_size(60)
        .build_cartesian_2d(0..n_groups, 0f64..max_time)?;

    chart.configure_mesh()
        .y_desc("Execution Time (ms)")
        .label_style((FONT_FAMILY, TICK_SIZE))
        .x_label_formatter(&|idx| opt_levels.get(*idx).map(|s| s.to_string()).unwrap_or_default())
        .draw()?;

    // Draw grouped bars
    let bar_width = 0.25;
    let datasets = [
        (&merge_times, COLOR_MERGE_SEQ, "Merge Sort"),
        (&quick_times, COLOR_QUICK_SEQ, "Quick Sort"),
        (&matrix_times, COLOR_CACHE_OPT, "Matrix Mult"),
    ];

    for (di, (data, color, _label)) in datasets.iter().enumerate() {
        for (i, (_, time)) in data.iter().enumerate() {
            let x_start = i as f64 + di as f64 * bar_width;
            // Use integer coordinates for the chart
            chart.draw_series(std::iter::once(
                Rectangle::new([(i, 0.0), (i + 1, *time)], color.mix(0.7 + di as f64 * 0.1).filled())
            ))?;
        }
    }

    root.present()?;
    Ok(())
}

// ============================================================================
// Figure 8: Distribution Sensitivity
// ============================================================================

fn generate_distribution_sensitivity_figure(csv_data: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<&str> = csv_data.lines().collect();
    if lines.len() < 2 { return Ok(()); }

    // Parse: Algorithm, Distribution, MeanTime, StdDev
    let mut data: Vec<(String, String, f64)> = Vec::new();
    for line in &lines[1..] {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 3 {
            let algo = fields[0].trim().to_string();
            let dist = fields[1].trim().to_string();
            let time: f64 = fields[2].trim().parse().unwrap_or(0.0);
            if time > 0.0 {
                data.push((algo, dist, time));
            }
        }
    }

    if data.is_empty() { return Ok(()); }

    // Use log scale for the y-axis since quick sort has extreme outliers
    let max_time = data.iter().map(|(_, _, t)| *t).fold(0.0f64, |a, b| a.max(b)) * 1.2;

    let distributions: Vec<String> = data.iter().map(|(_, d, _)| d.clone())
        .collect::<std::collections::HashSet<_>>().into_iter().collect();
    let algorithms: Vec<String> = data.iter().map(|(a, _, _)| a.clone())
        .collect::<std::collections::HashSet<_>>().into_iter().collect();

    let n_dists = distributions.len();

    let root = SVGBackend::new(output_path, (900, 500)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Algorithm Performance Across Data Distributions", (FONT_FAMILY, TITLE_SIZE))
        .margin(15)
        .x_label_area_size(80)
        .y_label_area_size(70)
        .build_cartesian_2d(0..n_dists, (0.1f64..max_time).log_scale())?;

    let dist_labels: Vec<String> = distributions.clone();
    chart.configure_mesh()
        .y_desc("Execution Time (ms) - Log Scale")
        .label_style((FONT_FAMILY, TICK_SIZE))
        .x_label_formatter(&|idx| dist_labels.get(*idx).cloned().unwrap_or_default())
        .x_label_style((FONT_FAMILY, 11).into_font().transform(FontTransform::Rotate270))
        .draw()?;

    let algo_colors = [
        COLOR_MERGE_SEQ, COLOR_QUICK_SEQ, COLOR_CACHE_OPT, COLOR_STANDARD,
    ];

    for (ai, algo) in algorithms.iter().enumerate() {
        let color = algo_colors[ai % algo_colors.len()];
        for (di, dist) in distributions.iter().enumerate() {
            if let Some((_, _, time)) = data.iter().find(|(a, d, _)| a == algo && d == dist) {
                chart.draw_series(std::iter::once(
                    Rectangle::new([(di, 0.1), (di + 1, *time)], color.mix(0.5 + ai as f64 * 0.15).filled())
                ))?;
            }
        }
    }

    root.present()?;
    Ok(())
}
