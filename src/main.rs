#![forbid(unsafe_code)]

use std::collections::HashMap;

mod math;
mod user_input;
mod utilities;

fn main() {
    let number: i32 = user_input::input("Enter a number:");
    let primes: Vec<i32> = math::primes_up_to(number);

    let mut reciprocal_repeating_digit_counts: Vec<i32> = Vec::with_capacity(number as usize);
    (1..number).into_iter().for_each(|i| {
        let element: i32 = primes[i as usize];
        let repeating_digit_count: i32 = math::reciprocal_decimal_count(element);

        reciprocal_repeating_digit_counts[i as usize] = repeating_digit_count;
    });

    (0..number).into_iter().for_each(|i| {
        println!("Reciprocal of {} repeats after {} digits.", primes[i as usize], reciprocal_repeating_digit_counts[i as usize]);
    });

    let primes_with_reciprocal_repeating_digit_counts: HashMap<i32, i32> = utilities::combine_vectors_into_hashmap(primes, reciprocal_repeating_digit_counts);
}