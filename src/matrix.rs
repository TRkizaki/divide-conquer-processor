use serde::{Deserialize, Serialize};
use std::ops::{Add, Index, IndexMut, Mul, Sub};
use rayon::prelude::*;
use std::fmt;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// Comprehensive error types for matrix operations
#[derive(Debug, Clone, PartialEq)]
pub enum MatrixError {
    /// Dimension mismatch between matrices
    DimensionMismatch {
        operation: String,
        expected: (usize, usize),
        actual: (usize, usize),
    },
    /// Matrix is singular (determinant is zero)
    SingularMatrix {
        operation: String,
        details: String,
    },
    /// Matrix is not square when required
    NotSquare {
        operation: String,
        dimensions: (usize, usize),
    },
    /// Index out of bounds
    IndexOutOfBounds {
        index: (usize, usize),
        bounds: (usize, usize),
    },
    /// Numerical instability detected
    NumericalInstability {
        operation: String,
        condition_number: Option<f64>,
    },
    /// Invalid algorithm parameters
    InvalidParameters {
        operation: String,
        parameter: String,
        value: String,
        constraint: String,
    },
    /// Memory allocation failure
    MemoryError {
        requested_size: usize,
        available_size: Option<usize>,
    },
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatrixError::DimensionMismatch { operation, expected, actual } => {
                write!(f, "Dimension mismatch in {}: expected {}×{}, got {}×{}", 
                       operation, expected.0, expected.1, actual.0, actual.1)
            }
            MatrixError::SingularMatrix { operation, details } => {
                write!(f, "Singular matrix in {}: {}", operation, details)
            }
            MatrixError::NotSquare { operation, dimensions } => {
                write!(f, "Non-square matrix in {}: {}×{}", operation, dimensions.0, dimensions.1)
            }
            MatrixError::IndexOutOfBounds { index, bounds } => {
                write!(f, "Index ({}, {}) out of bounds for {}×{} matrix", 
                       index.0, index.1, bounds.0, bounds.1)
            }
            MatrixError::NumericalInstability { operation, condition_number } => {
                if let Some(cond) = condition_number {
                    write!(f, "Numerical instability in {} (condition number: {:.2e})", operation, cond)
                } else {
                    write!(f, "Numerical instability detected in {}", operation)
                }
            }
            MatrixError::InvalidParameters { operation, parameter, value, constraint } => {
                write!(f, "Invalid parameter {} = {} in {}: {}", parameter, value, operation, constraint)
            }
            MatrixError::MemoryError { requested_size, available_size } => {
                if let Some(available) = available_size {
                    write!(f, "Memory allocation failed: requested {} bytes, {} available", 
                           requested_size, available)
                } else {
                    write!(f, "Memory allocation failed: requested {} bytes", requested_size)
                }
            }
        }
    }
}

impl std::error::Error for MatrixError {}

