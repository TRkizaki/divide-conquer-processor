/// Default threshold for switching from parallel to sequential execution.
/// Based on hardware-aware analysis: below this size, thread overhead
/// exceeds parallelization benefit.
const PARALLEL_THRESHOLD: usize = 8192;

/// Minimum partition size for parallel quicksort to avoid excessive task spawning
const QUICKSORT_PARALLEL_THRESHOLD: usize = 4096;

// ============================================================================
// Sequential Merge Sort
// ============================================================================

/// Sequential merge sort implementation using divide-and-conquer.
/// Time complexity: O(n log n) guaranteed.
/// Space complexity: O(n) auxiliary.
pub fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    merge_sort_recursive(arr, 0, len - 1);
}

fn merge_sort_recursive(arr: &mut [i32], left: usize, right: usize) {
    if left < right {
        let mid = left + (right - left) / 2;
        merge_sort_recursive(arr, left, mid);
        merge_sort_recursive(arr, mid + 1, right);
        merge(arr, left, mid, right);
    }
}

fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize) {
    let left_arr: Vec<i32> = arr[left..=mid].to_vec();
    let right_arr: Vec<i32> = arr[mid + 1..=right].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < left_arr.len() && j < right_arr.len() {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < left_arr.len() {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < right_arr.len() {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

// ============================================================================
// True Parallel Merge Sort (using rayon::join for recursive work-stealing)
// ============================================================================

/// Parallel merge sort using true divide-and-conquer parallelism via rayon::join.
///
/// Unlike a simple wrapper around `par_sort_unstable`, this implementation
/// preserves the merge sort algorithm structure while parallelizing the
/// recursive subproblem decomposition through work-stealing.
///
/// The threshold parameter controls the crossover point between parallel
/// and sequential execution, balancing thread overhead against parallelism.
pub fn parallel_merge_sort(arr: &mut [i32]) {
    parallel_merge_sort_with_threshold(arr, PARALLEL_THRESHOLD);
}

/// Parallel merge sort with configurable threshold for experimentation.
pub fn parallel_merge_sort_with_threshold(arr: &mut [i32], threshold: usize) {
    if arr.len() <= 1 {
        return;
    }
    // Allocate auxiliary buffer once, reused across all merge operations
    let mut buffer = vec![0i32; arr.len()];
    parallel_merge_sort_inner(arr, &mut buffer, threshold);
}

fn parallel_merge_sort_inner(arr: &mut [i32], buffer: &mut [i32], threshold: usize) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    // Below threshold: use sequential merge sort to avoid thread overhead
    if len <= threshold {
        merge_sort_recursive(arr, 0, len - 1);
        return;
    }

    let mid = len / 2;

    // Split the array and buffer into left and right halves
    let (left_arr, right_arr) = arr.split_at_mut(mid);
    let (left_buf, right_buf) = buffer.split_at_mut(mid);

    // Parallel recursive calls using rayon::join (work-stealing)
    rayon::join(
        || parallel_merge_sort_inner(left_arr, left_buf, threshold),
        || parallel_merge_sort_inner(right_arr, right_buf, threshold),
    );

    // Merge the sorted halves using the buffer
    merge_with_buffer(arr, mid);
}

/// Merge two sorted halves of a slice in-place using temporary allocation.
fn merge_with_buffer(arr: &mut [i32], mid: usize) {
    // Copy left half to temporary storage
    let left: Vec<i32> = arr[..mid].to_vec();
    let right_len = arr.len() - mid;

    let mut i = 0; // index into left
    let mut j = 0; // index into right half (starting at mid in arr)
    let mut k = 0; // index into output

    while i < left.len() && j < right_len {
        if left[i] <= arr[mid + j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = arr[mid + j];
            j += 1;
        }
        k += 1;
    }

    // Copy remaining left elements
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }
    // Remaining right elements are already in place
}

// ============================================================================
// Sequential Quick Sort
// ============================================================================

/// Sequential quick sort with median-of-three pivot selection.
/// Time complexity: O(n log n) average, O(n^2) worst case.
/// The median-of-three pivot selection mitigates worst-case behavior
/// on sorted/reverse-sorted inputs.
pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    quick_sort_recursive(arr, 0, arr.len() - 1);
}

fn quick_sort_recursive(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        // Use insertion sort for small subarrays
        if high - low < 16 {
            insertion_sort(arr, low, high);
            return;
        }

        let pivot_index = partition_median_of_three(arr, low, high);

        if pivot_index > 0 {
            quick_sort_recursive(arr, low, pivot_index - 1);
        }
        quick_sort_recursive(arr, pivot_index + 1, high);
    }
}

