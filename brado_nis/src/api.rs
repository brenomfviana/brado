//! Funções de API do módulo NIS.
//!
//! Cada função é um atalho para o método correspondente do trait
//! [`Document`](brado_core::Document) implementado por [`Nis`](crate::Nis).

use brado_core::Document;

use crate::Nis;

/// Realiza validação de NIS/NIT/PIS/PASEP, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um NIS/NIT/PIS/PASEP válido,
/// caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Números NIS/NIT/PIS/PASEP válidos:
/// ```
/// let result = brado_nis::validate("40865658047"); // true
/// assert!(result);
///
/// let result = brado_nis::validate("408.65658.04-7"); // true
/// assert!(result);
/// ```
///
/// Números NIS/NIT/PIS/PASEP inválidos:
/// ```
/// let result = brado_nis::validate("40865658046"); // false
/// assert!(!result);
///
/// let result = brado_nis::validate("408.65658.04-6"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    Nis::validate(doc)
}

/// Verifica se o argumento `doc` pode ser um NIS/NIT/PIS/PASEP sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Números NIS/NIT/PIS/PASEP válidos:
/// ```
/// let result = brado_nis::is_bare("40865658047"); // true
/// assert!(result);
///
/// let result = brado_nis::is_bare("408.65658.04-7"); // false
/// assert!(!result);
/// ```
///
/// Números NIS/NIT/PIS/PASEP inválidos:
/// ```
/// let result = brado_nis::is_bare("40865658046"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    Nis::is_bare(doc)
}

/// Verifica se o argumento `doc` pode ser um NIS/NIT/PIS/PASEP com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Números NIS/NIT/PIS/PASEP válidos:
/// ```
/// let result = brado_nis::is_masked("408.65658.04-7"); // true
/// assert!(result);
///
/// let result = brado_nis::is_masked("40865658047"); // false
/// assert!(!result);
/// ```
///
/// Números NIS/NIT/PIS/PASEP inválidos:
/// ```
/// let result = brado_nis::is_masked("408.65658.04-6"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    Nis::is_masked(doc)
}

/// Aplica máscara de NIS/NIT/PIS/PASEP no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 11 dígitos sem máscara:
/// ```
/// let result = match brado_nis::mask("40865658047") { // Ok("408.65658.04-7")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(brado_nis::is_masked(&result)); // true
/// ```
///
/// Documento de 11 dígitos com máscara:
/// ```should_panic
/// let result = match brado_nis::mask("408.65658.04-7") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    Nis::mask(doc)
}

/// Gera e retorna um NIS/NIT/PIS/PASEP aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_nis::generate(); // "40865658047"
/// assert!(brado_nis::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    Nis::generate()
}

/// Gera e retorna um NIS/NIT/PIS/PASEP aleatório com máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_nis::generate_masked(); // "408.65658.04-7"
/// assert!(brado_nis::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    Nis::generate_masked()
}