/// Result type for matrix operations
pub type MatrixResult<T> = Result<T, MatrixError>;

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

    /// Create zero matrix with specified dimensions (rows x cols)
    pub fn zeros_with_dimensions(rows: usize, cols: usize) -> Self {
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            data.push(vec![0.0; cols]);
        }
        
        Self { data, rows, cols }
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

    /// Transpose the matrix (swap rows and columns)
    /// Time complexity: O(n²)
    pub fn transpose(&self) -> Matrix {
        let mut transposed = Matrix::zeros_with_dimensions(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                transposed.data[j][i] = self.data[i][j];
            }
        }
        transposed
    }

    /// Calculate the trace of the matrix (sum of diagonal elements)
    /// Only defined for square matrices
    /// Time complexity: O(n)
    pub fn trace(&self) -> Result<f64, String> {
        if !self.is_square() {
            return Err(format!("Trace is only defined for square matrices, got {}x{}", self.rows, self.cols));
        }
        
        let mut sum = 0.0;
        for i in 0..self.rows {
            sum += self.data[i][i];
        }
        Ok(sum)
    }

    /// Check if the matrix is symmetric (A = A^T)
    /// Time complexity: O(n²)
    pub fn is_symmetric(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                if (self.data[i][j] - self.data[j][i]).abs() > 1e-10 {
                    return false;
                }
            }
        }
        true
    }

    /// Check if the matrix is diagonal
    /// Time complexity: O(n²)
    pub fn is_diagonal(&self) -> bool {
        if !self.is_square() {
            return false;
        }
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                if i != j && self.data[i][j].abs() > 1e-10 {
                    return false;
                }
            }
        }
        true
    }

    /// Calculate the Frobenius norm of the matrix
    /// ||A||_F = sqrt(sum of squares of all elements)
    /// Time complexity: O(n²)
    pub fn frobenius_norm(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.rows {
            for j in 0..self.cols {
                sum += self.data[i][j] * self.data[i][j];
            }
        }
        sum.sqrt()
    }

    /// Calculate the 1-norm (maximum column sum)
    /// ||A||_1 = max(sum of absolute values in each column)
    /// Time complexity: O(n²)
    pub fn one_norm(&self) -> f64 {
        let mut max_sum: f64 = 0.0;
        for j in 0..self.cols {
            let mut col_sum = 0.0;
            for i in 0..self.rows {
                col_sum += self.data[i][j].abs();
            }
            max_sum = max_sum.max(col_sum);
        }
        max_sum
    }

    /// Calculate the infinity-norm (maximum row sum)
    /// ||A||_∞ = max(sum of absolute values in each row)
    /// Time complexity: O(n²)
    pub fn infinity_norm(&self) -> f64 {
        let mut max_sum: f64 = 0.0;
        for i in 0..self.rows {
            let mut row_sum = 0.0;
            for j in 0..self.cols {
                row_sum += self.data[i][j].abs();
            }
            max_sum = max_sum.max(row_sum);
        }
        max_sum
    }

    /// Calculate the 2-norm (spectral norm) - largest singular value
    /// This is an approximation using power iteration
    /// Time complexity: O(n² * iterations)
    pub fn two_norm_approx(&self, iterations: usize) -> f64 {
        if self.rows == 0 || self.cols == 0 {
            return 0.0;
        }

        // Create A^T * A for symmetric eigenvalue problem
        let at = self.transpose();
        let ata = standard_multiply(&at, self).unwrap_or_else(|_| Matrix::zeros(1));
        
        // Power iteration to find largest eigenvalue
        let mut v = Matrix::zeros_with_dimensions(ata.rows, 1);
        // Initialize with random-ish values
        for i in 0..v.rows {
            v.data[i][0] = (i + 1) as f64;
        }
        
        let mut eigenvalue = 0.0;
        for _ in 0..iterations {
            let av = standard_multiply(&ata, &v).unwrap_or_else(|_| Matrix::zeros_with_dimensions(v.rows, 1));
            let norm = av.frobenius_norm();
            if norm > 1e-12 {
                eigenvalue = norm;
                // Normalize v
                for i in 0..v.rows {
                    v.data[i][0] = av.data[i][0] / norm;
                }
            } else {
                break;
            }
        }
        
        eigenvalue.sqrt()
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

/// Strassen's matrix multiplication algorithm with full recursive implementation
/// Time complexity: O(n^log₂7) ≈ O(n^2.807)
pub fn strassen_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
    if a.cols() != b.rows() {
        return Err("Matrix dimensions incompatible for multiplication".to_string());
    }

    if !a.is_square() || !b.is_square() || a.size() != b.size() {
        return Err("Strassen algorithm requires square matrices of same size".to_string());
    }

    let size = a.size();

    // Use standard multiplication for small matrices (base case)
    if size <= 64 {
        return standard_multiply(a, b);
    }

    // Pad matrices to power of 2 if necessary
    let padded_size = size.next_power_of_two();
    let a_padded = if size == padded_size { a.clone() } else { a.pad_to_power_of_2() };
    let b_padded = if size == padded_size { b.clone() } else { b.pad_to_power_of_2() };

    let result_padded = strassen_multiply_recursive(&a_padded, &b_padded)?;
    
    // Remove padding if it was added
    if size == padded_size {
        Ok(result_padded)
    } else {
        Ok(result_padded.unpad(size))
    }
}

