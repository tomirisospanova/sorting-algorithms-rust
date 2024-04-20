// main.rs

extern crate sorting_library; // Import the sorting library

use sorting_library::{quick_sort, selection_sort, insertion_sort, merge_sort};

fn main() {
    let mut numbers = vec![10, 7, 15, 6];
    println!("Original: {:?}", numbers);

    // Quick Sort
    let mut numbers_quick = numbers.clone();
    quick_sort(&mut numbers_quick);
    println!("Quick Sort: {:?}", numbers_quick);

    // Selection Sort
    let mut numbers_selection = numbers.clone();
    selection_sort(&mut numbers_selection);
    println!("Selection Sort: {:?}", numbers_selection);

    // Insertion Sort
    let mut numbers_insertion = numbers.clone();
    insertion_sort(&mut numbers_insertion);
    println!("Insertion Sort: {:?}", numbers_insertion);

    // Merge Sort
    let mut numbers_merge = numbers.clone();
    merge_sort(&mut numbers_merge);
    println!("Merge Sort: {:?}", numbers_merge);
}
