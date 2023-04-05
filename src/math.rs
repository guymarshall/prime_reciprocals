#![forbid(unsafe_code)]

use std::collections::HashMap;
use rayon::prelude::*;

fn is_prime(number: &i32) -> bool {
    if number % 2 == 0 {
        return false;
    }
    let number_dereferenced: i32 = *number;
    let root_of_number: i32 = (number_dereferenced as f64).sqrt() as i32;
    !(3..root_of_number).step_by(2).any(|n| root_of_number % n == 0)
}

pub fn primes_up_to(number: i32) -> Vec<i32> {
    (2..=number).into_par_iter().filter(is_prime).collect::<Vec<i32>>()
}

pub fn reciprocal_decimal_count(input: i32) -> i32 {
    let mut dividend: i32 = 1;
    let mut remainders: HashMap<i32, i32> = HashMap::new();
    let mut decimal_count: i32 = 0;

    while !remainders.contains_key(&dividend) {
        remainders.insert(dividend, decimal_count);
        dividend = (dividend * 10) % input;
        decimal_count += 1;
    }

    decimal_count - remainders.get(&dividend).unwrap()
}