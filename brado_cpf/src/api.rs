//! Funções de API do módulo CPF.
//!
//! Cada função é um atalho para o método correspondente do trait
//! [`Document`](brado_core::Document) implementado por [`Cpf`](crate::Cpf).

use brado_core::Document;

use crate::Cpf;

/// Realiza validação de CPF, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um CPF válido, caso contrário,
/// retorna `false`.
///
/// ## Exemplos
///
/// CPFs válidos:
/// ```
/// let result = brado_cpf::validate("63929247011"); // true
/// assert!(result);
///
/// let result = brado_cpf::validate("639.292.470-11"); // true
/// assert!(result);
/// ```
///
/// CPFs inválidos:
/// ```
/// let result = brado_cpf::validate("63929247010"); // false
/// assert!(!result);
///
/// let result = brado_cpf::validate("639.292.470-10"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    Cpf::validate(doc)
}

/// Verifica se o argumento `doc` pode ser um CPF sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CPFs válidos:
/// ```
/// let result = brado_cpf::is_bare("63929247011"); // true
/// assert!(result);
///
/// let result = brado_cpf::is_bare("639.292.470-11"); // false
/// assert!(!result);
/// ```
///
/// CPFs inválidos:
/// ```
/// let result = brado_cpf::is_bare("63929247010"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    Cpf::is_bare(doc)
}

/// Verifica se o argumento `doc` pode ser um CPF com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CPFs válidos:
/// ```
/// let result = brado_cpf::is_masked("639.292.470-11"); // true
/// assert!(result);
///
/// let result = brado_cpf::is_masked("63929247011"); // false
/// assert!(!result);
/// ```
///
/// CPFs inválidos:
/// ```
/// let result = brado_cpf::is_masked("639.292.470-10"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    Cpf::is_masked(doc)
}

/// Aplica máscara de CPF no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 11 dígitos sem máscara:
/// ```
/// let result = match brado_cpf::mask("63929247011") { // Ok("639.292.470-11")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(brado_cpf::is_masked(&result)); // true
/// ```
///
/// Documento de 11 dígitos com máscara:
/// ```should_panic
/// let result = match brado_cpf::mask("639.292.470-11") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    Cpf::mask(doc)
}

/// Gera e retorna um CPF aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_cpf::generate(); // "63929247011"
/// assert!(brado_cpf::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    Cpf::generate()
}

/// Gera e retorna um CPF aleatório com máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_cpf::generate_masked(); // "639.292.470-11"
/// assert!(brado_cpf::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    Cpf::generate_masked()
}