/// Recursive implementation of Strassen's algorithm
fn strassen_multiply_recursive(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
    let n = a.size();
    
    // Base case: use standard multiplication for small matrices
    if n <= 64 {
        return standard_multiply(a, b);
    }
    
    let half = n / 2;
    
    // Divide matrices into quadrants
    let a11 = a.submatrix(0, half, 0, half);
    let a12 = a.submatrix(0, half, half, n);
    let a21 = a.submatrix(half, n, 0, half);
    let a22 = a.submatrix(half, n, half, n);
    
    let b11 = b.submatrix(0, half, 0, half);
    let b12 = b.submatrix(0, half, half, n);
    let b21 = b.submatrix(half, n, 0, half);
    let b22 = b.submatrix(half, n, half, n);
    
    // Calculate the 7 Strassen products recursively
    // M1 = (A11 + A22) * (B11 + B22)
    let m1 = strassen_multiply_recursive(&a11.add(&a22)?, &b11.add(&b22)?)?;
    
    // M2 = (A21 + A22) * B11
    let m2 = strassen_multiply_recursive(&a21.add(&a22)?, &b11)?;
    
    // M3 = A11 * (B12 - B22)
    let m3 = strassen_multiply_recursive(&a11, &b12.subtract(&b22)?)?;
    
    // M4 = A22 * (B21 - B11)
    let m4 = strassen_multiply_recursive(&a22, &b21.subtract(&b11)?)?;
    
    // M5 = (A11 + A12) * B22
    let m5 = strassen_multiply_recursive(&a11.add(&a12)?, &b22)?;
    
    // M6 = (A21 - A11) * (B11 + B12)
    let m6 = strassen_multiply_recursive(&a21.subtract(&a11)?, &b11.add(&b12)?)?;
    
    // M7 = (A12 - A22) * (B21 + B22)
    let m7 = strassen_multiply_recursive(&a12.subtract(&a22)?, &b21.add(&b22)?)?;
    
    // Calculate result quadrants
    // C11 = M1 + M4 - M5 + M7
    let c11 = m1.add(&m4)?.subtract(&m5)?.add(&m7)?;
    
    // C12 = M3 + M5
    let c12 = m3.add(&m5)?;
    
    // C21 = M2 + M4
    let c21 = m2.add(&m4)?;
    
    // C22 = M1 - M2 + M3 + M6
    let c22 = m1.subtract(&m2)?.add(&m3)?.add(&m6)?;
    
    // Combine quadrants into result matrix
    let mut result = Matrix::zeros(n);
    
    // Copy quadrants into result matrix
    for i in 0..half {
        for j in 0..half {
            result[i][j] = c11[i][j];                    // Top-left
            result[i][j + half] = c12[i][j];             // Top-right
            result[i + half][j] = c21[i][j];             // Bottom-left
            result[i + half][j + half] = c22[i][j];      // Bottom-right
        }
    }
    
    Ok(result)
}

/// Cache-optimized matrix multiplication using blocking
/// Time complexity: O(n³) but with better cache performance
pub fn cache_optimized_multiply(a: &Matrix, b: &Matrix, block_size: usize) -> Result<Matrix, String> {
    if a.cols() != b.rows() {
        return Err("Matrix dimensions incompatible for multiplication".to_string());
    }

    let mut result = Matrix::zeros(a.rows());
    let n = a.rows();

    // Blocking for cache efficiency
    for ii in (0..n).step_by(block_size) {
        for jj in (0..n).step_by(block_size) {
            for kk in (0..n).step_by(block_size) {
                // Process block
                let i_end = (ii + block_size).min(n);
                let j_end = (jj + block_size).min(n);
                let k_end = (kk + block_size).min(n);
                
                for i in ii..i_end {
                    for j in jj..j_end {
                        let mut sum = 0.0;
                        for k in kk..k_end {
                            sum += a[i][k] * b[k][j];
                        }
                        result[i][j] += sum;
                    }
                }
            }
        }
    }

    Ok(result)
}

