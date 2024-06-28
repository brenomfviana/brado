//! Utilitários para validação de Registro Nacional de Veículos Automotores
//! (RENAVAM).

use crate::common::{get_digits, get_symbols, random_digit_vector};

/// Realiza validação de RENAVAM, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um RENAVAM válido, caso contrário,
/// retorna `false`.
///
/// ## Exemplos
///
/// RENAVAMs válidos:
/// ```
/// use brado::renavam;
///
/// let result = renavam::validate("79072338363"); // true
/// assert!(result);
///
/// let result = renavam::validate("7907233836-3"); // true
/// assert!(result);
/// ```
///
/// RENAVAMs inválidos:
/// ```
/// use brado::renavam;
///
/// let result = renavam::validate("79072338362"); // false
/// assert!(!result);
///
/// let result = renavam::validate("7907233836-2"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    let size: usize = doc.chars().count();

    if size != 11 && !is_masked(doc) {
        return false;
    }

    let digits: Vec<u16> = get_digits(doc);

    if digits.len() != 11 {
        return false;
    }

    let d11: u16 = generate_digit(&digits[..10]);

    d11 == digits[10]
}

fn generate_digit(doc_slice: &[u16]) -> u16 {
    let multipliers = [3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let sum: u16 = doc_slice
        .iter()
        .enumerate()
        .map(|(i, x)| x * multipliers[i])
        .sum();

    let rest = (sum * 10) % 11;

    if rest == 10 {
        0
    } else {
        rest
    }
}

/// Verifica se o argumento `doc` pode ser um RENAVAM sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// RENAVAMs válidos:
/// ```
/// use brado::renavam;
///
/// let result = renavam::is_bare("79072338363"); // true
/// assert!(result);
///
/// let result = renavam::is_bare("7907233836-3"); // false
/// assert!(!result);
/// ```
///
/// RENAVAMs inválidos:
/// ```
/// use brado::renavam;
///
/// let result = renavam::is_bare("79072338362"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    doc.chars().count() == 11 && get_digits(doc).len() == 11
}

/// Verifica se o argumento `doc` pode ser um RENAVAM com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// RENAVAMs válidos:
/// ```
/// use brado::renavam;
///
/// let result = renavam::is_masked("7907233836-3"); // true
/// assert!(result);
///
/// let result = renavam::is_masked("79072338363"); // false
/// assert!(!result);
/// ```
///
/// RENAVAMs inválidos:
/// ```
/// use brado::renavam;
///
/// let result = renavam::is_masked("7907233836-2"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(doc);
    let digits: Vec<u16> = get_digits(doc);

    if symbols.len() != 1 || digits.len() != 11 {
        return false;
    }

    symbols[0] == (10, '-')
}

/// Aplica máscara de RENAVAM no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 11 dígitos sem máscara:
/// ```
/// use brado::renavam;
///
/// let result = match renavam::mask("79072338363") { // Ok("7907233836-3")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(renavam::is_masked(&result)); // true
/// ```
///
/// Documento de 11 dígitos com máscara:
/// ```should_panic
/// use brado::renavam;
///
/// let result = match renavam::mask("7907233836-3") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    if !is_bare(doc) {
        return Err("The given string cannot be masked as RENAVAM!");
    }

    let masked_doc = format!("{}-{}", &doc[0..10], &doc[10..11]);

    Ok(masked_doc)
}

/// Gera e retorna um RENAVAM aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// use brado::renavam;
///
/// let result = renavam::generate(); // "79072338363"
/// assert!(renavam::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    let mut renavam: Vec<u16> = random_digit_vector(10);
    renavam.push(generate_digit(&renavam));

    renavam
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// Gera e retorna um RENAVAM aleatório com máscara.
///
/// ## Exemplo
/// ```
/// use brado::renavam;
///
/// let result = renavam::generate_masked(); // "7907233836-3"
/// assert!(renavam::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    mask(&generate()).expect("Valid RENAVAM!")
}
