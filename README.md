# Sorting Algorithms in Rust

This project implements four sorting algorithms in Rust: Quick Sort, Selection Sort, Insertion Sort, and Merge Sort. Each algorithm is implemented as a separate function in the `sorting_library` crate.

## Usage

To use the sorting algorithms provided in this project, you can follow these steps:

1. Clone the repository to your local machine:

    ```bash
    git clone https://github.com/tomirisospanova/sorting-algorithms-rust.git
    ```

2. Navigate to the project directory:

    ```bash
    cd sorting-algorithms-rust
    ```

3. Open the `src/main.rs` file and modify it to sort your desired array of elements. You can also modify or extend the sorting algorithms in `src/lib.rs` if needed.

4. Run the project using Cargo:

    ```bash
    cargo run
    ```

## Examples

Here are some examples demonstrating the usage of the sorting algorithms provided:

### Original Array

```rust
let numbers = vec![10, 7, 15, 6];
println!("Original: {:?}", numbers);