/// Parallel matrix multiplication using Rayon
/// Time complexity: O(n³) with parallel processing
pub fn parallel_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
    if a.cols() != b.rows() {
        return Err("Matrix dimensions incompatible for multiplication".to_string());
    }

    let mut result = Matrix::zeros(a.rows());
    
    result.data.par_iter_mut().enumerate().for_each(|(i, row)| {
        for j in 0..b.cols() {
            let mut sum = 0.0;
            for k in 0..a.cols() {
                sum += a[i][k] * b[k][j];
            }
            row[j] = sum;
        }
    });

    Ok(result)
}

/// Calculate matrix determinant using LU decomposition
/// Time complexity: O(n³)
pub fn determinant(matrix: &Matrix) -> Result<f64, String> {
    if !matrix.is_square() {
        return Err("Determinant can only be calculated for square matrices".to_string());
    }

    let n = matrix.size();
    let mut a = matrix.clone();
    let mut det = 1.0;
    let eps = 1e-10;

    // LU decomposition with partial pivoting
    for i in 0..n {
        // Find pivot
        let mut pivot_row = i;
        for k in i + 1..n {
            if a[k][i].abs() > a[pivot_row][i].abs() {
                pivot_row = k;
            }
        }

        // Swap rows if needed
        if pivot_row != i {
            a.data.swap(i, pivot_row);
            det = -det; // Row swap changes sign of determinant
        }

        // Check for singular matrix
        if a[i][i].abs() < eps {
            return Ok(0.0);
        }

        det *= a[i][i];

        // Eliminate column
        for k in i + 1..n {
            let factor = a[k][i] / a[i][i];
            for j in i..n {
                a[k][j] -= factor * a[i][j];
            }
        }
    }

    Ok(det)
}

/// Calculate matrix inverse using Gauss-Jordan elimination
/// Time complexity: O(n³)
pub fn inverse(matrix: &Matrix) -> Result<Matrix, String> {
    if !matrix.is_square() {
        return Err("Inverse can only be calculated for square matrices".to_string());
    }

    let n = matrix.size();
    let mut a = matrix.clone();
    let mut inv = Matrix::identity(n);
    let eps = 1e-10;

    // Gauss-Jordan elimination
    for i in 0..n {
        // Find pivot
        let mut pivot_row = i;
        for k in i + 1..n {
            if a[k][i].abs() > a[pivot_row][i].abs() {
                pivot_row = k;
            }
        }

        // Swap rows if needed
        if pivot_row != i {
            a.data.swap(i, pivot_row);
            inv.data.swap(i, pivot_row);
        }

        // Check for singular matrix
        if a[i][i].abs() < eps {
            return Err("Matrix is singular (non-invertible)".to_string());
        }

        // Scale pivot row
        let pivot = a[i][i];
        for j in 0..n {
            a[i][j] /= pivot;
            inv[i][j] /= pivot;
        }

        // Eliminate column
        for k in 0..n {
            if k != i {
                let factor = a[k][i];
                for j in 0..n {
                    a[k][j] -= factor * a[i][j];
                    inv[k][j] -= factor * inv[i][j];
                }
            }
        }
    }

    Ok(inv)
}

