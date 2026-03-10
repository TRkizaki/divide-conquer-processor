# Sorting Algorithms Implementation

A comprehensive Rust implementation of sorting algorithms featuring both sequential and parallel versions of merge sort and quick sort using the Rayon crate for parallel processing.

##  Features

- **Sequential Merge Sort**: Stable O(n log n) divide-and-conquer sorting
- **Sequential Quick Sort**: Average O(n log n) in-place sorting with pivot partitioning
- **Parallel Sorting**: High-performance parallel versions using Rayon for large datasets
- **Comprehensive Testing**: Unit tests for all sorting implementations
- **Memory Efficient**: Optimized memory usage with intelligent thresholds

## Table of Contents

- [Algorithm Overview](#algorithm-overview)
- [Implementation Details](#implementation-details)
- [Performance Characteristics](#performance-characteristics)
- [Usage Examples](#usage-examples)
- [Testing](#testing)
- [Dependencies](#dependencies)

##  Algorithm Overview

### Merge Sort

Merge sort is a **stable, divide-and-conquer** sorting algorithm that guarantees O(n log n) performance in all cases.

**How it works:**

1. **Divide**: Split the array into two halves recursively until single elements remain
2. **Conquer**: Merge the sorted subarrays back together in sorted order
3. **Stability**: Equal elements maintain their relative order

### Quick Sort

Quick sort is an **efficient, in-place** sorting algorithm with average O(n log n) performance.

**How it works:**

1. **Pivot Selection**: Choose the last element as pivot
2. **Partitioning**: Rearrange elements so smaller values are left of pivot, larger values are right
3. **Recursion**: Apply the same process to the left and right subarrays

##  Implementation Details

### Sequential Merge Sort

```rust
pub fn merge_sort(arr: &mut [i32])
```

**Key Implementation Features:**

- **Recursive Strategy**: `merge_sort_recursive()` handles the divide phase
- **Three-way Merge**: The `merge()` function combines two sorted subarrays
- **Boundary Handling**: Careful index management prevents overflow
- **Memory Management**: Creates temporary vectors for merging process

**Merge Process:**

```rust
fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize)
```

- Creates temporary arrays for left and right subarrays
- Uses three pointers (i, j, k) for efficient merging
- Handles remaining elements after one subarray is exhausted

### Sequential Quick Sort

```rust
pub fn quick_sort(arr: &mut [i32])
```

**Key Implementation Features:**

- **In-place Sorting**: No additional memory allocation required
- **Median-of-Three Pivot**: Selects median of first, middle, and last elements to mitigate worst-case O(n^2) on sorted data
- **Insertion Sort Base Case**: Subarrays smaller than 16 elements use insertion sort for efficiency
- **Recursive Divide**: Sorts subarrays independently
- **Index Safety**: Prevents underflow with careful boundary checks

**Partition Process:**

```rust
fn partition_median_of_three(arr: &mut [i32], low: usize, high: usize) -> usize
```

- Median-of-three pivot selection reduces worst-case probability
- Maintains invariant: elements ≤ pivot on left, elements > pivot on right
- Returns pivot’s final position for recursive calls
- Uses efficient swapping for element rearrangement

### Parallel Implementations

Both parallel functions use **true divide-and-conquer parallelism** via `rayon::join()`:

```rust
pub fn parallel_merge_sort(arr: &mut [i32])
pub fn parallel_quick_sort(arr: &mut [i32])
```

**True Parallel D&C Design:**

Unlike a simple wrapper around `par_sort_unstable()`, these implementations preserve the actual algorithm structure while parallelizing the recursive subproblem decomposition through Rayon’s work-stealing thread pool.

```rust
// Parallel recursive calls using rayon::join (work-stealing)
rayon::join(
    || parallel_merge_sort_inner(left_arr, left_buf, threshold),
    || parallel_merge_sort_inner(right_arr, right_buf, threshold),
);
```

**Configurable Thresholds:**

- **`PARALLEL_THRESHOLD = 8192`**: Merge sort crossover point
- **`QUICKSORT_PARALLEL_THRESHOLD = 4096`**: Quick sort crossover point
- Below threshold: sequential execution to avoid thread overhead
- Threshold-tuning functions available for experimentation

**Threshold Experimentation:**

```rust
pub fn parallel_merge_sort_with_threshold(arr: &mut [i32], threshold: usize)
pub fn parallel_quick_sort_with_threshold(arr: &mut [i32], threshold: usize)
pub fn benchmark_merge_sort_threshold(data: &[i32], threshold: usize) -> f64
pub fn benchmark_quick_sort_threshold(data: &[i32], threshold: usize) -> f64
```

## Performance Characteristics

|Algorithm            |Time Complexity                      |Space Complexity|Stability |In-Place |
|---------------------|-------------------------------------|----------------|----------|---------|
|**Merge Sort**       |O(n log n) - all cases               |O(n)            |✅ Stable  |❌ No     |
|**Quick Sort**       |O(n log n) - average<br>O(n²) - worst|O(log n)        |❌ Unstable|✅ Yes    |
|**Parallel Versions**|O(n log n / p)*                      |Varies          |Depends** |Depends**|

*p = number of processor cores  
**Depends on underlying algorithm used by Rayon

### When to Use Each Algorithm

**Use Merge Sort when:**

- Stability is required (maintaining relative order of equal elements)
- Consistent O(n log n) performance is critical
- Working with linked lists or external sorting

**Use Quick Sort when:**

- Memory usage must be minimized (in-place sorting)
- Average-case performance is acceptable
- Data is likely to be randomly distributed

**Use Parallel Versions when:**

- Dataset size > 1000 elements
- Multiple CPU cores are available
- Maximum performance is required

##  Usage Examples

### Basic Usage

```rust
use sorting::{merge_sort, quick_sort, parallel_merge_sort, parallel_quick_sort};

// Sequential sorting
let mut data = vec![64, 34, 25, 12, 22, 11, 90];
merge_sort(&mut data);
println!("{:?}", data); // [11, 12, 22, 25, 34, 64, 90]

// Quick sort
let mut data = vec![64, 34, 25, 12, 22, 11, 90];
quick_sort(&mut data);
println!("{:?}", data); // [11, 12, 22, 25, 34, 64, 90]
```

### Performance Comparison

```rust
use std::time::Instant;

let mut large_dataset = vec![/* 100,000 random integers */];

// Sequential merge sort
let start = Instant::now();
merge_sort(&mut large_dataset.clone());
println!("Sequential merge sort: {:?}", start.elapsed());

// Parallel merge sort
let start = Instant::now();
parallel_merge_sort(&mut large_dataset);
println!("Parallel merge sort: {:?}", start.elapsed());
```

### Benchmarking Different Data Patterns

```rust
// Best case for quick sort (random data)
let mut random_data = generate_random_data(10000);
quick_sort(&mut random_data);

// Worst case for quick sort (already sorted)
let mut sorted_data = (0..10000).collect::<Vec<i32>>();
merge_sort(&mut sorted_data); // Better choice for sorted data
```

##  Testing

The implementation includes comprehensive unit tests:

```rust
#[test]
fn test_merge_sort() {
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    merge_sort(&mut arr);
    assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
}
```

**Test Coverage:**

- ✅ Basic functionality for all algorithms
- ✅ Edge cases (empty arrays, single elements)
- ✅ Parallel vs sequential result consistency
- ✅ Large dataset handling

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_merge_sort

# Run with output
cargo test -- --nocapture
```

## Dependencies

```toml
[dependencies]
rayon = "1.8"
rand = "0.9.1"  # for test data generation
```

**Rayon Features Used:**

- `rayon::join()`: Fork-join parallelism for recursive D&C
- Automatic work-stealing thread pool
- Configurable thread pool via `ThreadPoolBuilder` (used with core affinity)

##  Technical Implementation Notes

### Memory Safety

- **No unsafe code**: All implementations use safe Rust
- **Bounds checking**: Careful index management prevents panics
- **Ownership**: Proper borrowing ensures memory safety

### Edge Case Handling

- **Empty arrays**: Immediate return without processing
- **Single elements**: Base case for recursion
- **Integer overflow**: Uses `left + (right - left) / 2` for safe midpoint calculation

### Optimization Techniques

- **Threshold-based parallelization**: Avoids overhead for small datasets
- **In-place partitioning**: Quick sort minimizes memory allocation
- **Cache-friendly merging**: Sequential memory access patterns in merge sort

## Future Enhancements

Potential improvements for the implementation:

1. **Generic Types**: Support for `T: Ord` instead of just `i32`
2. **Custom Comparators**: Allow custom comparison functions
3. **Memory Pool**: Reuse allocated memory for merge operations

## Related Documentation

- **[library_comparison.md](library_comparison.md)** - Benchmarks against std library and Rayon sorts
- **[thread_affinity.md](thread_affinity.md)** - P-core/E-core affinity experiments for parallel sorts
- **[comprehensive_benchmark.md](comprehensive_benchmark.md)** - Publication-quality benchmarking suite
- **[rayon.md](rayon.md)** - Parallel processing with Rayon

-----

**Note**: The parallel implementations use true divide-and-conquer parallelism via `rayon::join()`, preserving the algorithm structure while leveraging Rayon’s work-stealing thread pool for load balancing.