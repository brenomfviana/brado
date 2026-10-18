//! Estrutura que representa o documento NIS/NIT/PIS/PASEP.

use brado_core::Document;
use brado_core::common::{
    get_digits, get_symbols, is_repeated, random_decimal_vector, to_decimal,
};

const NIS_SIZE: usize = 11;

/// Marcador que representa o documento NIS/NIT/PIS/PASEP, implementando o
/// trait [`Document`](brado_core::Document).
///
/// ## Exemplos
///
/// ```
/// use brado_core::Document;
/// use brado_nis::Nis;
///
/// fn info<D: Document>() -> (&'static str, usize) {
///     (D::NAME, D::SIZE)
/// }
///
/// assert_eq!(info::<Nis>(), ("NIS", 11));
/// assert!(Nis::validate("408.65658.04-7"));
/// ```
pub struct Nis;

impl Document for Nis {
    const NAME: &'static str = "NIS";
    const SIZE: usize = NIS_SIZE;

    fn validate(doc: &str) -> bool {
        let size: usize = doc.chars().count();

        if size != NIS_SIZE && !Self::is_masked(doc) {
            return false;
        }

        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if digits.len() != NIS_SIZE || is_repeated(&digits) {
            return false;
        }

        let d11: u16 = generate_digit(&digits[..10]);

        d11 == digits[10]
    }

    fn is_bare(doc: &str) -> bool {
        doc.chars().count() == NIS_SIZE
            && get_digits(doc, to_decimal).len() == NIS_SIZE
    }

    fn is_masked(doc: &str) -> bool {
        let symbols: Vec<(usize, char)> = get_symbols(doc, to_decimal);
        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if symbols.len() != 3 || digits.len() != NIS_SIZE {
            return false;
        }

        symbols[0] == (3, '.')
            && symbols[1] == (9, '.')
            && symbols[2] == (12, '-')
    }

    fn mask(doc: &str) -> Result<String, &'static str> {
        if !Self::is_bare(doc) {
            return Err(
                "The given string cannot be masked as NIS/NIT/PIS/PASEP!",
            );
        }

        let masked_doc: String = format!(
            "{}.{}.{}-{}",
            &doc[0..3],
            &doc[3..8],
            &doc[8..10],
            &doc[10..11],
        );

        Ok(masked_doc)
    }

    fn generate() -> String {
        let mut nis: Vec<u16> = random_decimal_vector(10);
        nis.push(generate_digit(&nis));

        nis.iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .concat()
    }

    fn generate_masked() -> String {
        Self::mask(&Self::generate()).expect("Invalid NIS/NIT/PIS/PASEP!")
    }
}

fn generate_digit(doc_slice: &[u16]) -> u16 {
    let multipliers: Vec<u16> = vec![3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let sum: u16 = doc_slice
        .iter()
        .enumerate()
        .map(|(i, x)| x * multipliers[i])
        .sum();

    let rest: u16 = sum % 11;

    match rest >= 2 {
        true => 11 - rest,
        false => 0,
    }
}
