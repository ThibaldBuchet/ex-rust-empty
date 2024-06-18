/// Welcome to rust exercises
/// You will find some exercises to complete on the different modules of this crate.
/// You can complete them in the following order ;
/// 1. This file -> some basics
/// 2. [Booleans](./boolean.rs)
/// 3. [Loops](./loops.rs)
/// 4. [Arrays](./arrays.rs)
/// 5. [Struct and enums](./struct_enum.rs)
/// 6. [Deserialize csv](./deserialize_csv_file.rs)
/// 7. [Map filter reduce](./map_filter_reduce.rs)
///
/// Use the `cargo test` command to check your work, or directly run the test from your ide.
pub mod arrays;
pub mod boolean;
pub mod deserialize_csv_file;
pub mod loops;
pub mod map_filter_reduce;
pub mod struct_enum;

pub const CSV_FILE_LOCATION: &str = "2024-06-17_swisgrid_production.csv";

/// Add and return the two numbers given to his function
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::add(4, 8), 12);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    // Write your code here
    left + right
}
