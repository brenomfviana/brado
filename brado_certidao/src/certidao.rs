//! Estrutura que representa o documento Certidão.

use brado_core::Document;
use brado_core::common::{
    get_digits, get_symbols, random_decimal_vector, to_decimal,
};

const CERTIDAO_SIZE: usize = 32;

/// Marcador que representa o documento Certidão (Nascimento, Casamento e
/// Óbito), implementando o trait [`Document`](brado_core::Document).
///
/// ## Exemplos
///
/// ```
/// use brado_core::Document;
/// use brado_certidao::Certidao;
///
/// fn info<D: Document>() -> (&'static str, usize) {
///     (D::NAME, D::SIZE)
/// }
///
/// assert_eq!(info::<Certidao>(), ("Certidão", 32));
/// assert!(Certidao::validate("21924201552023106304243115818536"));
/// ```
pub struct Certidao;

impl Document for Certidao {
    const NAME: &'static str = "Certidão";
    const SIZE: usize = CERTIDAO_SIZE;

    fn validate(doc: &str) -> bool {
        let size: usize = doc.chars().count();

        if size != CERTIDAO_SIZE && !Self::is_masked(doc) {
            return false;
        }

        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if digits.len() != CERTIDAO_SIZE {
            return false;
        }

        let (d30, d31): (u16, u16) = generate_digits(&digits[..30]);

        (d30, d31) == (digits[30], digits[31])
    }

    fn is_bare(doc: &str) -> bool {
        doc.chars().count() == CERTIDAO_SIZE
            && get_digits(doc, to_decimal).len() == CERTIDAO_SIZE
    }

    fn is_masked(doc: &str) -> bool {
        let symbols: Vec<(usize, char)> = get_symbols(doc, to_decimal);
        let digits: Vec<u16> = get_digits(doc, to_decimal);

        if symbols.len() != 8 || digits.len() != CERTIDAO_SIZE {
            return false;
        }

        symbols[0] == (6, ' ')
            && symbols[1] == (9, ' ')
            && symbols[2] == (12, ' ')
            && symbols[3] == (17, ' ')
            && symbols[4] == (19, ' ')
            && symbols[5] == (25, ' ')
            && symbols[6] == (29, ' ')
            && symbols[7] == (37, '-')
    }

    fn mask(doc: &str) -> Result<String, &'static str> {
        if !Self::is_bare(doc) {
            return Err("The given string cannot be masked as Certidão!");
        }

        let masked_doc: String = format!(
            "{} {} {} {} {} {} {} {}-{}",
            &doc[0..6],
            &doc[6..8],
            &doc[8..10],
            &doc[10..14],
            &doc[14..15],
            &doc[15..20],
            &doc[20..23],
            &doc[23..30],
            &doc[30..32],
        );

        Ok(masked_doc)
    }

    fn generate() -> String {
        let mut certidao: Vec<u16> = random_decimal_vector(30);
        certidao.push(generate_digit(&certidao));
        certidao.push(generate_digit(&certidao));

        certidao
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .concat()
    }

    fn generate_masked() -> String {
        Self::mask(&Self::generate()).expect("Invalid Certidão!")
    }
}

fn generate_digits(doc_slice: &[u16]) -> (u16, u16) {
    let d30: u16 = generate_digit(doc_slice);
    let d31: u16 = generate_digit(&[doc_slice, &[d30]].concat());

    (d30, d31)
}

fn generate_digit(doc_slice: &[u16]) -> u16 {
    let mut multiplier: u16 = 32 - (doc_slice.len() as u16);

    let d: u16 = doc_slice
        .iter()
        .map(|x| {
            let result: u16 = x * multiplier;

            multiplier += 1;
            multiplier = match multiplier > 10 {
                true => 0,
                false => multiplier,
            };

            result
        })
        .sum();

    let rest: u16 = d % 11;

    match rest > 9 {
        true => 1,
        false => rest,
    }
}
