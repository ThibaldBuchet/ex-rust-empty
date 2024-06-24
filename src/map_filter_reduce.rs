use crate::deserialize_csv_file::EnergyProduction;

/// You receive a Vector containing many energy measures (from the previous exercise)
/// Compute the sum of energy produced and return the total
///
/// ```
/// assert_eq!(
///     rust_ex::map_filter_reduce::total_of_energy_production(vec![]),
///     0.0
/// );
/// assert_eq!(
///     rust_ex::map_filter_reduce::total_of_energy_production(
///         rust_ex::deserialize_csv_file::read_given_file_and_return_vec_of_structs(rust_ex::CSV_FILE_LOCATION)
///     ),
///     699209.7999999935
/// );
/// ```
pub fn total_of_energy_production(energy_measures: Vec<EnergyProduction>) -> f64 {
    let total_production: f64 = energy_measures.iter().map(|energy| energy.amount).sum();
    total_production

    // Write your code here
}
