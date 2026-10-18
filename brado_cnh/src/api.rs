//! Funções de API do módulo CNH.
//!
//! Cada função é um atalho para o método correspondente do trait
//! [`Document`](brado_core::Document) implementado por [`Cnh`](crate::Cnh).

use brado_core::Document;

use crate::Cnh;

/// Realiza validação de CNH, máscarado ou não.
/// Retorna `true` se o argumento `doc` for uma CNH válida, caso contrário,
/// retorna `false`.
///
/// ## Exemplos
///
/// CNHs válidas:
/// ```
/// let result = brado_cnh::validate("84718735264"); // true
/// assert!(result);
///
/// let result = brado_cnh::validate("847 187 352 64"); // true
/// assert!(result);
/// ```
///
/// CNHs inválidas:
/// ```
/// let result = brado_cnh::validate("84718735265"); // false
/// assert!(!result);
///
/// let result = brado_cnh::validate("847 187 352 65"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    Cnh::validate(doc)
}

/// Verifica se o argumento `doc` pode ser uma CNH sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNHs válidas:
/// ```
/// let result = brado_cnh::is_bare("84718735264"); // true
/// assert!(result);
///
/// let result = brado_cnh::is_bare("847 187 352 64"); // false
/// assert!(!result);
/// ```
///
/// CNHs inválidas:
/// ```
/// let result = brado_cnh::is_bare("84718735265"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    Cnh::is_bare(doc)
}

/// Verifica se o argumento `doc` pode ser uma CNH com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNHs válidas:
/// ```
/// let result = brado_cnh::is_masked("847 187 352 64"); // true
/// assert!(result);
///
/// let result = brado_cnh::is_masked("84718735264"); // false
/// assert!(!result);
/// ```
///
/// CNHs inválidas:
/// ```
/// let result = brado_cnh::is_masked("847 187 352 65"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    Cnh::is_masked(doc)
}

/// Aplica máscara de CNH no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 11 dígitos sem máscara:
/// ```
/// let result = match brado_cnh::mask("84718735264") { // Ok("847 187 352 64")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(brado_cnh::is_masked(&result)); // true
/// ```
///
/// Documento de 11 dígitos com máscara:
/// ```should_panic
/// let result = match brado_cnh::mask("847 187 352 64") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    Cnh::mask(doc)
}

/// Gera e retorna uma CNH aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_cnh::generate(); // "84718735264"
/// assert!(brado_cnh::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    Cnh::generate()
}

/// Gera e retorna uma CNH aleatório com máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_cnh::generate_masked(); // "847 187 352 64"
/// assert!(brado_cnh::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    Cnh::generate_masked()
}