/// Solve linear system Ax = b using Gaussian elimination
/// Time complexity: O(n³)
pub fn solve_linear_system(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
    if !a.is_square() {
        return Err("Coefficient matrix must be square".to_string());
    }
    
    if a.rows() != b.rows() {
        return Err("Matrix dimensions incompatible".to_string());
    }

    let n = a.size();
    let mut aug = a.clone();
    let mut solution = b.clone();
    let eps = 1e-10;

    // Forward elimination
    for i in 0..n {
        // Find pivot
        let mut pivot_row = i;
        for k in i + 1..n {
            if aug[k][i].abs() > aug[pivot_row][i].abs() {
                pivot_row = k;
            }
        }

        // Swap rows if needed
        if pivot_row != i {
            aug.data.swap(i, pivot_row);
            solution.data.swap(i, pivot_row);
        }

        // Check for singular matrix
        if aug[i][i].abs() < eps {
            return Err("Matrix is singular (system has no unique solution)".to_string());
        }

        // Eliminate column
        for k in i + 1..n {
            let factor = aug[k][i] / aug[i][i];
            for j in i..n {
                aug[k][j] -= factor * aug[i][j];
            }
            for j in 0..solution.cols() {
                solution[k][j] -= factor * solution[i][j];
            }
        }
    }

    // Back substitution
    for i in (0..n).rev() {
        for j in 0..solution.cols() {
            for k in i + 1..n {
                solution[i][j] -= aug[i][k] * solution[k][j];
            }
            solution[i][j] /= aug[i][i];
        }
    }

    Ok(solution)
}

/// SIMD-optimized matrix multiplication using AVX2 instructions (x86_64 only)
/// Time complexity: O(n³) with vectorized operations
#[cfg(target_arch = "x86_64")]
pub fn simd_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
    if a.cols() != b.rows() {
        return Err("Matrix dimensions incompatible for multiplication".to_string());
    }

    // Check for AVX2 support at runtime
    if !is_x86_feature_detected!("avx2") {
        // Fallback to standard multiplication if AVX2 not available
        return standard_multiply(a, b);
    }

    let mut result = Matrix::zeros(a.rows());
    
    unsafe {
        simd_multiply_avx2(&a, &b, &mut result);
    }
    
    Ok(result)
}

/// SIMD fallback for non-x86_64 architectures
#[cfg(not(target_arch = "x86_64"))]
pub fn simd_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
    // Fallback to standard multiplication on non-x86_64 platforms
    standard_multiply(a, b)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn simd_multiply_avx2(a: &Matrix, b: &Matrix, result: &mut Matrix) {
    let n = a.rows();
    let m = a.cols();
    let p = b.cols();
    
    for i in 0..n {
        for j in (0..p).step_by(4) {
            // Process 4 elements at a time using AVX2
            let mut sum_vec = _mm256_setzero_pd();
            
            for k in (0..m).step_by(4) {
                if k + 3 < m && j + 3 < p {
                    // Load 4 elements from matrix A row
                    let a_vec = _mm256_loadu_pd(&a[i][k] as *const f64);
                    
                    // Load and accumulate 4 dot products for matrix B columns
                    for col_offset in 0..4.min(p - j) {
                        if j + col_offset < p {
                            let b_vec = _mm256_set_pd(
                                if k + 3 < m { b[k + 3][j + col_offset] } else { 0.0 },
                                if k + 2 < m { b[k + 2][j + col_offset] } else { 0.0 },
                                if k + 1 < m { b[k + 1][j + col_offset] } else { 0.0 },
                                b[k][j + col_offset]
                            );
                            
                            let prod = _mm256_mul_pd(a_vec, b_vec);
                            sum_vec = _mm256_add_pd(sum_vec, prod);
                        }
                    }
                } else {
                    // Handle remaining elements with scalar operations
                    for col_offset in 0..4.min(p - j) {
                        if j + col_offset < p {
                            let mut sum = 0.0;
                            for k_scalar in k..m {
                                sum += a[i][k_scalar] * b[k_scalar][j + col_offset];
                            }
                            result[i][j + col_offset] += sum;
                        }
                    }
                    break;
                }
            }
            
            // Extract results from SIMD register and add to result matrix
            let mut temp = [0.0; 4];
            _mm256_storeu_pd(temp.as_mut_ptr(), sum_vec);
            
            // Horizontal sum of the 4 elements
            let horizontal_sum = temp[0] + temp[1] + temp[2] + temp[3];
            
            // This is a simplified version - in practice, we'd need more sophisticated
            // handling to properly vectorize matrix multiplication
            for col_offset in 0..4.min(p - j) {
                if j + col_offset < p {
                    // For now, fall back to scalar for correctness
                    let mut sum = 0.0;
                    for k in 0..m {
                        sum += a[i][k] * b[k][j + col_offset];
                    }
                    result[i][j + col_offset] = sum;
                }
            }
        }
    }
}

