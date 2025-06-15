use rayon::prelude::*;

/// Sequential merge sort implementation
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
    let left_size = mid - left + 1;
    let right_size = right - mid;
    
    // Create temporary arrays
    let left_arr: Vec<i32> = arr[left..=mid].to_vec();
    let right_arr: Vec<i32> = arr[mid + 1..=right].to_vec();
    
    let mut i = 0; // Index for left_arr
    let mut j = 0; // Index for right_arr
    let mut k = left; // Index for merged array
    
    // Merge the two arrays
    while i < left_size && j < right_size {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }
    
    // Copy remaining elements
    while i < left_size {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }
    
    while j < right_size {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

/// Parallel merge sort implementation using Rayon
pub fn parallel_merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    
    parallel_merge_sort_recursive(arr, 0, len - 1, 0);
}

fn parallel_merge_sort_recursive(arr: &mut [i32], left: usize, right: usize, depth: usize) {
    if left < right {
        let mid = left + (right - left) / 2;
        
        // Use parallel execution for larger chunks and limited depth
        if right - left > 1000 && depth < 4 {
            let (left_half, right_half) = arr.split_at_mut(mid + 1);
            rayon::join(
                || parallel_merge_sort_recursive(&mut left_half[left..], left, mid, depth + 1),
                || parallel_merge_sort_recursive(&mut right_half[..right - mid], 0, right - mid - 1, depth + 1),
            );
        } else {
            parallel_merge_sort_recursive(arr, left, mid, depth + 1);
            parallel_merge_sort_recursive(arr, mid + 1, right, depth + 1);
        }
        
        merge(arr, left, mid, right);
    }
}

/// Sequential quick sort implementation
pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    
    quick_sort_recursive(arr, 0, arr.len() - 1);
}

fn quick_sort_recursive(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        
        if pivot_index > 0 {
            quick_sort_recursive(arr, low, pivot_index - 1);
        }
        quick_sort_recursive(arr, pivot_index + 1, high);
    }
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    // Choose the rightmost element as pivot
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

/// Parallel quick sort implementation using Rayon
pub fn parallel_quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    
    parallel_quick_sort_recursive(arr, 0, arr.len() - 1, 0);
}

fn parallel_quick_sort_recursive(arr: &mut [i32], low: usize, high: usize, depth: usize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        
        // Use parallel execution for larger chunks and limited depth
        if high - low > 1000 && depth < 4 {
            rayon::join(
                || {
                    if pivot_index > 0 {
                        parallel_quick_sort_recursive(arr, low, pivot_index - 1, depth + 1);
                    }
                },
                || parallel_quick_sort_recursive(arr, pivot_index + 1, high, depth + 1),
            );
        } else {
            if pivot_index > 0 {
                parallel_quick_sort_recursive(arr, low, pivot_index - 1, depth + 1);
            }
            parallel_quick_sort_recursive(arr, pivot_index + 1, high, depth + 1);
        }
    }
}

/// Heap sort implementation (additional algorithm for comparison)
pub fn heap_sort(arr: &mut [i32]) {
    let len = arr.len();
    
    // Build max heap
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }
    
    // Extract elements from heap one by one
    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [i32], heap_size: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;
    
    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }
    
    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }
    
    if largest != root {
        arr.swap(root, largest);
        heapify(arr, heap_size, largest);
    }
}

/// Optimized quick sort with different pivot strategies
pub fn quick_sort_optimized(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    
    quick_sort_optimized_recursive(arr, 0, arr.len() - 1);
}

fn quick_sort_optimized_recursive(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        // Use insertion sort for small arrays
        if high - low < 10 {
            insertion_sort_range(arr, low, high);
            return;
        }
        
        let pivot_index = partition_median_of_three(arr, low, high);
        
        if pivot_index > 0 {
            quick_sort_optimized_recursive(arr, low, pivot_index - 1);
        }
        quick_sort_optimized_recursive(arr, pivot_index + 1, high);
    }
}

fn partition_median_of_three(arr: &mut [i32], low: usize, high: usize) -> usize {
    // Median-of-three pivot selection
    let mid = low + (high - low) / 2;
    
    if arr[mid] < arr[low] {
        arr.swap(low, mid);
    }
    if arr[high] < arr[low] {
        arr.swap(low, high);
    }
    if arr[high] < arr[mid] {
        arr.swap(mid, high);
    }
    
    // Place median at the end as pivot
    arr.swap(mid, high);
    
    partition(arr, low, high)
}

fn insertion_sort_range(arr: &mut [i32], low: usize, high: usize) {
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

/// Radix sort implementation for positive integers
pub fn radix_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    
    // Handle negative numbers by shifting
    let min_val = *arr.iter().min().unwrap();
    let offset = if min_val < 0 { -min_val } else { 0 };
    
    // Shift all values to be non-negative
    for val in arr.iter_mut() {
        *val += offset;
    }
    
    let max_val = *arr.iter().max().unwrap();
    let mut exp = 1;
    
    while max_val / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
    
    // Shift back to original range
    for val in arr.iter_mut() {
        *val -= offset;
    }
}

fn counting_sort_by_digit(arr: &mut [i32], exp: i32) {
    let len = arr.len();
    let mut output = vec![0; len];
    let mut count = [0; 10];
    
    // Count occurrences of each digit
    for &val in arr.iter() {
        let digit = ((val / exp) % 10) as usize;
        count[digit] += 1;
    }
    
    // Change count[i] to actual position
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    
    // Build output array
    for &val in arr.iter().rev() {
        let digit = ((val / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = val;
    }
    
    // Copy output back to arr
    arr.copy_from_slice(&output);
}

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
    fn test_quick_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }
    
    #[test]
    fn test_parallel_sorts() {
        let mut arr1 = vec![64, 34, 25, 12, 22, 11, 90];
        let mut arr2 = arr1.clone();
        
        parallel_merge_sort(&mut arr1);
        parallel_quick_sort(&mut arr2);
        
        let expected = vec![11, 12, 22, 25, 34, 64, 90];
        assert_eq!(arr1, expected);
        assert_eq!(arr2, expected);
    }
    
    #[test]
    fn test_heap_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }
    
    #[test]
    fn test_radix_sort() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }
    
    #[test]
    fn test_radix_sort_with_negatives() {
        let mut arr = vec![-64, 34, -25, 12, 22, -11, 90];
        radix_sort(&mut arr);
        assert_eq!(arr, vec![-64, -25, -11, 12, 22, 34, 90]);
    }
}