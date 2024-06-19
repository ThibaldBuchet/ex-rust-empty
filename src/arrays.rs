// Write the body of the above functions. You can use the tests at the end of file to validate your code.

/// Calculate the average value of all elements in array
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::arrays::average(&[2, 6]), 4.0);
/// assert_eq!(rust_ex::arrays::average(&[1, 5, 3, 6, 7, 8, 3, 4]), 4.625);
/// ```
pub fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

/// Sort all the numbers in the array by size
///
/// Usage example
/// ```
/// assert_eq!(
///     rust_ex::arrays::sort_array_by_size(&[1, 5, 3, 6, 7, 8, 3, 4]),
///     vec![1, 3, 3, 4, 5, 6, 7, 8]
/// );
/// ```
pub fn sort_array_by_size(numbers: &[i32]) -> Vec<i32> {
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort(); 
    sorted_numbers 
}


/// Find the biggest and the smallest values in the array
///
/// Usage example
/// ```
/// assert_eq!(
///     rust_ex::arrays::get_biggest_and_smallest_elements_from_array(&[1, 5, 3, 6, 7, 8, 3, 4]),
///     (&8, &1)
/// );
/// ```
/// Given no numbers, the function should panic
/// ```rust,should_panic
/// rust_ex::arrays::get_biggest_and_smallest_elements_from_array(&[]);
/// ```
pub fn get_biggest_and_smallest_elements_from_array(numbers: &[i32]) -> (&i32, &i32) {
    // Write your code here

    let max_value = numbers.iter().max().unwrap();
    let min_value = numbers.iter().min().unwrap();

    (max_value, min_value)

}

/// Create two arrays, one with all odd numbers from the original array, another with all even numbers
///
/// Usage example
/// ```
/// assert_eq!(
///     rust_ex::arrays::separate_odd_from_even_numbers(&[1, 5, 3, 6, 7, 8, 3, 4]),
///     (vec![1, 5, 3, 7, 3], vec![6, 8, 4])
/// )
/// ```
pub fn separate_odd_from_even_numbers(numbers: &[i32]) -> (Vec<i32>, Vec<i32>) {
    // Write your code here
    todo!()
}

/// Sum the odd numbers together, and the even one together
///
/// ```
/// assert_eq!(
///     rust_ex::arrays::add_odd_numbers_and_add_even_numbers(vec![2,3,4,45,64,233,954]),
///     (281, 1024)
/// );
/// ```
pub fn add_odd_numbers_and_add_even_numbers(numbers: Vec<i32>) -> (i32, i32) {
    // Write your code here
    todo!()
}
