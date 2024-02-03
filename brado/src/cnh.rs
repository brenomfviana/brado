use crate::common::{get_digits, get_symbols, is_repeated};

pub fn validate(cnh: &str) -> bool {
    let size: usize = cnh.chars().count();

    if size != 11 && !is_masked(cnh) {
        return false;
    }

    let digits: Vec<u16> = get_digits(cnh);

    if digits.len() != 11 {
        return false;
    }

    if is_repeated(&digits) {
        return false;
    }

    let (d10, d11): (u16, u16) = generate_digits(&digits);

    (d10, d11) == (digits[9], digits[10])
}

fn generate_digits(cpf_slice: &[u16]) -> (u16, u16) {
    let (d10, dsc): (u16, u16) = generate_first_digit(cpf_slice);
    let d11: u16 = generate_second_digit(cpf_slice, dsc);

    (d10, d11)
}

fn generate_first_digit(cnh: &[u16]) -> (u16, u16) {
    let mut sum: u16 = 0;
    let mut dsc: u16 = 0;

    for i in (0..=9).rev() {
        sum += cnh[9 - i] * (i as u16);
    }

    let mut first: u16 = sum % 11;

    if first >= 10 {
        first = 0;
        dsc = 2;
    }

    (first, dsc)
}

fn generate_second_digit(
    cnh: &[u16],
    dsc: u16,
) -> u16 {
    let mut sum: u16 = 0;

    for i in 1..=9 {
        sum += cnh[i - 1] * (i as u16);
    }

    let mut second: i16 = ((sum % 11) as i16) - (dsc as i16);

    if second < 0 {
        second += 11;
    } else if second >= 10 {
        second = 0;
    }

    second as u16
}

pub fn is_bare(cnh: &str) -> bool {
    cnh.chars().count() == 11 && get_digits(cnh).len() == 11
}

pub fn is_masked(cnh: &str) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(cnh);

    if symbols.len() != 3 {
        return false;
    }

    symbols[0] == (3, ' ') && symbols[1] == (7, ' ') && symbols[2] == (11, ' ')
}

pub fn mask(cnh: &str) -> String {
    if !is_bare(cnh) {
        panic!("The given string cannot be masked as CNH!")
    }

    format!(
        "{} {} {} {}",
        &cnh[0..3],
        &cnh[3..6],
        &cnh[6..9],
        &cnh[9..11],
    )
}
