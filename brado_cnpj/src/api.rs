//! Funções de API do módulo CNPJ.
//!
//! Cada função é um atalho para o método correspondente do trait
//! [`Document`](brado_core::Document) implementado por [`Cnpj`](crate::Cnpj).

use brado_core::Document;

use crate::Cnpj;

/// Realiza validação de CNPJ, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um CNPJ válido, caso contrário,
/// retorna `false`.
///
/// ## Exemplos
///
/// CNPJs válidos:
/// ```
/// let result = brado_cnpj::validate("05200851000100"); // true
/// assert!(result);
///
/// let result = brado_cnpj::validate("05.200.851/0001-00"); // true
/// assert!(result);
/// ```
///
/// CNPJs inválidos:
/// ```
/// let result = brado_cnpj::validate("05200851000101"); // false
/// assert!(!result);
///
/// let result = brado_cnpj::validate("05.200.851/0001-01"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    Cnpj::validate(doc)
}

/// Verifica se o argumento `doc` pode ser um CNPJ sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNPJs válidos:
/// ```
/// let result = brado_cnpj::is_bare("05200851000100"); // true
/// assert!(result);
///
/// let result = brado_cnpj::is_bare("05.200.851/0001-00"); // false
/// assert!(!result);
/// ```
///
/// CNPJs inválidos:
/// ```
/// let result = brado_cnpj::is_bare("05200851000101"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    Cnpj::is_bare(doc)
}

/// Verifica se o argumento `doc` pode ser um CNPJ com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNPJs válidos:
/// ```
/// let result = brado_cnpj::is_masked("05.200.851/0001-00"); // true
/// assert!(result);
///
/// let result = brado_cnpj::is_masked("05200851000100"); // false
/// assert!(!result);
/// ```
///
/// CNPJs inválidos:
/// ```
/// let result = brado_cnpj::is_masked("05.200.851/0001-01"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    Cnpj::is_masked(doc)
}

/// Aplica máscara de CNPJ no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 14 dígitos sem máscara:
/// ```
/// let result = match brado_cnpj::mask("05200851000100") { // Ok("05.200.851/0001-00")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(brado_cnpj::is_masked(&result)); // true
/// ```
///
/// Documento de 14 dígitos com máscara:
/// ```should_panic
/// let result = match brado_cnpj::mask("05.200.851/0001-00") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    Cnpj::mask(doc)
}

/// Gera e retorna um CNPJ aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_cnpj::generate(); // "05200851000100"
/// assert!(brado_cnpj::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    Cnpj::generate()
}

/// Gera e retorna um CNPJ aleatório com máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_cnpj::generate_masked(); // "05.200.851/0001-00"
/// assert!(brado_cnpj::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    Cnpj::generate_masked()
}
