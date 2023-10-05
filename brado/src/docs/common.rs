use std::collections::HashSet;
use crate::docs::doc::Document;

pub fn to_digit(document: &Document) -> Vec<u8> {
    const RADIX: u32 = 10;
    return document.chars()
        .into_iter()
        .filter_map(|c| {
            match c.to_digit(RADIX) {
                Some(c) => Some(c as u8),
                None => None,
            }
        })
        .collect();
}

pub fn valid_symbols(document: &Document, valid_symbols: HashSet<char>) -> bool {
    const RADIX: u32 = 10;
    let symbols: HashSet<char> = document.chars()
        .into_iter()
        .filter_map(|c| {
            match c.to_digit(RADIX) {
                Some(_c) => None,
                None => Some(c),
            }
        })
        .collect();
    return valid_symbols == symbols;
}
