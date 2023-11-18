use crate::docs::common::*;
use crate::docs::doc::Document;
use std::collections::HashSet;

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

pub fn is_repeated(digits: &[u8]) -> bool {
    let a_set: HashSet<u8> = HashSet::from_iter(digits.iter().cloned());
    a_set.len() == 1
}

pub fn validate(
    document: &Document,
    is_masked: bool,
    ignore_repeated: bool,
) -> bool {
    let symbols = HashSet::from_iter(['.', '-'].iter().cloned());
    if is_masked && !valid_symbols(document, symbols) {
        return false;
    }

    let digits: Vec<u8> = to_digit(document);

    if digits.len() != 11 {
        return false;
    }

    if !ignore_repeated && is_repeated(&digits) {
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

pub fn validate_str(
    document: &str,
    is_masked: bool,
    ignore_repeated: bool,
) -> bool {
    validate(&Document::new(document), is_masked, ignore_repeated)
}
