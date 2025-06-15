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
    if arr.len() <= 1000 {
        merge_sort(arr);
        return;
    }

    // Use a completely different approach to avoid borrowing issues
    let mut temp = arr.to_vec();
    parallel_merge_sort_recursive(&mut temp);
    arr.copy_from_slice(&temp);
}

fn parallel_merge_sort_recursive(arr: &mut [i32]) {
    if arr.len() <= 1000 {
        merge_sort(arr);
        return;
    }

    let mid = arr.len() / 2;

    // Create temporary vectors for left and right halves
    let mut left_vec = arr[..mid].to_vec();
    let mut right_vec = arr[mid..].to_vec();

    // Sort both halves in parallel
    rayon::join(
        || parallel_merge_sort_recursive(&mut left_vec),
        || parallel_merge_sort_recursive(&mut right_vec),
    );

    // Merge the sorted halves back into the original array
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left_vec.len() && j < right_vec.len() {
        if left_vec[i] <= right_vec[j] {
            arr[k] = left_vec[i];
            i += 1;
        } else {
            arr[k] = right_vec[j];
            j += 1;
        }
        k += 1;
    }

    while i < left_vec.len() {
        arr[k] = left_vec[i];
        i += 1;
        k += 1;
    }

    while j < right_vec.len() {
        arr[k] = right_vec[j];
        j += 1;
        k += 1;
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
    if arr.len() <= 1000 {
        quick_sort(arr);
        return;
    }

    // Use a similar approach as parallel merge sort
    let mut temp = arr.to_vec();
    parallel_quick_sort_recursive(&mut temp);
    arr.copy_from_slice(&temp);
}

fn parallel_quick_sort_recursive(arr: &mut [i32]) {
    if arr.len() <= 1000 {
        quick_sort(arr);
        return;
    }

    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr, 0, arr.len() - 1);

    // Create temporary vectors for left and right parts
    let mut left_vec = arr[..pivot_index].to_vec();
    let mut right_vec = if pivot_index + 1 < arr.len() {
        arr[pivot_index + 1..].to_vec()
    } else {
        Vec::new()
    };

    // Sort both parts in parallel
    rayon::join(
        || parallel_quick_sort_recursive(&mut left_vec),
        || parallel_quick_sort_recursive(&mut right_vec),
    );

    // Copy back the sorted results
    arr[..pivot_index].copy_from_slice(&left_vec);
    if !right_vec.is_empty() {
        arr[pivot_index + 1..].copy_from_slice(&right_vec);
    }
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
}
