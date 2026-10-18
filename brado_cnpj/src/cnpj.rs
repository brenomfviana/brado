//! Estrutura que representa o documento CNPJ.

use brado_core::Document;
use brado_core::common::{
    get_digits, get_symbols, is_repeated, random_string_from_alphabet,
    to_decimal,
};

const CNPJ_SIZE: usize = 14;

/// Marcador que representa o documento CNPJ, implementando o trait
/// [`Document`](brado_core::Document).
///
/// ## Exemplos
///
/// ```
/// use brado_core::Document;
/// use brado_cnpj::Cnpj;
///
/// fn info<D: Document>() -> (&'static str, usize) {
///     (D::NAME, D::SIZE)
/// }
///
/// assert_eq!(info::<Cnpj>(), ("CNPJ", 14));
/// assert!(Cnpj::validate("05.200.851/0001-00"));
/// ```
pub struct Cnpj;

impl Document for Cnpj {
    const NAME: &'static str = "CNPJ";
    const SIZE: usize = CNPJ_SIZE;

    fn validate(doc: &str) -> bool {
        let size: usize = doc.chars().count();

        if size != CNPJ_SIZE && !Self::is_masked(doc) {
            return false;
        }

        let digits: Vec<u16> = get_digits(doc, to_cnpj_digit);

        if digits.len() != CNPJ_SIZE || is_repeated(&digits) {
            return false;
        }

        let (d13, d14): (u16, u16) = generate_digits(&digits[..12]);

        (d13, d14) == (digits[12], digits[13])
    }

    fn is_bare(doc: &str) -> bool {
        doc.chars().count() == CNPJ_SIZE
            && get_digits(doc, to_cnpj_digit).len() == CNPJ_SIZE
    }

    fn is_masked(doc: &str) -> bool {
        let div: usize = doc.chars().count() - 2;
        let doc_slice1 = &doc[..div];
        let doc_slice2 = &doc[div..];

        let mut symbols: Vec<(usize, char)> =
            get_symbols(doc_slice1, to_cnpj_digit);

        for symbol in get_symbols(doc_slice2, to_decimal) {
            symbols.push(symbol);
        }

        let digits: Vec<u16> = get_digits(doc, to_cnpj_digit);

        if symbols.len() != 4 || digits.len() != CNPJ_SIZE {
            return false;
        }

        symbols[0] == (2, '.')
            && symbols[1] == (6, '.')
            && symbols[2] == (10, '/')
            && symbols[3] == (15, '-')
    }

    fn mask(doc: &str) -> Result<String, &'static str> {
        if !Self::is_bare(doc) {
            return Err("The given string cannot be masked as CNPJ!");
        }

        let masked_doc: String = format!(
            "{}.{}.{}/{}-{}",
            &doc[0..2],
            &doc[2..5],
            &doc[5..8],
            &doc[8..12],
            &doc[12..14],
        );

        Ok(masked_doc)
    }

    fn generate() -> String {
        let cnpj: String = random_string_from_alphabet(12, &alphabet());
        let digits: Vec<u16> = get_digits(&cnpj, to_cnpj_digit);
        let (d13, d14): (u16, u16) = generate_digits(&digits);

        [cnpj, d13.to_string(), d14.to_string()].concat()
    }

    fn generate_masked() -> String {
        Self::mask(&Self::generate()).expect("Invalid CNPJ!")
    }
}

/// Converte um caractere em um dígito válido de CNPJ.
///
/// CNPJ Alfanumérico: SS.SSS.SSS/SSSS-NN,
/// onde N: Número; S: Letra ou Número.
///
/// A validação permanece igual, porém, é necessário
/// substituir os caracteres pelos valores respectivos
/// da tabela ASCII e dele subtrair 48. Assim, '0'=0,
/// '1'=1, ..., 'A'=17, 'B'=18, ...
fn to_cnpj_digit(c: char) -> Option<u16> {
    let n = c as u16;
    match n >= 48 {
        true => Some(n - 48),
        false => to_decimal(c),
    }
}

fn generate_digits(doc_slice: &[u16]) -> (u16, u16) {
    let weights: Vec<u16> = vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let d13: u16 = generate_digit(doc_slice, weights);

    let doc_slice: Vec<u16> = [doc_slice, &[d13]].concat();

    let weights: Vec<u16> = vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let d14: u16 = generate_digit(&doc_slice, weights);

    (d13, d14)
}

fn generate_digit(
    doc_slice: &[u16],
    weights: Vec<u16>,
) -> u16 {
    let sum: u16 = doc_slice
        .iter()
        .enumerate()
        .map(|(i, x)| x * weights[i])
        .sum();

    let rest: u16 = sum % 11;

    match rest < 2 {
        true => 0,
        false => 11 - rest,
    }
}

fn alphabet() -> Vec<char> {
    vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D',
        'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
}