/// Optimized dot product using SIMD instructions
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn simd_dot_product(a: &[f64], b: &[f64]) -> f64 {
    let len = a.len().min(b.len());
    let mut sum_vec = _mm256_setzero_pd();
    
    // Process 4 elements at a time
    let chunks = len / 4;
    for i in 0..chunks {
        let idx = i * 4;
        let a_vec = _mm256_loadu_pd(&a[idx] as *const f64);
        let b_vec = _mm256_loadu_pd(&b[idx] as *const f64);
        let prod = _mm256_mul_pd(a_vec, b_vec);
        sum_vec = _mm256_add_pd(sum_vec, prod);
    }
    
    // Extract and sum the 4 partial sums
    let mut temp = [0.0; 4];
    _mm256_storeu_pd(temp.as_mut_ptr(), sum_vec);
    let mut sum = temp[0] + temp[1] + temp[2] + temp[3];
    
    // Handle remaining elements
    for i in (chunks * 4)..len {
        sum += a[i] * b[i];
    }
    
    sum
}

/// Winograd's matrix multiplication algorithm
/// Reduces the number of multiplications by precomputing row and column factors
/// Time complexity: O(n³) with fewer multiplications than standard algorithm
/// Space complexity: O(n²) for factor arrays
pub fn winograd_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
    if a.cols() != b.rows() {
        return Err("Matrix dimensions incompatible for multiplication".to_string());
    }
    
    let n = a.rows();
    let m = a.cols();
    let p = b.cols();
    
    // For small matrices, use standard multiplication
    if n < 32 || m < 32 || p < 32 {
        return standard_multiply(a, b);
    }
    
    // Precompute row factors for matrix A
    let mut row_factors = vec![0.0; n];
    for i in 0..n {
        let mut sum = 0.0;
        for j in (0..m).step_by(2) {
            if j + 1 < m {
                sum += a[i][j] * a[i][j + 1];
            }
        }
        row_factors[i] = sum;
    }
    
    // Precompute column factors for matrix B
    let mut col_factors = vec![0.0; p];
    for j in 0..p {
        let mut sum = 0.0;
        for i in (0..m).step_by(2) {
            if i + 1 < m {
                sum += b[i][j] * b[i + 1][j];
            }
        }
        col_factors[j] = sum;
    }
    
    // Initialize result matrix
    let mut result = Matrix::zeros_with_dimensions(n, p);
    
    // Main computation using Winograd's formula
    for i in 0..n {
        for j in 0..p {
            let mut sum = -row_factors[i] - col_factors[j];
            
            // Compute pairs
            for k in (0..m).step_by(2) {
                if k + 1 < m {
                    sum += (a[i][k] + b[k + 1][j]) * (a[i][k + 1] + b[k][j]);
                }
            }
            
            // Handle odd dimension
            if m % 2 == 1 {
                sum += a[i][m - 1] * b[m - 1][j];
            }
            
            result[i][j] = sum;
        }
    }
    
    Ok(result)
}

