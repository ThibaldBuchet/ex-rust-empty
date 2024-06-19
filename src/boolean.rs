// Write the body of the above functions. You can use the tests at the end of file to validate your code.

/// Test if both inputs are not true
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::nand(false, false), true);
/// assert_eq!(rust_ex::boolean::nand(false, true), true);
/// assert_eq!(rust_ex::boolean::nand(true, false), true);
/// assert_eq!(rust_ex::boolean::nand(true, true), false);
/// ```
/// An easier way to visualize the function is the following table:
/// | A | B | Out |
/// |---|---|-----|
/// | 0 | 0 |  1  |
/// | 0 | 1 |  1  |
/// | 1 | 0 |  1  |
/// | 1 | 1 |  0  |
pub fn nand(first: bool, second: bool) -> bool {
    !(first && second)
}

/// Test if none of the inputs are true
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::nor(false, false), true);
/// assert_eq!(rust_ex::boolean::nor(false, true), false);
/// assert_eq!(rust_ex::boolean::nor(true, false), false);
/// assert_eq!(rust_ex::boolean::nor(true, true), false);
/// ```
/// An easier way to visualize the function is the following table:
/// | A | B | Out |
/// |---|---|-----|
/// | 0 | 0 |  1  |
/// | 0 | 1 |  0  |
/// | 1 | 0 |  0  |
/// | 1 | 1 |  0  |
pub fn nor(first: bool, second: bool) -> bool {
    !(first || second)
}

/// Test if either one of the 2 inputs are true but not both
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::xor(false, false), false);
/// assert_eq!(rust_ex::boolean::xor(false, true), true);
/// assert_eq!(rust_ex::boolean::xor(true, false), true);
/// assert_eq!(rust_ex::boolean::xor(true, true), false);
/// ```
/// An easier way to visualize the function is the following table:
/// | A | B | Out |
/// |---|---|-----|
/// | 0 | 0 |  0  |
/// | 0 | 1 |  1  |
/// | 1 | 0 |  1  |
/// | 1 | 1 |  0  |
pub fn xor(first: bool, second: bool) -> bool {
    first ^ second
}

/// Test if both inputs are true or both are false
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::xnor(false, false), true);
/// assert_eq!(rust_ex::boolean::xnor(false, true), false);
/// assert_eq!(rust_ex::boolean::xnor(true, false), false);
/// assert_eq!(rust_ex::boolean::xnor(true, true), true);
/// ```
/// An easier way to visualize the function is the following table:
/// | A | B | Out |
/// |---|---|-----|
/// | 0 | 0 |  1  |
/// | 0 | 1 |  0  |
/// | 1 | 0 |  0  |
/// | 1 | 1 |  1  |
pub fn xnor(first: bool, second: bool) -> bool {
    !(first ^ second)
}

/// Test if the first number is a multiple of the second number
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::is_multiple(125, 5), true);
/// assert_eq!(rust_ex::boolean::is_multiple(5, 125), false);
/// assert_eq!(rust_ex::boolean::is_multiple(100, 33), false);
/// ```
pub fn is_multiple(number: i32, multiple: i32) -> bool {
    (number % multiple) == 0
}

/// Let's solve a prolem with boolean logic
/// You have a light bulb in a room and 3 switches outside the room
/// When you flip a switch, the state of the light bulb changes
/// You should implement a function that returns the state of the light bulb depending on the state of the switches
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::light_bulb(false, false, false), false);
/// assert_eq!(rust_ex::boolean::light_bulb(false, false, true), true);
/// assert_eq!(rust_ex::boolean::light_bulb(false, true, false), true);
/// assert_eq!(rust_ex::boolean::light_bulb(false, true, true), false);
/// assert_eq!(rust_ex::boolean::light_bulb(true, false, false), true);
/// assert_eq!(rust_ex::boolean::light_bulb(true, false, true), false);
/// assert_eq!(rust_ex::boolean::light_bulb(true, true, false), false);
/// assert_eq!(rust_ex::boolean::light_bulb(true, true, true), true);
/// ```
///
/// An easier way to visualize the function is the following table:
/// | Switch 1 | Switch 2 | Switch 3 | Light bulb |
/// |----------|----------|----------|------------|
/// |    0     |    0     |    0     |     0      |
/// |    0     |    0     |    1     |     1      |
/// |    0     |    1     |    0     |     1      |
/// |    0     |    1     |    1     |     0      |
/// |    1     |    0     |    0     |     1      |
/// |    1     |    0     |    1     |     0      |
/// |    1     |    1     |    0     |     0      |
/// |    1     |    1     |    1     |     1      |
pub fn light_bulb(switch1: bool, switch2: bool, switch3: bool) -> bool {

    switch1 ^ switch2 ^ switch3

}

/// Test if either the first 2 input are true or if the 3rd one is true
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::first2_or_third(false, false, false), false);
/// assert_eq!(rust_ex::boolean::first2_or_third(false, false, true), true);
/// assert_eq!(rust_ex::boolean::first2_or_third(false, true, false), false);
/// assert_eq!(rust_ex::boolean::first2_or_third(false, true, true), true);
/// assert_eq!(rust_ex::boolean::first2_or_third(true, false, false), false);
/// assert_eq!(rust_ex::boolean::first2_or_third(true, false, true), true);
/// assert_eq!(rust_ex::boolean::first2_or_third(true, true, false), true);
/// assert_eq!(rust_ex::boolean::first2_or_third(true, true, true), true);
/// ```
/// An easier way to visualize the function is the following table:
/// | A | B | C | Out |
/// |---|---|---|-----|
/// | 0 | 0 | 0 |  0  |
/// | 0 | 0 | 1 |  1  |
/// | 0 | 1 | 0 |  0  |
/// | 0 | 1 | 1 |  1  |
/// | 1 | 0 | 0 |  0  |
/// | 1 | 0 | 1 |  1  |
/// | 1 | 1 | 0 |  1  |
/// | 1 | 1 | 1 |  1  |
pub fn first2_or_third(first: bool, second: bool, third: bool) -> bool {
    // Add boolean operators between the inputs to make the test pass, do not change the order of the inputs
    // Write your code here 
    (first && second) || third
}

