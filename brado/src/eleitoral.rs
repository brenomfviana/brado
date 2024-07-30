//! Utilitários para validação de Título Eleitoral.

use crate::common::{get_digits, get_symbols, random_digit_vector, to_decimal};
use rand::Rng;

const ELEITORAL_SIZE: usize = 12;

/// Realiza validação de Título Eleitoral, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um Título Eleitoral válido,
/// caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Títulos Eleitorais válidos:
/// ```
/// use brado::eleitoral;
///
/// let result = eleitoral::validate("773537801651"); // true
/// assert!(result);
///
/// let result = eleitoral::validate("7735 3780 1651"); // true
/// assert!(result);
/// ```
///
/// Títulos Eleitorais inválidos:
/// ```
/// use brado::eleitoral;
///
/// let result = eleitoral::validate("773537801650"); // false
/// assert!(!result);
///
/// let result = eleitoral::validate("7735 3780 1650"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    let size: usize = doc.chars().count();

    if size != ELEITORAL_SIZE && !is_masked(doc) {
        return false;
    }

    let digits: Vec<u16> = get_digits(doc, &to_decimal);

    if digits.len() != ELEITORAL_SIZE {
        return false;
    }

    let (d11, d12): (u16, u16) = generate_digits(&digits);

    d11 == digits[10] && d12 == digits[11]
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

/// Verifica se o argumento `doc` pode ser um Título Eleitoral sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Títulos Eleitorais válidos:
/// ```
/// use brado::eleitoral;
///
/// let result = eleitoral::is_bare("773537801651"); // true
/// assert!(result);
///
/// let result = eleitoral::is_bare("7735 3780 1651"); // false
/// assert!(!result);
/// ```
///
/// Títulos Eleitorais inválidos:
/// ```
/// use brado::eleitoral;
///
/// let result = eleitoral::is_bare("773537801650"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    doc.chars().count() == ELEITORAL_SIZE
        && get_digits(doc, &to_decimal).len() == ELEITORAL_SIZE
}

/// Verifica se o argumento `doc` pode ser um Título Eleitoral com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Títulos Eleitorais válidos:
/// ```
/// use brado::eleitoral;
///
/// let result = eleitoral::is_masked("7735 3780 1651"); // true
/// assert!(result);
///
/// let result = eleitoral::is_masked("773537801651"); // false
/// assert!(!result);
/// ```
///
/// Títulos Eleitorais inválidos:
/// ```
/// use brado::eleitoral;
///
/// let result = eleitoral::is_masked("7735 3780 1650"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(doc, &to_decimal);
    let digits: Vec<u16> = get_digits(doc, &to_decimal);

    if symbols.len() != 2 || digits.len() != ELEITORAL_SIZE {
        return false;
    }

    symbols[0] == (4, ' ') && symbols[1] == (9, ' ')
}

/// Aplica máscara de Título Eleitoral no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 12 dígitos sem máscara:
/// ```
/// use brado::eleitoral;
///
/// let result = match eleitoral::mask("773537801651") { // Ok("7735 3780 1651")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(eleitoral::is_masked(&result)); // true
/// ```
///
/// Documento de 12 dígitos com máscara:
/// ```should_panic
/// use brado::eleitoral;
///
/// let result = match eleitoral::mask("7735 3780 1651") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    if !is_bare(doc) {
        return Err("The given string cannot be masked as Título Eleitoral!");
    }

    let masked_doc: String =
        format!("{} {} {}", &doc[0..4], &doc[4..8], &doc[8..12]);

    Ok(masked_doc)
}

/// Gera e retorna um Título Eleitoral aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// use brado::eleitoral;
///
/// let result = eleitoral::generate(); // "773537801651"
/// assert!(eleitoral::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    let mut eleitoral: Vec<u16> = random_digit_vector(8);
    eleitoral.extend(generate_state_identifier());

    let d11: u16 = generate_first_digit(&eleitoral[0..8]);
    eleitoral.push(d11);
    let d12: u16 = generate_second_digit(&eleitoral[8..10], d11);
    eleitoral.push(d12);

    eleitoral
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn generate_state_identifier() -> Vec<u16> {
    let mut rng = rand::thread_rng();

    vec![rng.gen_range(0..2), rng.gen_range(0..10)]
}

/// Gera e retorna um Título Eleitoral aleatório com máscara.
///
/// ## Exemplo
/// ```
/// use brado::eleitoral;
///
/// let result = eleitoral::generate_masked(); // "7735 3780 1651"
/// assert!(eleitoral::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    mask(&generate()).expect("Invalid Título Eleitoral!")
}
