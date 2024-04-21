pub mod sorting {
    use std::cmp::Ordering;

    // Quick Sort
    pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        if let Some(pivot_index) = partition(arr) {
            quick_sort(&mut arr[0..pivot_index]);
            quick_sort(&mut arr[pivot_index + 1..]);
        }
    }

    fn partition<T: PartialOrd>(arr: &mut [T]) -> Option<usize> {
        let pivot = arr.len() - 1;
        let mut i = 0;
        for j in 0..pivot {
            match arr[j].partial_cmp(&arr[pivot]) {
                Some(Ordering::Less) | Some(Ordering::Equal) => {
                    arr.swap(i, j);
                    i += 1;
                }
                _ => {}
            }
        }
        arr.swap(i, pivot);
        Some(i)
    }

    // Insertion Sort
    pub fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j].partial_cmp(&arr[j - 1]).map_or(false, |ord| ord == Ordering::Less) {
                arr.swap(j - 1, j);
                j -= 1;
            }
        }
    }

    // Selection Sort
    pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
        for i in 0..arr.len() {
            let mut min_index = i;
            for j in i + 1..arr.len() {
                if arr[j].partial_cmp(&arr[min_index]).map_or(false, |ord| ord == Ordering::Less) {
                    min_index = j;
                }
            }
            arr.swap(i, min_index);
        }
    }

    // Merge Sort
    pub fn merge_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
        let mid = arr.len() / 2;
        if mid == 0 {
            return;
        }
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();
        merge_sort(&mut left);
        merge_sort(&mut right);
        merge(arr, &left, &right);
    }

    fn merge<T: PartialOrd + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
        let mut left_idx = 0;
        let mut right_idx = 0;
        let mut target_idx = 0;

        while left_idx < left.len() && right_idx < right.len() {
            if left[left_idx].partial_cmp(&right[right_idx]).map_or(false, |ord| ord != Ordering::Greater) {
                arr[target_idx] = left[left_idx].clone();
                left_idx += 1;
            } else {
                arr[target_idx] = right[right_idx].clone();
                right_idx += 1;
            }
            target_idx += 1;
        }

        while left_idx < left.len() {
            arr[target_idx] = left[left_idx].clone();
            left_idx += 1;
            target_idx += 1;
        }

        while right_idx < right.len() {
            arr[target_idx] = right[right_idx].clone();
            right_idx += 1;
            target_idx += 1;
        }
    }
}
