//! Estrutura que representa o documento Título Eleitoral.

use brado_core::Document;
use brado_core::common::{
    get_digits, get_symbols, random_decimal_vector, to_decimal,
};
use rand::Rng;

const ELEITORAL_SIZE: usize = 12;

/// Marcador que representa o documento Título Eleitoral, implementando o
/// trait [`Document`](brado_core::Document).
///
/// ## Exemplos
///
/// ```
/// use brado_core::Document;
/// use brado_eleitoral::Eleitoral;
///
/// fn info<D: Document>() -> (&'static str, usize) {
///     (D::NAME, D::SIZE)
/// }
///
/// assert_eq!(info::<Eleitoral>(), ("Título Eleitoral", 12));
/// assert!(Eleitoral::validate("773537801651"));
/// ```
pub struct Eleitoral;

impl Document for Eleitoral {
    const NAME: &'static str = "Título Eleitoral";
    const SIZE: usize = ELEITORAL_SIZE;

    fn validate(doc: &str) -> bool {
        let size: usize = doc.chars().count();

        if size != ELEITORAL_SIZE && !Self::is_masked(doc) {
            return false;
        }

        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if digits.len() != ELEITORAL_SIZE {
            return false;
        }

        let (d11, d12): (u16, u16) = generate_digits(&digits);

        (d11, d12) == (digits[10], digits[11])
    }

    fn is_bare(doc: &str) -> bool {
        doc.chars().count() == ELEITORAL_SIZE
            && get_digits(doc, to_decimal).len() == ELEITORAL_SIZE
    }

    fn is_masked(doc: &str) -> bool {
        let symbols: Vec<(usize, char)> = get_symbols(doc, to_decimal);
        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if symbols.len() != 2 || digits.len() != ELEITORAL_SIZE {
            return false;
        }

        symbols[0] == (4, ' ') && symbols[1] == (9, ' ')
    }

    fn mask(doc: &str) -> Result<String, &'static str> {
        if !Self::is_bare(doc) {
            return Err(
                "The given string cannot be masked as Título Eleitoral!",
            );
        }

        let masked_doc: String =
            format!("{} {} {}", &doc[0..4], &doc[4..8], &doc[8..12]);

        Ok(masked_doc)
    }

    fn generate() -> String {
        let mut eleitoral: Vec<u16> = random_decimal_vector(8);
        eleitoral.extend(generate_state_identifier());

        let d11: u16 = generate_first_digit(&eleitoral[0..8]);
        eleitoral.push(d11);
        let d12: u16 = generate_second_digit(&eleitoral[8..10], d11);
        eleitoral.push(d12);

        eleitoral
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .concat()
    }

    fn generate_masked() -> String {
        Self::mask(&Self::generate()).expect("Invalid Título Eleitoral!")
    }
}

fn generate_digits(doc_slice: &[u16]) -> (u16, u16) {
    let d11: u16 = generate_first_digit(&doc_slice[0..8]);
    let d12: u16 = generate_second_digit(&doc_slice[8..10], d11);

    (d11, d12)
}

fn generate_first_digit(doc_slice: &[u16]) -> u16 {
    let multipliers: Vec<u16> = (2..10).collect();

    let sum: u16 = doc_slice
        .iter()
        .enumerate()
        .map(|(i, x)| x * multipliers[i])
        .sum();

    let rest: u16 = sum % 11;

    match rest {
        10 => 0,
        _ => rest,
    }
}

fn generate_second_digit(
    doc_slice: &[u16],
    first_digit: u16,
) -> u16 {
    let multipliers: Vec<u16> = (7..10).collect();

    let mut doc_slice: Vec<u16> = doc_slice.to_vec();
    doc_slice.extend(vec![first_digit]);

    let sum: u16 = doc_slice
        .iter()
        .enumerate()
        .map(|(i, x)| x * multipliers[i])
        .sum();

    let rest: u16 = sum % 11;

    match rest {
        10 => 0,
        _ => rest,
    }
}

fn generate_state_identifier() -> Vec<u16> {
    let mut rng = rand::thread_rng();

    vec![rng.gen_range(0..2), rng.gen_range(0..10)]
}
