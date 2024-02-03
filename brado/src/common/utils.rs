use std::collections::HashSet;

const RADIX: u32 = 10;

pub fn is_repeated(digits: &[u8]) -> bool {
    let a_set: HashSet<u8> = HashSet::from_iter(digits.iter().cloned());
    a_set.len() == 1
}

pub fn get_digits(document: &String) -> Vec<u8> {
    document
        .chars()
        .into_iter()
        .filter_map(|c| c.to_digit(RADIX).map(|c| c as u8))
        .collect()
}

pub fn get_symbols(document: &String) -> Vec<(usize, char)> {
    document
        .chars()
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| match c.to_digit(RADIX) {
            Some(_) => None,
            None => Some((i, c)),
        })
        .collect()
}

pub fn valid_symbols(
    document: &String,
    valid_symbols: HashSet<char>,
) -> bool {
    let symbols: HashSet<char> = document
        .chars()
        .into_iter()
        .filter(|c| c.to_digit(RADIX).is_none())
        .collect();

    valid_symbols == symbols
}

pub fn unmask(document: &String) -> String {
    document
        .chars()
        .into_iter()
        .filter_map(|c| c.to_digit(RADIX).map(|c| c.to_string()))
        .collect::<Vec<String>>()
        .join("")
}
