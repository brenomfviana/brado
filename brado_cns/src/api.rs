//! Funções de API do módulo CNS.
//!
//! Cada função é um atalho para o método correspondente do trait
//! [`Document`](brado_core::Document) implementado por [`Cns`](crate::Cns).

use brado_core::Document;

use crate::Cns;

/// Realiza validação de CNS, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um CNS válido, caso contrário,
/// retorna `false`.
///
/// ## Exemplos
///
/// CNSs válidos:
/// ```
/// let result = brado_cns::validate("144082627260004"); // true
/// assert!(result);
///
/// let result = brado_cns::validate("144 0826 2726 0004"); // true
/// assert!(result);
/// ```
///
/// CNSs inválidos:
/// ```
/// let result = brado_cns::validate("144082627260005"); // false
/// assert!(!result);
///
/// let result = brado_cns::validate("144 0826 2726 0005"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    Cns::validate(doc)
}

/// Verifica se o argumento `doc` pode ser um CNS sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNSs válidos:
/// ```
/// let result = brado_cns::is_bare("144082627260004"); // true
/// assert!(result);
///
/// let result = brado_cns::is_bare("144 0826 2726 0004"); // false
/// assert!(!result);
/// ```
///
/// CNSs inválidos:
/// ```
/// let result = brado_cns::is_bare("144082627260005"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    Cns::is_bare(doc)
}

/// Verifica se o argumento `doc` pode ser um CNS com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNSs válidos:
/// ```
/// let result = brado_cns::is_masked("144 0826 2726 0004"); // true
/// assert!(result);
///
/// let result = brado_cns::is_masked("144082627260004"); // false
/// assert!(!result);
/// ```
///
/// CNSs inválidos:
/// ```
/// let result = brado_cns::is_masked("144 0826 2726 0005"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    Cns::is_masked(doc)
}

/// Aplica máscara de CNS no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 15 dígitos sem máscara:
/// ```
/// let result = match brado_cns::mask("144082627260004") { // Ok("144 0826 2726 0004")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(brado_cns::is_masked(&result)); // true
/// ```
///
/// Documento de 15 dígitos com máscara:
/// ```should_panic
/// let result = match brado_cns::mask("144 0826 2726 0004") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    Cns::mask(doc)
}

/// Gera e retorna um CNS aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_cns::generate(); // "144082627260004"
/// assert!(brado_cns::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    Cns::generate()
}

/// Gera e retorna um CNS aleatório com máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_cns::generate_masked(); // "144 0826 2726 0004"
/// assert!(brado_cns::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    Cns::generate_masked()
}
