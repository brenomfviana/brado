//! Utilitários para validação de Carteira Nacional de Habilitação (CNH).

use crate::common::{
    get_digits, get_symbols, is_repeated, random_digit_vector,
};

/// Realiza validação de CNH, máscarado ou não.
/// Retorna `true` se o argumento `doc` for uma CNH válida, caso contrário,
/// retorna `false`.
///
/// ## Exemplos
///
/// CNHs válidas:
/// ```
/// use brado::cnh;
///
/// let result = cnh::validate("84718735264"); // true
/// assert!(result);
///
/// let result = cnh::validate("847 187 352 64"); // true
/// assert!(result);
/// ```
///
/// CNHs inválidas:
/// ```
/// use brado::cnh;
///
/// let result = cnh::validate("84718735265"); // false
/// assert!(!result);
///
/// let result = cnh::validate("847 187 352 65"); // false
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

    if is_repeated(&digits) {
        return false;
    }

    let (d10, d11): (u16, u16) = generate_digits(&digits[..9]);

    (d10, d11) == (digits[9], digits[10])
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

    if rest >= 10 {
        (0, 2)
    } else {
        (rest, 0)
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

    let second: i16 = ((sum % 11) as i16) - (dsc as i16);

    if second >= 10 {
        0
    } else if second < 0 {
        (second + 11) as u16
    } else {
        second as u16
    }
}

/// Verifica se o argumento `doc` pode ser uma CNH sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNHs válidas:
/// ```
/// use brado::cnh;
///
/// let result = cnh::is_bare("84718735264"); // true
/// assert!(result);
///
/// let result = cnh::is_bare("847 187 352 64"); // false
/// assert!(!result);
/// ```
///
/// CNHs inválidas:
/// ```
/// use brado::cnh;
///
/// let result = cnh::is_bare("84718735265"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    doc.chars().count() == 11 && get_digits(doc).len() == 11
}

/// Verifica se o argumento `doc` pode ser uma CNH com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNHs válidas:
/// ```
/// use brado::cnh;
///
/// let result = cnh::is_masked("847 187 352 64"); // true
/// assert!(result);
///
/// let result = cnh::is_masked("84718735264"); // false
/// assert!(!result);
/// ```
///
/// CNHs inválidas:
/// ```
/// use brado::cnh;
///
/// let result = cnh::is_masked("847 187 352 65"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(doc);
    let digits: Vec<u16> = get_digits(doc);

    if symbols.len() != 3 || digits.len() != 11 {
        return false;
    }

    symbols[0] == (3, ' ') && symbols[1] == (7, ' ') && symbols[2] == (11, ' ')
}

/// Aplica máscara de CNH no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 11 dígitos sem máscara:
/// ```
/// use brado::cnh;
///
/// let result = match cnh::mask("84718735264") { // Ok("847 187 352 64")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(cnh::is_masked(&result)); // true
/// ```
///
/// Documento de 11 dígitos com máscara:
/// ```should_panic
/// use brado::cnh;
///
/// let result = match cnh::mask("847 187 352 64") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    if !is_bare(doc) {
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

/// Gera e retorna uma CNH aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// use brado::cnh;
///
/// let result = cnh::generate(); // "84718735264"
/// assert!(cnh::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    let mut cnh: Vec<u16> = random_digit_vector(9);
    let (d10, dsc): (u16, u16) = generate_first_digit(&cnh);
    cnh.push(d10);
    let d11: u16 = generate_second_digit(&cnh, dsc);
    cnh.push(d11);

    cnh.iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// Gera e retorna uma CNH aleatório com máscara.
///
/// ## Exemplo
/// ```
/// use brado::cnh;
///
/// let result = cnh::generate_masked(); // "847 187 352 64"
/// assert!(cnh::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    mask(&generate()).expect("Valid CNH!")
}
