**Rayon** is Rust’s **data parallelism library** and an incredibly powerful tool.

##  **Core Features**

### **1. Parallel Iterators**

```rust
use rayon::prelude::*;

// Sequential processing
let sum: i32 = (0..1000000).map(|x| x * x).sum();

// Parallel processing (automatically distributed across multiple CPU cores)
let sum: i32 = (0..1000000).into_par_iter().map(|x| x * x).sum();
```

### **2. Parallel Sorting**

```rust
// Regular sorting
arr.sort();

// Parallel sorting (dramatically faster for large datasets)
arr.par_sort();
```

### **3. Parallel Fork-Join Processing**

```rust
use rayon;

// Execute two tasks in parallel
let (result1, result2) = rayon::join(
    || expensive_computation_1(),
    || expensive_computation_2()
);
```

##  **The Power of Rayon**

### **A. Automatic Parallelization**

- **Automatically detects CPU core count** for optimal distribution
- **Work-stealing algorithm** for load balancing
- **Automated thread pool management**

### **B. Safety**

- **Prevents data races** through the type system
- **Guarantees memory safety**
- **Avoids deadlocks**

### **C. Performance**

```rust
// Example: Processing 1 million elements

// Sequential: 500ms
vec.iter().map(|x| heavy_computation(x)).collect();

// Parallel (8 cores): 65ms (approximately 8x faster)
vec.par_iter().map(|x| heavy_computation(x)).collect();
```

## **Application in Our Project**

### **True Parallel D&C via `rayon::join()`:**

Our parallel sorting implementations use `rayon::join()` to parallelize the recursive divide-and-conquer structure, rather than delegating to `par_sort_unstable()`:

```rust
pub fn parallel_merge_sort(arr: &mut [i32]) {
    parallel_merge_sort_with_threshold(arr, PARALLEL_THRESHOLD);
}

fn parallel_merge_sort_inner(arr: &mut [i32], buffer: &mut [i32], threshold: usize) {
    if arr.len() <= threshold {
        merge_sort_recursive(arr, 0, arr.len() - 1);
        return;
    }

    let mid = arr.len() / 2;
    let (left_arr, right_arr) = arr.split_at_mut(mid);
    let (left_buf, right_buf) = buffer.split_at_mut(mid);

    // Parallel recursive calls using rayon::join (work-stealing)
    rayon::join(
        || parallel_merge_sort_inner(left_arr, left_buf, threshold),
        || parallel_merge_sort_inner(right_arr, right_buf, threshold),
    );

    merge_with_buffer(arr, mid);
}
```

### **Thread Affinity with `start_handler`:**

For reproducible benchmarks on hybrid architectures (Intel P-core/E-core), we bind Rayon worker threads to specific cores using `ThreadPoolBuilder::start_handler`:

```rust
let pool = rayon::ThreadPoolBuilder::new()
    .num_threads(thread_count)
    .start_handler(move |thread_index| {
        // Pin each Rayon worker thread to a specific core
        if thread_index < cores.len() {
            let _ = core_affinity::set_for_current(cores[thread_index]);
        }
    })
    .build()
    .unwrap();

pool.install(|| {
    parallel_merge_sort(&mut data);
});
```

### **Expected Performance:**

- **< 8192 elements**: Sequential (below threshold)
- **10,000 elements**: 2-3x speedup
- **100,000 elements**: 4-8x speedup (depending on CPU cores)
- **1,000,000 elements**: 8-15x speedup

##  **Real-World Applications**

### **1. Data Analysis**

```rust
// Parallel aggregation of large datasets
let averages: Vec<f64> = data_chunks
    .par_iter()
    .map(|chunk| chunk.iter().sum::<f64>() / chunk.len() as f64)
    .collect();
```

### **2. Image Processing**

```rust
// Parallel image filtering
image_pixels
    .par_chunks_mut(width)
    .for_each(|row| apply_filter(row));
```

### **3. Scientific Computing**

```rust
// Parallel numerical computation
let results: Vec<f64> = (0..1000000)
    .into_par_iter()
    .map(|i| complex_math_function(i))
    .collect();
```

##  **Rayon’s Magic**

### **Automatic Scaling**

- **2 cores**: 2x speedup
- **8 cores**: 8x speedup
- **16 cores**: 16x speedup

### **Minimal Code Changes**

```rust
// Just change this one line
.iter()     →  .par_iter()     // Parallelization complete!
.sort()     →  .par_sort()     // Parallel sorting!
```

Rayon is called the library that **”democratized parallel processing”**. It made parallel programming, which traditionally required specialized knowledge, accessible to everyone.

## Related Documentation

- **[sorting.md](sorting.md)** - True parallel D&C sort implementations using `rayon::join()`
- **[thread_affinity.md](thread_affinity.md)** - Core binding with `ThreadPoolBuilder::start_handler`
- **[library_comparison.md](library_comparison.md)** - Benchmarks comparing custom sorts vs Rayon `par_sort`