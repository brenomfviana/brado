//! Utilitários para validação de Cadastro de Pessoa Física (CPF).

use crate::common::{
    get_digits, get_symbols, is_repeated, random_digit_vector,
};

/// Realiza validação de CPF, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um CPF válido,
/// caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CPFs válidos:
/// ```
/// use brado::cpf;
///
/// let result = cpf::validate("63929247011"); // true
/// assert!(result);
///
/// let result = cpf::validate("639.292.470-11"); // true
/// assert!(result);
/// ```
///
/// CPFs inválidos:
/// ```
/// use brado::cpf;
///
/// let result = cpf::validate("63929247010"); // false
/// assert!(!result);
///
/// let result = cpf::validate("639.292.470-10"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    let size: usize = doc.chars().count();

    if size != 11 && !is_masked(doc) {
        return false;
    }

    let digits: Vec<u16> = get_digits(doc);

    if digits.len() != 11 || is_repeated(&digits) {
        return false;
    }

    let (d10, d11): (u16, u16) = generate_digits(&digits[..9]);

    (d10, d11) == (digits[9], digits[10])
}

fn generate_digits(doc_slice: &[u16]) -> (u16, u16) {
    let d10: u16 = generate_digit(doc_slice);
    let d11: u16 = generate_digit(&[doc_slice, &[d10]].concat());

    (d10, d11)
}

fn generate_digit(doc_slice: &[u16]) -> u16 {
    let max = doc_slice.len() + 1;

    let sum: u16 = doc_slice
        .iter()
        .enumerate()
        .map(|(i, d)| d * (max - i) as u16)
        .sum();

    let result = (sum * 10) % 11;

    if result == 10 {
        0
    } else {
        result
    }
}

/// Verifica se o argumento `doc` pode ser um CPF sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CPFs válidos:
/// ```
/// use brado::cpf;
///
/// let result = cpf::is_bare("63929247011"); // true
/// assert!(result);
///
/// let result = cpf::is_bare("639.292.470-11"); // false
/// assert!(!result);
/// ```
///
/// CPFs inválidos:
/// ```
/// use brado::cpf;
///
/// let result = cpf::is_bare("63929247010"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    doc.chars().count() == 11 && get_digits(doc).len() == 11
}

/// Verifica se o argumento `doc` pode ser um CPF com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CPFs válidos:
/// ```
/// use brado::cpf;
///
/// let result = cpf::is_masked("639.292.470-11"); // true
/// assert!(result);
///
/// let result = cpf::is_masked("63929247011"); // false
/// assert!(!result);
/// ```
///
/// CPFs inválidos:
/// ```
/// use brado::cpf;
///
/// let result = cpf::is_masked("639.292.470-10"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(doc);
    let digits: Vec<u16> = get_digits(doc);

    if symbols.len() != 3 || digits.len() != 11 {
        return false;
    }

    symbols[0] == (3, '.') && symbols[1] == (7, '.') && symbols[2] == (11, '-')
}

/// Aplica máscara de CPF no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário,
/// deve lançar erro.
///
/// ## Exemplos
///
/// Documento de 11 dígitos sem máscara:
/// ```
/// use brado::cpf;
///
/// let result = match cpf::mask("63929247011") { // Ok("639.292.470-11")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(cpf::is_masked(&result)); // true
/// ```
///
/// Documento de 11 dígitos com máscara:
/// ```should_panic
/// use brado::cpf;
///
/// let result = match cpf::mask("639.292.470-11") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    if !is_bare(doc) {
        return Err("The given string cannot be masked as CPF!");
    }

    let masked_doc = format!(
        "{}.{}.{}-{}",
        &doc[0..3],
        &doc[3..6],
        &doc[6..9],
        &doc[9..11],
    );

    Ok(masked_doc)
}

/// Gera e retorna um CPF aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// use brado::cpf;
///
/// let result = cpf::generate(); // "63929247011"
/// assert!(cpf::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    let mut cpf: Vec<u16> = random_digit_vector(9);
    cpf.push(generate_digit(&cpf));
    cpf.push(generate_digit(&cpf));

    cpf.iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// Gera e retorna um CPF aleatório com máscara.
///
/// ## Exemplo
/// ```
/// use brado::cpf;
///
/// let result = cpf::generate_masked(); // "639.292.470-11"
/// assert!(cpf::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    mask(&generate()).expect("Valid CPF!")
}
