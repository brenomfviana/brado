//! Estrutura que representa o documento CPF.

use brado_core::Document;
use brado_core::common::{
    get_digits, get_symbols, is_repeated, random_decimal_vector, to_decimal,
};

const CPF_SIZE: usize = 11;

/// Marcador que representa o documento CPF, implementando o trait
/// [`Document`](brado_core::Document).
///
/// ## Exemplos
///
/// ```
/// use brado_core::Document;
/// use brado_cpf::Cpf;
///
/// fn info<D: Document>() -> (&'static str, usize) {
///     (D::NAME, D::SIZE)
/// }
///
/// assert_eq!(info::<Cpf>(), ("CPF", 11));
/// assert!(Cpf::validate("639.292.470-11"));
/// ```
pub struct Cpf;

impl Document for Cpf {
    const NAME: &'static str = "CPF";
    const SIZE: usize = CPF_SIZE;

    fn validate(doc: &str) -> bool {
        let size: usize = doc.chars().count();

        if size != CPF_SIZE && !Self::is_masked(doc) {
            return false;
        }

        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if digits.len() != CPF_SIZE || is_repeated(&digits) {
            return false;
        }

        let (d10, d11): (u16, u16) = generate_digits(&digits[..9]);

        (d10, d11) == (digits[9], digits[10])
    }

    fn is_bare(doc: &str) -> bool {
        doc.chars().count() == CPF_SIZE
            && get_digits(doc, to_decimal).len() == CPF_SIZE
    }

    fn is_masked(doc: &str) -> bool {
        let symbols: Vec<(usize, char)> = get_symbols(doc, to_decimal);
        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if symbols.len() != 3 || digits.len() != CPF_SIZE {
            return false;
        }

        symbols[0] == (3, '.')
            && symbols[1] == (7, '.')
            && symbols[2] == (11, '-')
    }

    fn mask(doc: &str) -> Result<String, &'static str> {
        if !Self::is_bare(doc) {
            return Err("The given string cannot be masked as CPF!");
        }

        let masked_doc: String = format!(
            "{}.{}.{}-{}",
            &doc[0..3],
            &doc[3..6],
            &doc[6..9],
            &doc[9..11],
        );

        Ok(masked_doc)
    }

    fn generate() -> String {
        let mut cpf: Vec<u16> = random_decimal_vector(9);
        cpf.push(generate_digit(&cpf));
        cpf.push(generate_digit(&cpf));

        cpf.iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .concat()
    }

    fn generate_masked() -> String {
        Self::mask(&Self::generate()).expect("Invalid CPF!")
    }
}

fn generate_digits(doc_slice: &[u16]) -> (u16, u16) {
    let d10: u16 = generate_digit(doc_slice);
    let d11: u16 = generate_digit(&[doc_slice, &[d10]].concat());

    (d10, d11)
}

fn generate_digit(doc_slice: &[u16]) -> u16 {
    let max: usize = doc_slice.len() + 1;

    let sum: u16 = doc_slice
        .iter()
        .enumerate()
        .map(|(i, d)| d * (max - i) as u16)
        .sum();

    let rest: u16 = (sum * 10) % 11;

    match rest {
        10 => 0,
        _ => rest,
    }
}
