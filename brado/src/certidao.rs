//! Utilitários para validação de Certidões de Nascimento, Casamento e Óbito.

use crate::common::{get_digits, get_symbols, random_digit_vector};

/// Realiza validação de Certidão, máscarada ou não.
/// Retorna `true` se o argumento `doc` for uma Certidão válido, caso contrário,
/// retorna `false`.
///
/// ## Exemplos
///
/// Certidões válidos:
/// ```
/// use brado::certidao;
///
/// let result = certidao::validate("21924201552023106304243115818536"); // true
/// assert!(result);
///
/// let result = certidao::validate("219242 01 55 2023 1 06304 243 1158185-36"); // true
/// assert!(result);
/// ```
///
/// Certidões inválidos:
/// ```
/// use brado::certidao;
///
/// let result = certidao::validate("21924201552023106304243115818535"); // false
/// assert!(!result);
///
/// let result = certidao::validate("219242 01 55 2023 1 06304 243 1158185-35"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    let size: usize = doc.chars().count();

    if size != 32 && !is_masked(doc) {
        return false;
    }

    let digits: Vec<u16> = get_digits(doc);

    if digits.len() != 32 {
        return false;
    }

    let (d30, d31): (u16, u16) = generate_digits(&digits[..30]);

    d30 == digits[30] && d31 == digits[31]
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
            multiplier = if multiplier > 10 { 0 } else { multiplier };

            result
        })
        .sum();

    let rest: u16 = d % 11;

    if rest > 9 {
        1
    } else {
        rest
    }
}

/// Verifica se o argumento `doc` pode ser uma Certidão sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Certidões válidos:
/// ```
/// use brado::certidao;
///
/// let result = certidao::is_bare("21924201552023106304243115818536"); // true
/// assert!(result);
///
/// let result = certidao::is_bare("219242 01 55 2023 1 06304 243 1158185-36"); // false
/// assert!(!result);
/// ```
///
/// Certidões inválidos:
/// ```
/// use brado::certidao;
///
/// let result = certidao::is_bare("21924201552023106304243115818535"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    doc.chars().count() == 32 && get_digits(doc).len() == 32
}

/// Verifica se o argumento `doc` pode ser uma Certidão com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Certidões válidos:
/// ```
/// use brado::certidao;
///
/// let result = certidao::is_masked("219242 01 55 2023 1 06304 243 1158185-36"); // true
/// assert!(result);
///
/// let result = certidao::is_masked("21924201552023106304243115818536"); // false
/// assert!(!result);
/// ```
///
/// Certidões inválidos:
/// ```
/// use brado::certidao;
///
/// let result = certidao::is_masked("219242 01 55 2023 1 06304 243 1158185-35"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    let symbols: Vec<(usize, char)> = get_symbols(doc);
    let digits: Vec<u16> = get_digits(doc);

    if symbols.len() != 8 || digits.len() != 32 {
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

/// Aplica máscara de Certidão no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 11 dígitos sem máscara:
/// ```
/// use brado::certidao;
///
/// let result = match certidao::mask("21924201552023106304243115818536") { // Ok("219242 01 55 2023 1 06304 243 1158185-36")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(certidao::is_masked(&result)); // true
/// ```
///
/// Documento de 11 dígitos com máscara:
/// ```should_panic
/// use brado::certidao;
///
/// let result = match certidao::mask("219242 01 55 2023 1 06304 243 1158185-36") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    if !is_bare(doc) {
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

/// Gera e retorna uma Certidão aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// use brado::certidao;
///
/// let result = certidao::generate(); // "21924201552023106304243115818536"
/// assert!(certidao::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    let mut certidao: Vec<u16> = random_digit_vector(30);
    certidao.push(generate_digit(&certidao));
    certidao.push(generate_digit(&certidao));

    certidao
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// Gera e retorna uma Certidão aleatório com máscara.
///
/// ## Exemplo
/// ```
/// use brado::certidao;
///
/// let result = certidao::generate_masked(); // "219242 01 55 2023 1 06304 243 1158185-36"
/// assert!(certidao::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    mask(&generate()).expect("Valid Certidão!")
}
