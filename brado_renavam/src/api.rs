//! Funções de API do módulo RENAVAM.
//!
//! Cada função é um atalho para o método correspondente do trait
//! [`Document`](brado_core::Document) implementado por
//! [`Renavam`](crate::Renavam).

use brado_core::Document;

use crate::Renavam;

/// Realiza validação de RENAVAM, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um RENAVAM válido, caso contrário,
/// retorna `false`.
///
/// ## Exemplos
///
/// RENAVAMs válidos:
/// ```
/// let result = brado_renavam::validate("79072338363"); // true
/// assert!(result);
///
/// let result = brado_renavam::validate("7907233836-3"); // true
/// assert!(result);
/// ```
///
/// RENAVAMs inválidos:
/// ```
/// let result = brado_renavam::validate("79072338362"); // false
/// assert!(!result);
///
/// let result = brado_renavam::validate("7907233836-2"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    Renavam::validate(doc)
}

/// Verifica se o argumento `doc` pode ser um RENAVAM sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// RENAVAMs válidos:
/// ```
/// let result = brado_renavam::is_bare("79072338363"); // true
/// assert!(result);
///
/// let result = brado_renavam::is_bare("7907233836-3"); // false
/// assert!(!result);
/// ```
///
/// RENAVAMs inválidos:
/// ```
/// let result = brado_renavam::is_bare("79072338362"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    Renavam::is_bare(doc)
}

/// Verifica se o argumento `doc` pode ser um RENAVAM com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// RENAVAMs válidos:
/// ```
/// let result = brado_renavam::is_masked("7907233836-3"); // true
/// assert!(result);
///
/// let result = brado_renavam::is_masked("79072338363"); // false
/// assert!(!result);
/// ```
///
/// RENAVAMs inválidos:
/// ```
/// let result = brado_renavam::is_masked("7907233836-2"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    Renavam::is_masked(doc)
}

/// Aplica máscara de RENAVAM no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 11 dígitos sem máscara:
/// ```
/// let result = match brado_renavam::mask("79072338363") { // Ok("7907233836-3")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(brado_renavam::is_masked(&result)); // true
/// ```
///
/// Documento de 11 dígitos com máscara:
/// ```should_panic
/// let result = match brado_renavam::mask("7907233836-3") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    Renavam::mask(doc)
}

/// Gera e retorna um RENAVAM aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_renavam::generate(); // "79072338363"
/// assert!(brado_renavam::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    Renavam::generate()
}

/// Gera e retorna um RENAVAM aleatório com máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_renavam::generate_masked(); // "7907233836-3"
/// assert!(brado_renavam::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    Renavam::generate_masked()
}
