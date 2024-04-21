use sorting_library::sorting::*;

fn main() {
    let mut characters = vec!['u', 'b', 's', 'l'];
    merge_sort(&mut characters);
    println!(" Merge Sort: {:?}", characters);

    let mut vec = vec![67,38 ,99, 78, 65];
    quick_sort(&mut vec);
    println!(" Quick Sort: {:?}", vec);

    let mut floats = vec![5.67, 6.45, 6.78, 9.78, 3.78];
    selection_sort(&mut floats);
    println!("Selection Sort: {:?}", floats);

    let mut words = vec!["blockchain".to_string(), "tomi".to_string(), "dragon".to_string()];
    insertion_sort(&mut words);
    println!("Insertion Sort: {:?}", words);
    
}
