#![forbid(unsafe_code)]

use std::collections::HashMap;

mod math;
mod user_input;
mod utilities;

fn main() {
    let number: i32 = user_input::input("Enter a number:");
    let primes: Vec<i32> = math::primes_up_to(number);

    let reciprocal_repeating_digit_counts: Vec<i32> = primes
        .iter()
        .map(|&prime| math::reciprocal_decimal_count(prime))
        .collect();

    let primes_with_reciprocal_repeating_digit_counts: HashMap<i32, i32> = utilities::combine_vectors_into_hashmap(primes, reciprocal_repeating_digit_counts);

    let sorted_pairs: Vec<(&i32, &i32)> = utilities::sort_hashmap_by_key(&primes_with_reciprocal_repeating_digit_counts);

    utilities::print_hashmap(&sorted_pairs); // needs fixing
}