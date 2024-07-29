//! Utilitários para validação de Cadastro Nacional de Pessoa Jurídica (CNPJ).

use crate::common::{get_digits, get_symbols, random_digit_vector, to_decimal};

fn to_cnpj_digit(
    i: usize,
    c: char,
) -> Option<u16> {
    let n = c as u16;
    match n >= 48 && i < 12 {
        true => Some(n - 48),
        false => to_decimal(i, c),
    }
}

/// Realiza validação de CNPJ, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um CNPJ válido, caso contrário,
/// retorna `false`.
///
/// ## Exemplos
///
/// CNPJs válidos:
/// ```
/// use brado::cnpj;
///
/// let result = cnpj::validate("05200851000100"); // true
/// assert!(result);
///
/// let result = cnpj::validate("05.200.851/0001-00"); // true
/// assert!(result);
/// ```
///
/// CNPJs inválidos:
/// ```
/// use brado::cnpj;
///
/// let result = cnpj::validate("05200851000101"); // false
/// assert!(!result);
///
/// let result = cnpj::validate("05.200.851/0001-01"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    let size: usize = doc.chars().count();

    if size != 14 && !is_masked(doc) {
        return false;
    }

    let digits: Vec<u16> = get_digits(doc, Box::new(to_cnpj_digit));

    if digits.len() != 14 {
        return false;
    }

    for i in 0..10 {
        if digits.iter().filter(|&n| *n == i).count() == 14 {
            return false;
        }
    }

    let (d13, d14): (u16, u16) = generate_digits(&digits[..12]);

    (d13, d14) == (digits[12], digits[13])
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

    if rest < 2 {
        0
    } else {
        11 - rest
    }
}

/// Verifica se o argumento `doc` pode ser um CNPJ sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNPJs válidos:
/// ```
/// use brado::cnpj;
///
/// let result = cnpj::is_bare("05200851000100"); // true
/// assert!(result);
///
/// let result = cnpj::is_bare("05.200.851/0001-00"); // false
/// assert!(!result);
/// ```
///
/// CNPJs inválidos:
/// ```
/// use brado::cnpj;
///
/// let result = cnpj::is_bare("05200851000101"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    doc.chars().count() == 14
        && get_digits(doc, Box::new(to_cnpj_digit)).len() == 14
}

/// Verifica se o argumento `doc` pode ser um CNPJ com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNPJs válidos:
/// ```
/// use brado::cnpj;
///
/// let result = cnpj::is_masked("05.200.851/0001-00"); // true
/// assert!(result);
///
/// let result = cnpj::is_masked("05200851000100"); // false
/// assert!(!result);
/// ```
///
/// CNPJs inválidos:
/// ```
/// use brado::cnpj;
///
/// let result = cnpj::is_masked("05.200.851/0001-01"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(doc, Box::new(to_cnpj_digit));
    let digits: Vec<u16> = get_digits(doc, Box::new(to_cnpj_digit));

    if symbols.len() != 4 || digits.len() != 14 {
        return false;
    }

    symbols[0] == (2, '.')
        && symbols[1] == (6, '.')
        && symbols[2] == (10, '/')
        && symbols[3] == (15, '-')
}

/// Aplica máscara de CNPJ no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 14 dígitos sem máscara:
/// ```
/// use brado::cnpj;
///
/// let result = match cnpj::mask("05200851000100") { // Ok("05.200.851/0001-00")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(cnpj::is_masked(&result)); // true
/// ```
///
/// Documento de 14 dígitos com máscara:
/// ```should_panic
/// use brado::cnpj;
///
/// let result = match cnpj::mask("05.200.851/0001-00") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    if !is_bare(doc) {
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

/// Gera e retorna um CNPJ aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// use brado::cnpj;
///
/// let result = cnpj::generate(); // "05200851000100"
/// assert!(cnpj::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    let cnpj: Vec<u16> = random_digit_vector(12);
    let (d13, d14): (u16, u16) = generate_digits(&cnpj);
    let cnpj: Vec<u16> = [cnpj, vec![d13, d14]].concat();

    cnpj.iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// Gera e retorna um CNPJ aleatório com máscara.
///
/// ## Exemplo
/// ```
/// use brado::cnpj;
///
/// let result = cnpj::generate_masked(); // "05.200.851/0001-00"
/// assert!(cnpj::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    mask(&generate()).expect("Valid CNPJ!")
}
