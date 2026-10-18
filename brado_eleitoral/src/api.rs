//! Funções de API do módulo Eleitoral.
//!
//! Cada função é um atalho para o método correspondente do trait
//! [`Document`](brado_core::Document) implementado por
//! [`Eleitoral`](crate::Eleitoral).

use brado_core::Document;

use crate::Eleitoral;

/// Realiza validação de Título Eleitoral, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um Título Eleitoral válido,
/// caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Títulos Eleitorais válidos:
/// ```
/// let result = brado_eleitoral::validate("773537801651"); // true
/// assert!(result);
///
/// let result = brado_eleitoral::validate("7735 3780 1651"); // true
/// assert!(result);
/// ```
///
/// Títulos Eleitorais inválidos:
/// ```
/// let result = brado_eleitoral::validate("773537801650"); // false
/// assert!(!result);
///
/// let result = brado_eleitoral::validate("7735 3780 1650"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    Eleitoral::validate(doc)
}

/// Verifica se o argumento `doc` pode ser um Título Eleitoral sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Títulos Eleitorais válidos:
/// ```
/// let result = brado_eleitoral::is_bare("773537801651"); // true
/// assert!(result);
///
/// let result = brado_eleitoral::is_bare("7735 3780 1651"); // false
/// assert!(!result);
/// ```
///
/// Títulos Eleitorais inválidos:
/// ```
/// let result = brado_eleitoral::is_bare("773537801650"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    Eleitoral::is_bare(doc)
}

/// Verifica se o argumento `doc` pode ser um Título Eleitoral com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Títulos Eleitorais válidos:
/// ```
/// let result = brado_eleitoral::is_masked("7735 3780 1651"); // true
/// assert!(result);
///
/// let result = brado_eleitoral::is_masked("773537801651"); // false
/// assert!(!result);
/// ```
///
/// Títulos Eleitorais inválidos:
/// ```
/// let result = brado_eleitoral::is_masked("7735 3780 1650"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    Eleitoral::is_masked(doc)
}

/// Aplica máscara de Título Eleitoral no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 12 dígitos sem máscara:
/// ```
/// let result = match brado_eleitoral::mask("773537801651") { // Ok("7735 3780 1651")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(brado_eleitoral::is_masked(&result)); // true
/// ```
///
/// Documento de 12 dígitos com máscara:
/// ```should_panic
/// let result = match brado_eleitoral::mask("7735 3780 1651") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    Eleitoral::mask(doc)
}

/// Gera e retorna um Título Eleitoral aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_eleitoral::generate(); // "773537801651"
/// assert!(brado_eleitoral::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    Eleitoral::generate()
}

/// Gera e retorna um Título Eleitoral aleatório com máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_eleitoral::generate_masked(); // "7735 3780 1651"
/// assert!(brado_eleitoral::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    Eleitoral::generate_masked()
}
