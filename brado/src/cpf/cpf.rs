use crate::common::utils::{get_digits, get_symbols, is_repeated};

pub fn validate(cpf: &String) -> bool {
    let size: usize = cpf.chars().count();

    if size != 11 && !is_masked(&cpf) {
        return false;
    }

    let digits: Vec<u8> = get_digits(&cpf);

    if digits.len() != 11 || is_repeated(&digits) {
        return false;
    }

    let (d10, d11): (u8, u8) = generate_digits(&digits[..10]);

    (d10, d11) == (digits[9], digits[10])
}

fn generate_digits(cpf: &[u8]) -> (u8, u8) {
    (generate_digit(&cpf, 10), generate_digit(&cpf, 11))
}

fn generate_digit(
    cpf: &[u8],
    max: u16,
) -> u8 {
    let mut sum: u16 = 0;

    for i in (2..=max).rev() {
        let idx = (max - i) as usize;
        let digit = cpf[idx] as u16;
        sum += digit * i;
    }

    sum = (sum * 10) % 11;

    if sum == 10 {
        sum = 0;
    }

    sum as u8
}

pub fn is_bare(cpf: &String) -> bool {
    cpf.chars().count() == 11 && get_digits(cpf).len() == 11
}

pub fn is_masked(cpf: &String) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(&cpf);
    if symbols.len() != 3 {
        return false;
    }
    symbols[0] == (3, '.') && symbols[1] == (7, '.') && symbols[2] == (11, '-')
}

pub fn mask(cpf: &String) -> String {
    if !is_bare(&cpf) {
        panic!("The given string cannot be masked as CPF!")
    }
    format!(
        "{}.{}.{}-{}",
        &cpf[0..3],
        &cpf[3..6],
        &cpf[6..9],
        &cpf[9..11],
    )
}
