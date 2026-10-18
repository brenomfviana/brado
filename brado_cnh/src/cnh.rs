//! Estrutura que representa o documento CNH.

use brado_core::Document;
use brado_core::common::{
    get_digits, get_symbols, is_repeated, random_decimal_vector, to_decimal,
};

const CNH_SIZE: usize = 11;

/// Marcador que representa o documento CNH, implementando o trait
/// [`Document`](brado_core::Document).
///
/// ## Exemplos
///
/// ```
/// use brado_core::Document;
/// use brado_cnh::Cnh;
///
/// fn info<D: Document>() -> (&'static str, usize) {
///     (D::NAME, D::SIZE)
/// }
///
/// assert_eq!(info::<Cnh>(), ("CNH", 11));
/// assert!(Cnh::validate("84718735264"));
/// ```
pub struct Cnh;

impl Document for Cnh {
    const NAME: &'static str = "CNH";
    const SIZE: usize = CNH_SIZE;

    fn validate(doc: &str) -> bool {
        let size: usize = doc.chars().count();

        if size != CNH_SIZE && !Self::is_masked(doc) {
            return false;
        }

        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if digits.len() != CNH_SIZE || is_repeated(&digits) {
            return false;
        }

        let (d10, d11): (u16, u16) = generate_digits(&digits[..9]);

        (d10, d11) == (digits[9], digits[10])
    }

    fn is_bare(doc: &str) -> bool {
        doc.chars().count() == CNH_SIZE
            && get_digits(doc, to_decimal).len() == CNH_SIZE
    }

    fn is_masked(doc: &str) -> bool {
        let symbols: Vec<(usize, char)> = get_symbols(doc, to_decimal);
        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if symbols.len() != 3 || digits.len() != CNH_SIZE {
            return false;
        }

        symbols[0] == (3, ' ')
            && symbols[1] == (7, ' ')
            && symbols[2] == (11, ' ')
    }

    fn mask(doc: &str) -> Result<String, &'static str> {
        if !Self::is_bare(doc) {
            return Err("The given string cannot be masked as CNH!");
        }

        let masked_doc: String = format!(
            "{} {} {} {}",
            &doc[0..3],
            &doc[3..6],
            &doc[6..9],
            &doc[9..11],
        );

        Ok(masked_doc)
    }

    fn generate() -> String {
        let mut cnh: Vec<u16> = random_decimal_vector(9);
        let (d10, dsc): (u16, u16) = generate_first_digit(&cnh);
        cnh.push(d10);
        let d11: u16 = generate_second_digit(&cnh, dsc);
        cnh.push(d11);

        cnh.iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .concat()
    }

    fn generate_masked() -> String {
        Self::mask(&Self::generate()).expect("Invalid CNH!")
    }
}

fn generate_digits(doc_slice: &[u16]) -> (u16, u16) {
    let (d10, dsc): (u16, u16) = generate_first_digit(doc_slice);
    let d11: u16 = generate_second_digit(doc_slice, dsc);

    (d10, d11)
}

fn generate_first_digit(doc_slice: &[u16]) -> (u16, u16) {
    let sum: u16 = doc_slice
        .iter()
        .enumerate()
        .map(|(i, x)| x * (9 - i) as u16)
        .sum();

    let rest: u16 = sum % 11;

    match rest >= 10 {
        true => (0, 2),
        false => (rest, 0),
    }
}

fn generate_second_digit(
    doc_slice: &[u16],
    dsc: u16,
) -> u16 {
    let mut sum: u16 = 0;

    for i in 1..=9 {
        sum += doc_slice[i - 1] * (i as u16);
    }

    let rest: u16 = sum % 11;
    let second: u16 = match rest >= dsc {
        true => rest - dsc,
        false => 11 + rest - dsc,
    };

    match second >= 10 {
        true => 0,
        false => second,
    }
}