/// Insertion sort for small subarrays (used as base case in quicksort)
fn insertion_sort(arr: &mut [i32], low: usize, high: usize) {
    for i in (low + 1)..=high {
        let key = arr[i];
        let mut j = i;
        while j > low && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

/// Median-of-three pivot selection to avoid worst-case O(n^2) on sorted data.
/// Selects median of first, middle, and last elements as pivot.
fn partition_median_of_three(arr: &mut [i32], low: usize, high: usize) -> usize {
    let mid = low + (high - low) / 2;

    // Sort low, mid, high to find median
    if arr[low] > arr[mid] {
        arr.swap(low, mid);
    }
    if arr[low] > arr[high] {
        arr.swap(low, high);
    }
    if arr[mid] > arr[high] {
        arr.swap(mid, high);
    }

    // Place median pivot at high-1 position
    arr.swap(mid, high);
    let pivot = arr[high];

    let mut i = low;
    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

// ============================================================================
// True Parallel Quick Sort (using rayon::join for recursive work-stealing)
// ============================================================================

/// Parallel quick sort using true divide-and-conquer parallelism via rayon::join.
///
/// This implementation parallelizes the recursive partitioning of quicksort,
/// spawning left and right partition sorts as independent tasks that can be
/// stolen by idle threads in Rayon's work-stealing pool.
///
/// Uses median-of-three pivot selection to reduce worst-case probability.
pub fn parallel_quick_sort(arr: &mut [i32]) {
    parallel_quick_sort_with_threshold(arr, QUICKSORT_PARALLEL_THRESHOLD);
}

/// Parallel quick sort with configurable threshold for experimentation.
pub fn parallel_quick_sort_with_threshold(arr: &mut [i32], threshold: usize) {
    if arr.len() <= 1 {
        return;
    }
    parallel_quick_sort_inner(arr, threshold);
}

fn parallel_quick_sort_inner(arr: &mut [i32], threshold: usize) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    // Below threshold: use sequential sort to avoid thread overhead
    if len <= threshold {
        quick_sort_recursive(arr, 0, len - 1);
        return;
    }

    // Partition the array
    let pivot_index = partition_median_of_three(arr, 0, len - 1);

    // Split around pivot and recurse in parallel
    let (left, right_with_pivot) = arr.split_at_mut(pivot_index);
    let right = if right_with_pivot.len() > 1 {
        &mut right_with_pivot[1..] // skip the pivot element
    } else {
        &mut []
    };

    // Parallel recursive calls using rayon::join (work-stealing)
    rayon::join(
        || parallel_quick_sort_inner(left, threshold),
        || parallel_quick_sort_inner(right, threshold),
    );
}

// ============================================================================
// Threshold Experimentation Support
// ============================================================================

/// Run merge sort with a specific threshold value for threshold optimization studies.
/// Returns execution time in milliseconds.
pub fn benchmark_merge_sort_threshold(data: &[i32], threshold: usize) -> f64 {
    let mut arr = data.to_vec();
    let start = std::time::Instant::now();
    parallel_merge_sort_with_threshold(&mut arr, threshold);
    start.elapsed().as_secs_f64() * 1000.0
}

/// Run quick sort with a specific threshold value for threshold optimization studies.
/// Returns execution time in milliseconds.
pub fn benchmark_quick_sort_threshold(data: &[i32], threshold: usize) -> f64 {
    let mut arr = data.to_vec();
    let start = std::time::Instant::now();
    parallel_quick_sort_with_threshold(&mut arr, threshold);
    start.elapsed().as_secs_f64() * 1000.0
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        merge_sort(&mut arr);
        let expected: Vec<i32> = vec![];
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_merge_sort_single() {
        let mut arr = vec![42];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_merge_sort_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_quick_sort_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_duplicates() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_parallel_merge_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        parallel_merge_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_parallel_quick_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        parallel_quick_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn test_parallel_sorts_large() {
        use rand::Rng;
        let mut rng = rand::rng();
        let data: Vec<i32> = (0..50_000).map(|_| rng.random_range(-100_000..100_000)).collect();

        let mut merge_data = data.clone();
        let mut quick_data = data.clone();
        let mut reference = data.clone();

        parallel_merge_sort(&mut merge_data);
        parallel_quick_sort(&mut quick_data);
        reference.sort();

        assert_eq!(merge_data, reference, "Parallel merge sort produced incorrect result");
        assert_eq!(quick_data, reference, "Parallel quick sort produced incorrect result");
    }

    #[test]
    fn test_parallel_sorts_with_thresholds() {
        let data = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for threshold in [1, 2, 5, 10, 100] {
            let mut merge_data = data.clone();
            let mut quick_data = data.clone();

            parallel_merge_sort_with_threshold(&mut merge_data, threshold);
            parallel_quick_sort_with_threshold(&mut quick_data, threshold);

            assert_eq!(merge_data, expected, "Failed at merge sort threshold {}", threshold);
            assert_eq!(quick_data, expected, "Failed at quick sort threshold {}", threshold);
        }
    }
}
