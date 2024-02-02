use crate::common::utils::{to_digit, valid_symbols};
use std::collections::HashSet;

fn generate_digit(
    document: &[u8],
    max: usize,
    weights: Vec<u16>,
) -> u8 {
    let mut sum: u16 = 0;

    for i in 0..max {
        let digit = document[i] as u16;
        sum += digit * weights[i];
    }

    sum %= 11;

    if sum < 10 {
        sum = 0;
    } else {
        sum = 11 - sum;
    }

    sum as u8
}

pub fn validate(
    document: &String,
    is_masked: bool,
) -> bool {
    let symbols = HashSet::from_iter(['.', '/', '-'].iter().cloned());

    if is_masked && !valid_symbols(document, symbols) {
        return false;
    }

    let digits: Vec<u8> = to_digit(document);

    if digits.len() != 14 {
        return false;
    }

    for i in 0..10 {
        if digits.iter().filter(|&n| *n == i).count() == 14 {
            return false;
        }
    }

    let digit13 = digits[12];
    let digit14 = digits[13];

    let weights = vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let generated_digit13 = generate_digit(&digits, 12, weights);
    let weights = vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let generated_digit14 = generate_digit(&digits, 13, weights);

    let check_digit13 = digit13 == generated_digit13;
    let check_digit14 = digit14 == generated_digit14;

    check_digit13 && check_digit14
}

pub fn validate_str(
    document: &str,
    is_masked: bool,
) -> bool {
    validate(&String::from(document), is_masked)
}
