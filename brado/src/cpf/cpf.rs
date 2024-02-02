use crate::common::utils::{is_repeated, to_digit, valid_symbols};
use std::collections::HashSet;

pub fn validate_str(
    document: &str,
    is_masked: bool,
) -> bool {
    validate(&String::from(document), is_masked)
}

pub fn validate(
    document: &String,
    is_masked: bool,
) -> bool {
    let symbols = HashSet::from_iter(['.', '-'].iter().cloned());
    if is_masked && !valid_symbols(&document, symbols) {
        return false;
    }

    let digits: Vec<u8> = to_digit(&document);

    if digits.len() != 11 {
        return false;
    }

    #[cfg(not(ignore_repeated))]
    if is_repeated(&digits) {
        return false;
    }

    let digit10 = digits[9];
    let digit11 = digits[10];

    let generated_digit10 = generate_digit(&digits, 10);
    let generated_digit11 = generate_digit(&digits, 11);

    let check_digit10 = digit10 == generated_digit10;
    let check_digit11 = digit11 == generated_digit11;

    check_digit10 && check_digit11
}

fn generate_digit(
    document: &[u8],
    max: u16,
) -> u8 {
    let mut sum: u16 = 0;

    for i in (2..=max).rev() {
        let idx = (max - i) as usize;
        let digit = document[idx] as u16;
        sum += digit * i;
    }

    sum = (sum * 10) % 11;

    if sum == 10 {
        sum = 0;
    }

    sum as u8
}
