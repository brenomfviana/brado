use std::collections::HashSet;
use crate::docs::doc::Document;
use crate::docs::common::*;

fn generate_digit(document: &Vec<u8>, max: u16) -> u8 {
    let mut sum: u16 = 0;
    for i in (2..=max).rev() {
        let aux = (max - i) as usize;
        let digit = document[aux] as u16;
        sum += digit * i;
    }
    sum = (sum * 10) % 11;
    if sum == 10 {
        sum = 0;
    }
    let sum: u8 = sum as u8;
    return sum
}

pub fn is_repeated(digits: &Vec<u8>) -> bool {
    let a_set: HashSet<u8> = HashSet::from_iter(digits.iter().cloned());
    return a_set.len() == 1;
}

pub fn validate(document: Document, is_masked: bool, ignore_repeated: bool) -> bool {
    let symbols = HashSet::from_iter(vec!['.', '-'].iter().cloned());
    if is_masked && !valid_symbols(&document, symbols) {
        return false;
    }

    let digits: Vec<u8> = to_digit(&document);

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

    return check_digit10 && check_digit11;
}
