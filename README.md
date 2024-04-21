# Sorting Algorithms in Rust

This project implements four sorting algorithms in Rust: Quick Sort, Selection Sort, Insertion Sort, and Merge Sort. Each algorithm is implemented as a separate function in the `sorting_library` crate.

## Usage

To use the sorting algorithms provided in this project, you can follow these steps:

1. Clone the repository to your local machine:

    ```bash
    git clone https://github.com/tomirisospanova/sorting-algorithms-rust.git
    ```
<img width="1020" alt="image" src="https://github.com/tomirisospanova/sorting-algorithms-rust/assets/124910398/ce8784dd-64ed-4fbe-81e6-abe40ae7212b">

2. Navigate to the project directory:

    ```bash
    cd sorting-algorithms-rust
    ```
<img width="299" alt="image" src="https://github.com/tomirisospanova/sorting-algorithms-rust/assets/124910398/cd0c09b2-5550-4eaa-b6b1-a7daad749318">

3. Open the `src/main.rs` file and modify it to sort your desired array of elements. You can also modify or extend the sorting algorithms in `src/lib.rs` if needed.
<img width="119" alt="image" src="https://github.com/tomirisospanova/sorting-algorithms-rust/assets/124910398/f180fc97-6364-40db-8f68-6d49a253de71">
<img width="270" alt="image" src="https://github.com/tomirisospanova/sorting-algorithms-rust/assets/124910398/10f728bf-7e48-4163-b745-f18424e62fe5">

4. Run the project using Cargo:

    ```bash
    cargo run
    ```
<img width="336" alt="image" src="https://github.com/tomirisospanova/sorting-algorithms-rust/assets/124910398/e5a5cc46-5b88-493d-a5f7-18d1e0584f3f">
<img width="306" alt="image" src="https://github.com/tomirisospanova/sorting-algorithms-rust/assets/124910398/0a4e3e54-b35f-44f3-abf0-4f2a65bbe91d">

## Examples

Here are some examples demonstrating the usage of the sorting algorithms provided:
<img width="315" alt="image" src="https://github.com/tomirisospanova/sorting-algorithms-rust/assets/124910398/9f77b52b-3419-4111-9ce2-a423921f1e63">
<img width="306" alt="image" src="https://github.com/tomirisospanova/sorting-algorithms-rust/assets/124910398/ce071e20-c24d-4e3b-9212-b1b153d5e930">


### Original Array

```rust
let numbers = vec![10, 7, 15, 6];
println!("Original: {:?}", numbers);
