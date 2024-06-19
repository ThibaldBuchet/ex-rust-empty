/// Create an array with the number of elements given in the parameter
/// Each element should be the value of the parameter
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::loops::create_array_with_given_number_of_elements(2), vec![2,2]);
/// assert_eq!(rust_ex::loops::create_array_with_given_number_of_elements(5), vec![5,5,5,5,5]);
/// ```
pub fn create_array_with_given_number_of_elements(number_of_elements: u32) -> Vec<u32> {
    // Write your code here

    vec![number_of_elements; number_of_elements as usize]
}

/// Write a loop that creates a suite of int from the first given parameter to the last one (inclusive)
/// For example, given 1 and 5, will create an array with 1,2,3,4,5
///
/// ```
/// assert_eq!(rust_ex::loops::create_array_with_elements_between_given_numbers(2,4), vec![2,3,4]);
/// assert_eq!(rust_ex::loops::create_array_with_elements_between_given_numbers(-2,3), vec![-2,-1,0,1,2,3]);
/// ```
pub fn create_array_with_elements_between_given_numbers(start: i32, end: i32) -> Vec<i32> {
    let mut result = Vec::new();

    for num in start..=end {
        result.push(num);
    }

    result
}

/// Write 2 loops that create a matrix with the given colums and rows.
/// The matrix cell must be filled with the multiplication of the column index and the row index
///
/// ```
/// assert_eq!(
///     rust_ex::loops::matrix_generation(2,2),
///     vec![
///         vec![1,2],
///         vec![2,4]
///     ]
/// )
/// ```
pub fn matrix_generation(rows: u32, columns: u32) -> Vec<Vec<u32>> {
    (0..rows)
        .map(|i| (0..columns).map(|j| (i + 1) * (j + 1)).collect())
        .collect()
}
