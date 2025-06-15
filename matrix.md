# Matrix Library

A high-performance matrix computation library implemented in Rust, featuring both standard and optimized matrix multiplication algorithms with comprehensive matrix operations.

## Features

- **Flexible Matrix Creation**: Create matrices with custom initialization functions, from 2D vectors, or use predefined patterns (zeros, identity)
- **Comprehensive Operations**: Element-wise addition, subtraction, and matrix multiplication
- **Multiple Multiplication Algorithms**: Standard O(n³) and Strassen’s O(n^2.807) algorithms
- **Matrix Utilities**: Submatrix extraction, padding, dimension queries
- **Memory Safe**: Built with Rust’s ownership system for guaranteed memory safety
- **Serialization Support**: JSON serialization/deserialization via Serde

## Quick Start

```rust
use matrix_library::Matrix;

// Create a 3x3 identity matrix
let identity = Matrix::identity(3);

// Create a matrix from a 2D vector
let data = vec![
    vec![1.0, 2.0, 3.0],
    vec![4.0, 5.0, 6.0],
    vec![7.0, 8.0, 9.0],
];
let matrix = Matrix::from_vec(data);

// Matrix multiplication
let result = standard_multiply(&identity, &matrix)?;
```

## API Reference

### Matrix Creation

#### `Matrix::new<F>(size: usize, init_fn: F) -> Self`

Creates a square matrix with custom initialization function.

```rust
// Create a matrix where each element equals i + j
let matrix = Matrix::new(3, |i, j| (i + j) as f64);
```

#### `Matrix::from_vec(data: Vec<Vec<f64>>) -> Self`

Creates a matrix from a 2D vector.

```rust
let matrix = Matrix::from_vec(vec![
    vec![1.0, 2.0],
    vec![3.0, 4.0],
]);
```

#### `Matrix::zeros(size: usize) -> Self`

Creates a square matrix filled with zeros.

```rust
let zeros = Matrix::zeros(5);
```

#### `Matrix::identity(size: usize) -> Self`

Creates an identity matrix (diagonal elements = 1, others = 0).

```rust
let identity = Matrix::identity(4);
```

### Matrix Properties

#### `size() -> usize`

Returns the size of a square matrix (assumes square matrix).

#### `rows() -> usize` / `cols() -> usize`

Returns the number of rows or columns.

#### `is_square() -> bool`

Checks if the matrix is square (rows == cols).

### Element Access

#### `get(i: usize, j: usize) -> f64`

Retrieves the element at position (i, j).

#### `set(&mut self, i: usize, j: usize, value: f64)`

Sets the element at position (i, j).

#### Index Operators

Direct access via indexing syntax:

```rust
let value = matrix[i][j];  // Read
matrix[i][j] = 5.0;        // Write
```

### Matrix Operations

#### `add(&self, other: &Matrix) -> Result<Matrix, String>`

Element-wise matrix addition.

```rust
let result = matrix_a.add(&matrix_b)?;
```

#### `subtract(&self, other: &Matrix) -> Result<Matrix, String>`

Element-wise matrix subtraction.

```rust
let result = matrix_a.subtract(&matrix_b)?;
```

#### `submatrix(start_row: usize, end_row: usize, start_col: usize, end_col: usize) -> Matrix`

Extracts a submatrix from the specified range.

```rust
let sub = matrix.submatrix(1, 3, 1, 3); // Extract 2x2 submatrix
```

### Utility Functions

#### `pad_to_power_of_2(&self) -> Matrix`

Pads the matrix with zeros to the next power-of-2 size (useful for certain algorithms).

#### `unpad(&self, original_size: usize) -> Matrix`

Removes padding to return to the original size.

## Matrix Multiplication Algorithms

### Standard Matrix Multiplication

#### `standard_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String>`

Classic matrix multiplication algorithm with O(n³) time complexity.

- **Time Complexity**: O(n³)
- **Space Complexity**: O(n²)
- **Best for**: General-purpose matrix multiplication, all matrix sizes

```rust
let result = standard_multiply(&matrix_a, &matrix_b)?;
```

### Strassen’s Algorithm

#### `strassen_multiply(a: &Matrix, b: &Matrix) -> Result<Matrix, String>`

Advanced divide-and-conquer algorithm with improved asymptotic complexity.

- **Time Complexity**: O(n^log₂7) ≈ O(n^2.807)
- **Space Complexity**: O(n²)
- **Requirements**: Square matrices of the same size
- **Best for**: Large matrices (theoretical improvement)
- **Current Implementation**: Falls back to standard multiplication for matrices ≤ 64x64

```rust
let result = strassen_multiply(&matrix_a, &matrix_b)?;
```

**Note**: The current implementation uses standard multiplication as a base case. A full Strassen implementation would recursively apply the seven-multiplication formula for larger matrices.

## Error Handling

All fallible operations return `Result<Matrix, String>` with descriptive error messages:

- **Dimension Mismatch**: When matrix dimensions are incompatible for the operation
- **Invalid Arguments**: When requirements for specific algorithms aren’t met

```rust
match matrix_a.add(&matrix_b) {
    Ok(result) => println!("Addition successful"),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Serialization

The library supports JSON serialization via Serde:

```rust
use serde_json;

let matrix = Matrix::identity(3);
let json = serde_json::to_string(&matrix)?;
let restored: Matrix = serde_json::from_str(&json)?;
```

## Performance Considerations

- **Small Matrices**: Standard multiplication is often faster due to lower overhead
- **Large Matrices**: Strassen’s algorithm provides theoretical improvements but requires careful implementation
- **Memory**: All matrices store data as `Vec<Vec<f64>>` for flexibility
- **Cache Performance**: Row-major storage order optimizes for typical access patterns

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

## Examples

### Basic Matrix Operations

```rust
use matrix_library::*;

fn main() -> Result<(), String> {
    // Create matrices
    let a = Matrix::from_vec(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    
    let b = Matrix::from_vec(vec![
        vec![5.0, 6.0],
        vec![7.0, 8.0],
    ]);
    
    // Addition
    let sum = a.add(&b)?;
    println!("Sum: {:?}", sum);
    
    // Multiplication
    let product = standard_multiply(&a, &b)?;
    println!("Product: {:?}", product);
    
    Ok(())
}
```

### Working with Submatrices

```rust
let large_matrix = Matrix::new(5, |i, j| (i * 5 + j) as f64);
let corner = large_matrix.submatrix(0, 2, 0, 2);
println!("Top-left 2x2 corner: {:?}", corner);
```

## Future Enhancements

- Full Strassen algorithm implementation with recursive decomposition
- Additional algorithms (Winograd, Coppersmith-Winograd)
- SIMD optimizations for standard multiplication
- Sparse matrix support
- Parallel computation support
- More comprehensive error types

## License

This library is provided as-is for educational and research purposes.