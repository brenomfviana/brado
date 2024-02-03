use crate::common::utils::{get_digits, get_symbols};

pub fn validate(cnpj: &String) -> bool {
    let size: usize = cnpj.chars().count();

    if size != 14 && !is_masked(&cnpj) {
        return false;
    }

    let digits: Vec<u8> = get_digits(&cnpj);

    if digits.len() != 14 {
        return false;
    }

    for i in 0..10 {
        if digits.iter().filter(|&n| *n == i).count() == 14 {
            return false;
        }
    }

    let (d13, d14): (u8, u8) = generate_digits(&digits[..12]);

    (d13, d14) == (digits[12], digits[13])
}

fn generate_digits(cnpj_slice: &[u8]) -> (u8, u8) {
    let weights = vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let d13 = generate_digit(&cnpj_slice, 12, weights);

    let cnpj_slice = [&cnpj_slice[..], &vec![d13]].concat();
    let weights = vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let d14 = generate_digit(&cnpj_slice, 13, weights);

    (d13, d14)
}

fn generate_digit(
    cnpj_slice: &[u8],
    max: usize,
    weights: Vec<u16>,
) -> u8 {
    let mut sum: u16 = 0;

    for i in 0..max {
        let digit = cnpj_slice[i] as u16;
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

pub fn is_bare(cnpj: &String) -> bool {
    cnpj.chars().count() == 14 && get_digits(cnpj).len() == 14
}

pub fn is_masked(cnpj: &String) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(&cnpj);
    if symbols.len() != 4 {
        return false;
    }
    symbols[0] == (2, '.')
        && symbols[1] == (6, '.')
        && symbols[2] == (10, '/')
        && symbols[3] == (15, '-')
}

pub fn mask(cnpj: &String) -> String {
    if !is_bare(&cnpj) {
        panic!("The given string cannot be masked as CNPJ!")
    }
    format!(
        "{}.{}.{}/{}-{}",
        &cnpj[0..2],
        &cnpj[2..5],
        &cnpj[5..8],
        &cnpj[8..12],
        &cnpj[12..14],
    )
}
