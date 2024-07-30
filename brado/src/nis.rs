//! Utilitários para validação de NIS/NIT/PIS/PASEP.
//!
//! NIS: Número de Identificação Social;
//! NIT: Número de Inscrição do Trabalhador;
//! PIS: Programa de Integração Social;
//! PASEP: Programa de Formação do Patrimônio do Servidor Público.

use crate::common::{
    get_digits, get_symbols, is_repeated, random_digit_vector, to_decimal,
};

const NIS_SIZE: usize = 11;

/// Realiza validação de NIS/NIT/PIS/PASEP, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um NIS/NIT/PIS/PASEP válido,
/// caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Números NIS/NIT/PIS/PASEP válidos:
/// ```
/// use brado::nis;
///
/// let result = nis::validate("40865658047"); // true
/// assert!(result);
///
/// let result = nis::validate("408.65658.04-7"); // true
/// assert!(result);
/// ```
///
/// Números NIS/NIT/PIS/PASEP inválidos:
/// ```
/// use brado::nis;
///
/// let result = nis::validate("40865658046"); // false
/// assert!(!result);
///
/// let result = nis::validate("408.65658.04-6"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    let size: usize = doc.chars().count();

    if size != NIS_SIZE && !is_masked(doc) {
        return false;
    }

    let digits: Vec<u16> = get_digits(doc, to_decimal);

    if digits.len() != NIS_SIZE || is_repeated(&digits) {
        return false;
    }

    let d11: u16 = generate_digit(&digits[..10]);

    d11 == digits[10]
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

/// Verifica se o argumento `doc` pode ser um NIS/NIT/PIS/PASEP sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Números NIS/NIT/PIS/PASEP válidos:
/// ```
/// use brado::nis;
///
/// let result = nis::is_bare("40865658047"); // true
/// assert!(result);
///
/// let result = nis::is_bare("408.65658.04-7"); // false
/// assert!(!result);
/// ```
///
/// Números NIS/NIT/PIS/PASEP inválidos:
/// ```
/// use brado::nis;
///
/// let result = nis::is_bare("40865658046"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    doc.chars().count() == NIS_SIZE
        && get_digits(doc, to_decimal).len() == NIS_SIZE
}

/// Verifica se o argumento `doc` pode ser um NIS/NIT/PIS/PASEP com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Números NIS/NIT/PIS/PASEP válidos:
/// ```
/// use brado::nis;
///
/// let result = nis::is_masked("408.65658.04-7"); // true
/// assert!(result);
///
/// let result = nis::is_masked("40865658047"); // false
/// assert!(!result);
/// ```
///
/// Números NIS/NIT/PIS/PASEP inválidos:
/// ```
/// use brado::nis;
///
/// let result = nis::is_masked("408.65658.04-6"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(doc, to_decimal);
    let digits: Vec<u16> = get_digits(doc, to_decimal);

    if symbols.len() != 3 || digits.len() != NIS_SIZE {
        return false;
    }

    symbols[0] == (3, '.') && symbols[1] == (9, '.') && symbols[2] == (12, '-')
}

/// Aplica máscara de NIS/NIT/PIS/PASEP no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 11 dígitos sem máscara:
/// ```
/// use brado::nis;
///
/// let result = match nis::mask("40865658047") { // Ok("408.65658.04-7")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(nis::is_masked(&result)); // true
/// ```
///
/// Documento de 11 dígitos com máscara:
/// ```should_panic
/// use brado::nis;
///
/// let result = match nis::mask("408.65658.04-7") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    if !is_bare(doc) {
        return Err("The given string cannot be masked as NIS/NIT/PIS/PASEP!");
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

/// Gera e retorna um NIS/NIT/PIS/PASEP aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// use brado::nis;
///
/// let result = nis::generate(); // "40865658047"
/// assert!(nis::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    let mut nis: Vec<u16> = random_digit_vector(10);
    nis.push(generate_digit(&nis));

    nis.iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// Gera e retorna um NIS/NIT/PIS/PASEP aleatório com máscara.
///
/// ## Exemplo
/// ```
/// use brado::nis;
///
/// let result = nis::generate_masked(); // "408.65658.04-7"
/// assert!(nis::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    mask(&generate()).expect("Invalid NIS/NIT/PIS/PASEP!")
}