/// Test if the last 2 input are true or if the 1st one is true
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::last2_or_first(false, false, false), false);
/// assert_eq!(rust_ex::boolean::last2_or_first(false, false, true), false);
/// assert_eq!(rust_ex::boolean::last2_or_first(false, true, false), false);
/// assert_eq!(rust_ex::boolean::last2_or_first(false, true, true), true);
/// assert_eq!(rust_ex::boolean::last2_or_first(true, false, false), true);
/// assert_eq!(rust_ex::boolean::last2_or_first(true, false, true), true);
/// assert_eq!(rust_ex::boolean::last2_or_first(true, true, false), true);
/// assert_eq!(rust_ex::boolean::last2_or_first(true, true, true), true);
/// ```
/// An easier way to visualize the function is the following table:
/// | A | B | C | Out |
/// |---|---|---|-----|
/// | 0 | 0 | 0 |  0  |
/// | 0 | 0 | 1 |  0  |
/// | 0 | 1 | 0 |  0  |
/// | 0 | 1 | 1 |  1  |
/// | 1 | 0 | 0 |  1  |
/// | 1 | 0 | 1 |  1  |
/// | 1 | 1 | 0 |  1  |
/// | 1 | 1 | 1 |  1  |
pub fn last2_or_first(first: bool, second: bool, third: bool) -> bool {
    // Add boolean operators between the inputs to make the test pass, do not change the order of the inputs
    // Write your code here
    first || second && third  

}

/// Test if the first number is between the second and the third number
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::is_between(5, 3, 7), true);
/// assert_eq!(rust_ex::boolean::is_between(5, 7, 3), true);
/// assert_eq!(rust_ex::boolean::is_between(3, 5, 7), false);
/// assert_eq!(rust_ex::boolean::is_between(7, 5, 3), false);
/// assert_eq!(rust_ex::boolean::is_between(-1, -3, 3), true);
/// ```
/// The function should return false if the first number is equal to the second or the third number
/// ```
/// assert_eq!(rust_ex::boolean::is_between(5, 5, 7), false);
/// assert_eq!(rust_ex::boolean::is_between(7, 5, 7), false);
/// ```
pub fn is_between(number: i32, first: i32, second: i32) -> bool {
    let min = first.min(second);  
    let max = first.max(second);  
    number > min && number < max  
}


/// Addition of 2 true bit binary numbers (false or true)
/// The function should return a tuple with the sum and the carry
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::half_adder(false, false), (false, false));
/// assert_eq!(rust_ex::boolean::half_adder(false, true), (true, false));
/// assert_eq!(rust_ex::boolean::half_adder(true, false), (true, false));
/// assert_eq!(rust_ex::boolean::half_adder(true, true), (false, true));
/// ```
/// An easier way to visualize the function is the following table:
/// | A | B | Sum | Cout |
/// |---|---|-----|------|
/// | 0 | 0 |  0  |  0   |
/// | 0 | 1 |  1  |  0   |
/// | 1 | 0 |  1  |  0   |
/// | 1 | 1 |  0  |  1   |
pub fn half_adder(a: bool, b: bool) -> (bool, bool) {
    // Write your code here

    let sum = a ^ b;
    let cout = a && b; 
    (sum, cout)
}

// Half adder are great but they are limited to adding 2 bits, if we want to be able to add any
// number together we need to create a full adder as you can chain them together to add any number
/// Addition 2 true bit binary numbers (false or true) and a carry bit (false or true)
/// The function should return a tuple with the sum and the carry
///
/// Usage example
/// ```
/// assert_eq!(rust_ex::boolean::full_adder(false, false, false), (false, false));
/// assert_eq!(rust_ex::boolean::full_adder(false, false, true), (true, false));
/// assert_eq!(rust_ex::boolean::full_adder(false, true, false), (true, false));
/// assert_eq!(rust_ex::boolean::full_adder(false, true, true), (false, true));
/// assert_eq!(rust_ex::boolean::full_adder(true, false, false), (true, false));
/// assert_eq!(rust_ex::boolean::full_adder(true, false, true), (false, true));
/// assert_eq!(rust_ex::boolean::full_adder(true, true, false), (false, true));
/// assert_eq!(rust_ex::boolean::full_adder(true, true, true), (true, true));
/// ```
/// An easier way to visualize the function is the following table:
/// | A | B | Cin | Sum | Cout |
/// |---|---|-----|-----|------|
/// | 0 | 0 |  0  |  0  |  0   |
/// | 0 | 0 |  1  |  1  |  0   |
/// | 0 | 1 |  0  |  1  |  0   |
/// | 0 | 1 |  1  |  0  |  1   |
/// | 1 | 0 |  0  |  1  |  0   |
/// | 1 | 0 |  1  |  0  |  1   |
/// | 1 | 1 |  0  |  0  |  1   |
/// | 1 | 1 |  1  |  1  |  1   |
pub fn full_adder(a: bool, b: bool, carry: bool) -> (bool, bool) {
    let sum = a ^ b ^ carry;
    let carry_out = (a & b) | (b & carry) | (a & carry);
    (sum, carry_out)
}
