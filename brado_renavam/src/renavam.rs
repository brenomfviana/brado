//! Estrutura que representa o documento RENAVAM.

use brado_core::Document;
use brado_core::common::{
    get_digits, get_symbols, random_decimal_vector, to_decimal,
};

const RENAVAM_SIZE: usize = 11;

/// Marcador que representa o documento RENAVAM, implementando o trait
/// [`Document`](brado_core::Document).
///
/// ## Exemplos
///
/// ```
/// use brado_core::Document;
/// use brado_renavam::Renavam;
///
/// fn info<D: Document>() -> (&'static str, usize) {
///     (D::NAME, D::SIZE)
/// }
///
/// assert_eq!(info::<Renavam>(), ("RENAVAM", 11));
/// assert!(Renavam::validate("79072338363"));
/// ```
pub struct Renavam;

impl Document for Renavam {
    const NAME: &'static str = "RENAVAM";
    const SIZE: usize = RENAVAM_SIZE;

    fn validate(doc: &str) -> bool {
        let size: usize = doc.chars().count();

        if size != RENAVAM_SIZE && !Self::is_masked(doc) {
            return false;
        }

        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if digits.len() != RENAVAM_SIZE {
            return false;
        }

        let d11: u16 = generate_digit(&digits[..10]);

        d11 == digits[10]
    }

    fn is_bare(doc: &str) -> bool {
        doc.chars().count() == RENAVAM_SIZE
            && get_digits(doc, to_decimal).len() == RENAVAM_SIZE
    }

    fn is_masked(doc: &str) -> bool {
        let symbols: Vec<(usize, char)> = get_symbols(doc, to_decimal);
        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if symbols.len() != 1 || digits.len() != RENAVAM_SIZE {
            return false;
        }

        symbols[0] == (10, '-')
    }

    fn mask(doc: &str) -> Result<String, &'static str> {
        if !Self::is_bare(doc) {
            return Err("The given string cannot be masked as RENAVAM!");
        }

        let masked_doc: String = format!("{}-{}", &doc[0..10], &doc[10..11]);

        Ok(masked_doc)
    }

    fn generate() -> String {
        let mut renavam: Vec<u16> = random_decimal_vector(10);
        renavam.push(generate_digit(&renavam));

        renavam
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .concat()
    }

    fn generate_masked() -> String {
        Self::mask(&Self::generate()).expect("Invalid RENAVAM!")
    }
}

fn generate_digit(doc_slice: &[u16]) -> u16 {
    let multipliers: [u16; 10] = [3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let sum: u16 = doc_slice
        .iter()
        .enumerate()
        .map(|(i, x)| x * multipliers[i])
        .sum();

    let rest: u16 = (sum * 10) % 11;

    match rest {
        10 => 0,
        _ => rest,
    }
}
