use rand::prelude::*;
use crate::geometry::Point;
use crate::matrix::Matrix;

pub struct DataGenerator;

impl DataGenerator {
    /// Generate random integer array
    pub fn generate_random_integers(size: usize) -> Vec<i32> {
        let mut rng = thread_rng();
        (0..size).map(|_| rng.gen_range(-1000000..=1000000)).collect()
    }

    /// Generate sorted array (for worst case testing)
    pub fn generate_sorted_integers(size: usize) -> Vec<i32> {
        (0..size as i32).collect()
    }

    /// Generate reverse sorted array (for worst case testing)
    pub fn generate_reverse_sorted_integers(size: usize) -> Vec<i32> {
        (0..size as i32).rev().collect()
    }

    /// Generate partially sorted array
    pub fn generate_partially_sorted_integers(size: usize, sorted_ratio: f64) -> Vec<i32> {
        let mut data = Self::generate_random_integers(size);
        let sorted_count = (size as f64 * sorted_ratio) as usize;
        
        // Sort the first portion
        data[..sorted_count].sort();
        
        data
    }

    /// Generate array with many duplicate elements
    pub fn generate_duplicate_heavy_integers(size: usize, unique_values: usize) -> Vec<i32> {
        let mut rng = thread_rng();
        let unique_vals: Vec<i32> = (0..unique_values as i32).collect();
        
        (0..size)
            .map(|_| unique_vals[rng.gen_range(0..unique_vals.len())])
            .collect()
    }

    /// Generate random 2D points
    pub fn generate_random_points(count: usize) -> Vec<Point> {
        let mut rng = thread_rng();
        (0..count)
            .map(|_| Point {
                x: rng.gen_range(-1000.0..=1000.0),
                y: rng.gen_range(-1000.0..=1000.0),
            })
            .collect()
    }

    /// Generate points on a circle (specific pattern)
    pub fn generate_circular_points(count: usize, radius: f64) -> Vec<Point> {
        (0..count)
            .map(|i| {
                let angle = 2.0 * std::f64::consts::PI * i as f64 / count as f64;
                Point {
                    x: radius * angle.cos(),
                    y: radius * angle.sin(),
                }
            })
            .collect()
    }

    /// Generate grid points
    pub fn generate_grid_points(grid_size: usize) -> Vec<Point> {
        let mut points = Vec::new();
        for i in 0..grid_size {
            for j in 0..grid_size {
                points.push(Point {
                    x: i as f64,
                    y: j as f64,
                });
            }
        }
        points
    }

    /// Generate clustered points
    pub fn generate_clustered_points(cluster_count: usize, points_per_cluster: usize, cluster_radius: f64) -> Vec<Point> {
        let mut rng = thread_rng();
        let mut points = Vec::new();
        
        for _ in 0..cluster_count {
            // Randomly determine cluster center
            let center_x = rng.gen_range(-500.0..=500.0);
            let center_y = rng.gen_range(-500.0..=500.0);
            
            // Generate points within cluster
            for _ in 0..points_per_cluster {
                let angle = rng.gen_range(0.0..2.0 * std::f64::consts::PI);
                let distance = rng.gen_range(0.0..cluster_radius);
                
                points.push(Point {
                    x: center_x + distance * angle.cos(),
                    y: center_y + distance * angle.sin(),
                });
            }
        }
        
        points
    }

    /// Generate random square matrix pair
    pub fn generate_random_matrices(size: usize) -> (Matrix, Matrix) {
        let mut rng = thread_rng();
        
        let matrix_a = Matrix::new(size, |_, _| rng.gen_range(-100.0..=100.0));
        let matrix_b = Matrix::new(size, |_, _| rng.gen_range(-100.0..=100.0));
        
        (matrix_a, matrix_b)
    }

    /// Generate identity matrix
    pub fn generate_identity_matrix(size: usize) -> Matrix {
        Matrix::new(size, |i, j| if i == j { 1.0 } else { 0.0 })
    }

    /// Generate sparse matrix (many elements are 0)
    pub fn generate_sparse_matrix(size: usize, density: f64) -> Matrix {
        let mut rng = thread_rng();
        
        Matrix::new(size, |_, _| {
            if rng.gen::<f64>() < density {
                rng.gen_range(-100.0..=100.0)
            } else {
                0.0
            }
        })
    }

    /// Generate diagonal matrix
    pub fn generate_diagonal_matrix(size: usize) -> Matrix {
        let mut rng = thread_rng();
        
        Matrix::new(size, |i, j| {
            if i == j {
                rng.gen_range(1.0..=100.0)
            } else {
                0.0
            }
        })
    }

    /// Generate test datasets for testing
    pub fn generate_test_datasets() -> TestDatasets {
        TestDatasets {
            small_integers: Self::generate_random_integers(100),
            medium_integers: Self::generate_random_integers(1000),
            large_integers: Self::generate_random_integers(10000),
            sorted_integers: Self::generate_sorted_integers(1000),
            reverse_sorted_integers: Self::generate_reverse_sorted_integers(1000),
            duplicate_heavy_integers: Self::generate_duplicate_heavy_integers(1000, 10),
            random_points: Self::generate_random_points(1000),
            circular_points: Self::generate_circular_points(100, 50.0),
            clustered_points: Self::generate_clustered_points(5, 20, 10.0),
            small_matrices: Self::generate_random_matrices(8),
            medium_matrices: Self::generate_random_matrices(64),
            large_matrices: Self::generate_random_matrices(256),
        }
    }
}

/// Collection of test datasets
pub struct TestDatasets {
    pub small_integers: Vec<i32>,
    pub medium_integers: Vec<i32>,
    pub large_integers: Vec<i32>,
    pub sorted_integers: Vec<i32>,
    pub reverse_sorted_integers: Vec<i32>,
    pub duplicate_heavy_integers: Vec<i32>,
    pub random_points: Vec<Point>,
    pub circular_points: Vec<Point>,
    pub clustered_points: Vec<Point>,
    pub small_matrices: (Matrix, Matrix),
    pub medium_matrices: (Matrix, Matrix),
    pub large_matrices: (Matrix, Matrix),
}

impl TestDatasets {
    /// Display dataset statistics
    pub fn print_statistics(&self) {
        println!("=== Test Dataset Statistics ===");
        
        println!("Integer data:");
        println!("  Small: {} elements", self.small_integers.len());
        println!("  Medium: {} elements", self.medium_integers.len());
        println!("  Large: {} elements", self.large_integers.len());
        
        println!("Point data:");
        println!("  Random: {} points", self.random_points.len());
        println!("  Circular: {} points", self.circular_points.len());
        println!("  Clustered: {} points", self.clustered_points.len());
        
        println!("Matrix data:");
        println!("  Small: {}x{}", self.small_matrices.0.size(), self.small_matrices.0.size());
        println!("  Medium: {}x{}", self.medium_matrices.0.size(), self.medium_matrices.0.size());
        println!("  Large: {}x{}", self.large_matrices.0.size(), self.large_matrices.0.size());
    }
}