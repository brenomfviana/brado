use crate::common::{get_digits, get_symbols};

pub fn validate(cnpj: &str) -> bool {
    let size: usize = cnpj.chars().count();

    if size != 14 && !is_masked(cnpj) {
        return false;
    }

    let digits: Vec<u16> = get_digits(cnpj);

    if digits.len() != 14 {
        return false;
    }

    for i in 0..10 {
        if digits.iter().filter(|&n| *n == i).count() == 14 {
            return false;
        }
    }

    let (d13, d14): (u16, u16) = generate_digits(&digits[..12]);

    (d13, d14) == (digits[12], digits[13])
}

fn generate_digits(cnpj_slice: &[u16]) -> (u16, u16) {
    let mut cnpj_slice: Vec<u16> = cnpj_slice.to_vec();

    let weights: Vec<u16> = vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let d13: u16 = generate_digit(&cnpj_slice, 12, weights);

    cnpj_slice.push(d13);

    let weights: Vec<u16> = vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let d14: u16 = generate_digit(&cnpj_slice, 13, weights);

    (d13, d14)
}

fn generate_digit(
    cnpj_slice: &[u16],
    max: usize,
    weights: Vec<u16>,
) -> u16 {
    let mut sum: u16 = 0;

    for i in 0..max {
        sum += cnpj_slice[i] * weights[i];
    }

    sum %= 11;

    if sum < 10 {
        sum = 0;
    } else {
        sum = 11 - sum;
    }

    sum
}

pub fn is_bare(cnpj: &str) -> bool {
    cnpj.chars().count() == 14 && get_digits(cnpj).len() == 14
}

pub fn is_masked(cnpj: &str) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(cnpj);

    if symbols.len() != 4 {
        return false;
    }

    symbols[0] == (2, '.')
        && symbols[1] == (6, '.')
        && symbols[2] == (10, '/')
        && symbols[3] == (15, '-')
}

pub fn mask(cnpj: &str) -> String {
    if !is_bare(cnpj) {
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
