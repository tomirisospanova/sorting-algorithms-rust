// src/lib.rs

// Quicksort
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);

    quick_sort(&mut arr[..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);

    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

// Selection Sort
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

// Insertion Sort
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

// Merge Sort
pub fn merge_sort<T: Ord + Clone + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut result = Vec::with_capacity(len);
    let (mut i, mut j) = (0, mid);

    while i < mid && j < len {
        if arr[i] <= arr[j] {
            result.push(arr[i].clone());
            i += 1;
        } else {
            result.push(arr[j].clone());
            j += 1;
        }
    }

    if i < mid {
        result.extend_from_slice(&arr[i..mid]);
    } else {
        result.extend_from_slice(&arr[j..]);
    }

    arr.copy_from_slice(&result);
}

