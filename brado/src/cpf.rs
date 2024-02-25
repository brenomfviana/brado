use crate::common::{get_digits, get_symbols, is_repeated, random_digits};

pub fn validate(cpf: &str) -> bool {
    let size: usize = cpf.chars().count();

    if size != 11 && !is_masked(cpf) {
        return false;
    }

    let digits: Vec<u16> = get_digits(cpf);

    if digits.len() != 11 || is_repeated(&digits) {
        return false;
    }

    let (d10, d11): (u16, u16) = generate_digits(&digits[..10]);

    (d10, d11) == (digits[9], digits[10])
}

fn generate_digits(cpf_slice: &[u16]) -> (u16, u16) {
    let d10: u16 = generate_digit(cpf_slice, 10);
    let d11: u16 = generate_digit(cpf_slice, 11);

    (d10, d11)
}

fn generate_digit(
    cpf_slice: &[u16],
    max: u16,
) -> u16 {
    let mut sum: u16 = 0;

    for i in (2..=max).rev() {
        let idx: usize = (max - i) as usize;
        sum += cpf_slice[idx] * i;
    }

    sum = (sum * 10) % 11;

    if sum == 10 {
        sum = 0;
    }

    sum
}

pub fn generate() -> String {
    let mut cpf: Vec<u16> = random_digits(9);
    cpf.push(generate_digit(&cpf, 10));
    cpf.push(generate_digit(&cpf, 11));

    cpf.iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn is_bare(cpf: &str) -> bool {
    cpf.chars().count() == 11 && get_digits(cpf).len() == 11
}

pub fn is_masked(cpf: &str) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(cpf);

    if symbols.len() != 3 {
        return false;
    }

    symbols[0] == (3, '.') && symbols[1] == (7, '.') && symbols[2] == (11, '-')
}

pub fn mask(cpf: &str) -> String {
    if !is_bare(cpf) {
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
