//! Utilitários para validação de Cadastro Nacional de Pessoa Jurídica (CNPJ).
use crate::common::{
    get_digits, get_symbols, random_string_from_alphabet, to_decimal,
};

const CNPJ_SIZE: usize = 14;

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

    if size != CNPJ_SIZE && !is_masked(doc) {
        return false;
    }

    let digits: Vec<u16> = get_digits(doc, to_cnpj_digit);

    if digits.len() != CNPJ_SIZE {
        return false;
    }

    for i in 0..=9 {
        if digits.iter().filter(|&n| *n == i).count() == CNPJ_SIZE {
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

    match rest < 2 {
        true => 0,
        false => 11 - rest,
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
    doc.chars().count() == CNPJ_SIZE
        && get_digits(doc, to_cnpj_digit).len() == CNPJ_SIZE
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

fn alphabet() -> Vec<char> {
    vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D',
        'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
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
    let cnpj: String = random_string_from_alphabet(12, &alphabet());
    let digits: Vec<u16> = get_digits(&cnpj, to_cnpj_digit);
    let (d13, d14): (u16, u16) = generate_digits(&digits);

    [cnpj, d13.to_string(), d14.to_string()].concat()
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
    mask(&generate()).expect("Invalid CNPJ!")
}
