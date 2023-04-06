#![forbid(unsafe_code)]

fn is_prime(number: &i32) -> bool {
    if *number <= 1 {
        return false;
    }
    if *number == 2 {
        return true;
    }
    if *number % 2 == 0 {
        return false;
    }
    let root_of_number: i32 = ( *number as f64).sqrt() as i32;
    !(3..=root_of_number).step_by(2).any(|n| *number % n == 0)
}

pub fn primes(count: usize) -> Vec<i32> {
    (2..).filter(|&n| is_prime(&n)).take(count).collect()
}

pub fn reciprocal_decimal_count(input: i32) -> i32 {
    let mut dividend: i32 = 1;
    let mut remainders: Vec<i32> = vec![-1; input as usize];
    let mut decimal_count: i32 = 0;

    while remainders[dividend as usize] == -1 {
        remainders[dividend as usize] = decimal_count;
        dividend = (dividend * 10) % input;
        decimal_count += 1;
    }

    decimal_count - remainders[dividend as usize]
}