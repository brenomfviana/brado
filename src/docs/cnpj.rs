use std::collections::HashSet;
use crate::docs::doc::Document;
use crate::docs::common::*;

fn generate_digit(document: &Vec<u8>, max: usize, weights: Vec<u16>) -> u8 {
    let mut sum: u16 = 0;
    for i in 0..max {
        let digit = document[i] as u16;
        sum += digit * weights[i];
    }
    sum = sum % 11;
    if sum < 10 {
        sum = 0;
    } else {
        sum = 11 - sum;
    }
    let sum: u8 = sum as u8;
    return sum
}

pub fn validate(document: Document) -> bool {
    let symbols = HashSet::from_iter(vec!['.', '/', '-'].iter().cloned());

    if !valid_symbols(&document, symbols) {
        return false;
    }

    let digits: Vec<u8> = to_digit(&document);

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

    let generated_digit13 = generate_digit(&digits, 12, vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);
    let generated_digit14 = generate_digit(&digits, 13, vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);

    let check_digit13 = digit13 == generated_digit13;
    let check_digit14 = digit14 == generated_digit14;

    return check_digit13 && check_digit14;
}
