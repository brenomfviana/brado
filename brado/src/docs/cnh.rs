use crate::docs::common::*;
use crate::docs::doc::Document;
use std::collections::HashSet;

fn generate_digits(document: &[u8]) -> (u8, u8) {
    // First Digit
    let mut sum: u16 = 0;
    let mut dsc: u16 = 0;

    for i in (0..=9).rev() {
        sum += (document[9 - i] as u16) * (i as u16);
    }

    let mut first = sum % 11;
    if first >= 10 {
        first = 0;
        dsc = 2;
    }

    // Second Digit
    let mut sum: u16 = 0;

    for i in 1..=9 {
        sum += (document[i - 1] as u16) * (i as u16);
    }

    let mut second: i16 = (sum % 11) as i16 - dsc as i16;
    if second < 0 {
        second += 11;
    } else if second >= 10 {
        second = 0;
    }

    (first as u8, second as u8)
}

pub fn validate(
    document: &Document,
    is_masked: bool,
    ignore_repeated: bool,
) -> bool {
    println!("{:?}", document.chars());
    let symbols = HashSet::from_iter([' '].iter().cloned());

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

    let (generated_digit10, generated_digit11) = generate_digits(&digits);
    println!("{} {}", generated_digit10, generated_digit11);

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
