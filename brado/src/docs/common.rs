use crate::docs::doc::Document;
use std::collections::HashSet;

const RADIX: u32 = 10;

pub fn to_digit(document: &Document) -> Vec<u8> {
    document
        .chars()
        .into_iter()
        .filter_map(|c| c.to_digit(RADIX).map(|c| c as u8))
        .collect()
}

pub fn valid_symbols(
    document: &Document,
    valid_symbols: HashSet<char>,
) -> bool {
    let symbols: HashSet<char> = document
        .chars()
        .into_iter()
        .filter(|c| c.to_digit(RADIX).is_none())
        .collect();

    valid_symbols == symbols
}
