use std::collections::HashSet;

const RADIX: u32 = 10;

pub fn is_repeated(digits: &[u16]) -> bool {
    let a_set: HashSet<u16> = HashSet::from_iter(digits.iter().cloned());
    a_set.len() == 1
}

pub fn get_digits(document: &str) -> Vec<u16> {
    document
        .chars()
        .filter_map(|c| c.to_digit(RADIX).map(|c| c as u16))
        .collect()
}

pub fn get_symbols(document: &str) -> Vec<(usize, char)> {
    document
        .chars()
        .enumerate()
        .filter_map(|(i, c)| match c.to_digit(RADIX) {
            Some(_) => None,
            None => Some((i, c)),
        })
        .collect()
}

pub fn unmask(document: &str) -> String {
    document
        .chars()
        .filter_map(|c| c.to_digit(RADIX).map(|c| c.to_string()))
        .collect::<Vec<String>>()
        .join("")
}