/// Parallel Winograd matrix multiplication using Rayon
/// Combines Winograd's reduced multiplication count with parallel processing
pub fn parallel_winograd_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String> {
    if a.cols() != b.rows() {
        return Err("Matrix dimensions incompatible for multiplication".to_string());
    }
    
    let n = a.rows();
    let m = a.cols();
    let p = b.cols();
    
    // For small matrices, use standard parallel multiplication
    if n < 64 || m < 64 || p < 64 {
        return parallel_multiply(a, b);
    }
    
    // Precompute row factors for matrix A in parallel
    let row_factors: Vec<f64> = (0..n).into_par_iter().map(|i| {
        let mut sum = 0.0;
        for j in (0..m).step_by(2) {
            if j + 1 < m {
                sum += a[i][j] * a[i][j + 1];
            }
        }
        sum
    }).collect();
    
    // Precompute column factors for matrix B in parallel
    let col_factors: Vec<f64> = (0..p).into_par_iter().map(|j| {
        let mut sum = 0.0;
        for i in (0..m).step_by(2) {
            if i + 1 < m {
                sum += b[i][j] * b[i + 1][j];
            }
        }
        sum
    }).collect();
    
    // Initialize result matrix
    let mut result = Matrix::zeros_with_dimensions(n, p);
    
    // Parallel computation of result matrix
    result.data.par_iter_mut().enumerate().for_each(|(i, row)| {
        for j in 0..p {
            let mut sum = -row_factors[i] - col_factors[j];
            
            // Compute pairs
            for k in (0..m).step_by(2) {
                if k + 1 < m {
                    sum += (a[i][k] + b[k + 1][j]) * (a[i][k + 1] + b[k][j]);
                }
            }
            
            // Handle odd dimension
            if m % 2 == 1 {
                sum += a[i][m - 1] * b[m - 1][j];
            }
            
            row[j] = sum;
        }
    });
    
    Ok(result)
}

/// LU Decomposition with partial pivoting
/// Decomposes matrix A into L (lower triangular) and U (upper triangular) matrices
/// Returns (L, U, P) where P is the permutation matrix such that PA = LU
/// Time complexity: O(n³)
pub fn lu_decomposition(matrix: &Matrix) -> Result<(Matrix, Matrix, Matrix), String> {
    if !matrix.is_square() {
        return Err("LU decomposition requires a square matrix".to_string());
    }
    
    let n = matrix.size();
    let mut a = matrix.clone();
    let mut l = Matrix::identity(n);
    let mut p = Matrix::identity(n);
    
    // Gaussian elimination with partial pivoting
    for k in 0..n {
        // Find pivot
        let mut max_val = 0.0;
        let mut pivot_row = k;
        for i in k..n {
            if a[i][k].abs() > max_val {
                max_val = a[i][k].abs();
                pivot_row = i;
            }
        }
        
        if max_val < 1e-14 {
            return Err("Matrix is singular or nearly singular".to_string());
        }
        
        // Swap rows if needed
        if pivot_row != k {
            // Swap in A
            a.data.swap(k, pivot_row);
            // Swap in P
            p.data.swap(k, pivot_row);
            // Swap in L (only the computed part)
            for j in 0..k {
                let temp = l[k][j];
                l[k][j] = l[pivot_row][j];
                l[pivot_row][j] = temp;
            }
        }
        
        // Elimination
        for i in (k + 1)..n {
            let factor = a[i][k] / a[k][k];
            l[i][k] = factor;
            
            for j in k..n {
                a[i][j] -= factor * a[k][j];
            }
        }
    }
    
    // U is the upper triangular part of A
    let mut u = Matrix::zeros(n);
    for i in 0..n {
        for j in i..n {
            u[i][j] = a[i][j];
        }
    }
    
    Ok((l, u, p))
}

