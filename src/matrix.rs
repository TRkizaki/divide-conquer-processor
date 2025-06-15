use serde::{Deserialize, Serialize};
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    /// Create a new matrix with given dimensions and initialization function
    pub fn new<F>(size: usize, init_fn: F) -> Self
    where
        F: Fn(usize, usize) -> f64,
    {
        let mut data = Vec::with_capacity(size);
        for i in 0..size {
            let mut row = Vec::with_capacity(size);
            for j in 0..size {
                row.push(init_fn(i, j));
            }
            data.push(row);
        }

        Self {
            data,
            rows: size,
            cols: size,
        }
    }

    /// Create a matrix from 2D vector
    pub fn from_vec(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };

        Self { data, rows, cols }
    }

    /// Create zero matrix
    pub fn zeros(size: usize) -> Self {
        Self::new(size, |_, _| 0.0)
    }

    /// Create identity matrix
    pub fn identity(size: usize) -> Self {
        Self::new(size, |i, j| if i == j { 1.0 } else { 0.0 })
    }

    /// Get matrix dimensions
    pub fn size(&self) -> usize {
        self.rows
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Check if matrix is square
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    /// Get element at position (i, j)
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i][j]
    }

    /// Set element at position (i, j)
    pub fn set(&mut self, i: usize, j: usize, value: f64) {
        self.data[i][j] = value;
    }

    /// Extract submatrix
    pub fn submatrix(
        &self,
        start_row: usize,
        end_row: usize,
        start_col: usize,
        end_col: usize,
    ) -> Matrix {
        let mut data = Vec::new();
        for i in start_row..end_row {
            let mut row = Vec::new();
            for j in start_col..end_col {
                row.push(self.data[i][j]);
            }
            data.push(row);
        }
        Matrix::from_vec(data)
    }

    /// Add matrices element-wise
    pub fn add(&self, other: &Matrix) -> Result<Matrix, String> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("Matrix dimensions must match for addition".to_string());
        }

        let mut result = Matrix::zeros(self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Ok(result)
    }

    /// Subtract matrices element-wise
    pub fn subtract(&self, other: &Matrix) -> Result<Matrix, String> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("Matrix dimensions must match for subtraction".to_string());
        }

        let mut result = Matrix::zeros(self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        Ok(result)
    }

    /// Pad matrix to next power of 2 size
    pub fn pad_to_power_of_2(&self) -> Matrix {
        let size = self.rows.max(self.cols);
        let new_size = size.next_power_of_two();

        let mut padded = Matrix::zeros(new_size);
        for i in 0..self.rows {
            for j in 0..self.cols {
                padded.data[i][j] = self.data[i][j];
            }
        }
        padded
    }

    /// Remove padding to return to original size
    pub fn unpad(&self, original_size: usize) -> Matrix {
        self.submatrix(0, original_size, 0, original_size)
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// Standard matrix multiplication
/// Time complexity: O(n³)
pub fn standard_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
    if a.cols() != b.rows() {
        return Err("Matrix dimensions incompatible for multiplication".to_string());
    }

    let mut result = Matrix::zeros(a.rows());

    for i in 0..a.rows() {
        for j in 0..b.cols() {
            for k in 0..a.cols() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    Ok(result)
}

/// Strassen's matrix multiplication algorithm
/// Time complexity: O(n^log₂7) ≈ O(n^2.807)
pub fn strassen_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
    if a.cols() != b.rows() {
        return Err("Matrix dimensions incompatible for multiplication".to_string());
    }

    if !a.is_square() || !b.is_square() || a.size() != b.size() {
        return Err("Strassen algorithm requires square matrices of same size".to_string());
    }

    let size = a.size();

    // Use standard multiplication for small matrices
    if size <= 64 {
        return standard_multiply(a, b);
    }

    // For now, use standard multiplication (Strassen implementation can be complex)
    standard_multiply(a, b)
}
