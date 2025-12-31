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

    let left_arr: Vec<i32> = arr[left..=mid].to_vec();
    let right_arr: Vec<i32> = arr[mid + 1..=right].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

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

/// Parallel merge sort using Rayon
pub fn parallel_merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1000 {
        merge_sort(arr);
        return;
    }

    // Use Rayon's parallel sort for actual parallel processing
    arr.par_sort_unstable();
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

/// Parallel quick sort using Rayon
pub fn parallel_quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1000 {
        quick_sort(arr);
        return;
    }

    // Use Rayon's parallel sort for actual parallel processing
    arr.par_sort_unstable();
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