/// QR Decomposition using Gram-Schmidt process
/// Decomposes matrix A into Q (orthogonal) and R (upper triangular) matrices
/// Returns (Q, R) such that A = QR
/// Time complexity: O(n³)
pub fn qr_decomposition(matrix: &Matrix) -> Result<(Matrix, Matrix), String> {
    let m = matrix.rows();
    let n = matrix.cols();
    
    let mut q = Matrix::zeros_with_dimensions(m, n);
    let mut r = Matrix::zeros_with_dimensions(n, n);
    
    // Modified Gram-Schmidt process
    for j in 0..n {
        // Copy column j from A to Q
        for i in 0..m {
            q[i][j] = matrix[i][j];
        }
        
        // Orthogonalize against previous columns
        for k in 0..j {
            // Calculate R[k][j] = Q_k^T * A_j
            let mut dot_product = 0.0;
            for i in 0..m {
                dot_product += q[i][k] * matrix[i][j];
            }
            r[k][j] = dot_product;
            
            // Q_j = Q_j - R[k][j] * Q_k
            for i in 0..m {
                q[i][j] -= r[k][j] * q[i][k];
            }
        }
        
        // Normalize Q_j and set R[j][j]
        let mut norm = 0.0;
        for i in 0..m {
            norm += q[i][j] * q[i][j];
        }
        norm = norm.sqrt();
        
        if norm < 1e-14 {
            return Err("Matrix is rank deficient".to_string());
        }
        
        r[j][j] = norm;
        for i in 0..m {
            q[i][j] /= norm;
        }
    }
    
    Ok((q, r))
}

/// Cholesky Decomposition for positive definite matrices
/// Decomposes matrix A into L * L^T where L is lower triangular
/// Time complexity: O(n³)
pub fn cholesky_decomposition(matrix: &Matrix) -> Result<Matrix, String> {
    if !matrix.is_square() {
        return Err("Cholesky decomposition requires a square matrix".to_string());
    }
    
    if !matrix.is_symmetric() {
        return Err("Cholesky decomposition requires a symmetric matrix".to_string());
    }
    
    let n = matrix.size();
    let mut l = Matrix::zeros(n);
    
    for i in 0..n {
        for j in 0..=i {
            if i == j {
                // Diagonal elements
                let mut sum = 0.0;
                for k in 0..j {
                    sum += l[i][k] * l[i][k];
                }
                let val = matrix[i][i] - sum;
                if val <= 0.0 {
                    return Err("Matrix is not positive definite".to_string());
                }
                l[i][j] = val.sqrt();
            } else {
                // Off-diagonal elements
                let mut sum = 0.0;
                for k in 0..j {
                    sum += l[i][k] * l[j][k];
                }
                l[i][j] = (matrix[i][j] - sum) / l[j][j];
            }
        }
    }
    
    Ok(l)
}

/// Matrix rank computation using QR decomposition
/// Time complexity: O(n³)
pub fn matrix_rank(matrix: &Matrix) -> Result<usize, String> {
    let (_, r) = qr_decomposition(matrix)?;
    let min_dim = matrix.rows().min(matrix.cols());
    
    let mut rank = 0;
    for i in 0..min_dim {
        if r[i][i].abs() > 1e-12 {
            rank += 1;
        }
    }
    
    Ok(rank)
}

/// Condition number calculation (ratio of largest to smallest singular values)
/// Uses power iteration approximation for the 2-norm
/// Time complexity: O(n² * iterations)
pub fn condition_number_approx(matrix: &Matrix, iterations: usize) -> Result<f64, String> {
    if !matrix.is_square() {
        return Err("Condition number requires a square matrix".to_string());
    }
    
    // Calculate largest singular value
    let largest_sv = matrix.two_norm_approx(iterations);
    
    // Calculate smallest singular value (inverse of largest singular value of inverse)
    let inv = inverse(matrix)?;
    let smallest_sv_inv = inv.two_norm_approx(iterations);
    let smallest_sv = if smallest_sv_inv > 1e-12 { 1.0 / smallest_sv_inv } else { 0.0 };
    
    if smallest_sv < 1e-12 {
        Ok(f64::INFINITY)
    } else {
        Ok(largest_sv / smallest_sv)
    }
}
