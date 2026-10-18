//! Funções de API do módulo Certidão.
//!
//! Cada função é um atalho para o método correspondente do trait
//! [`Document`](brado_core::Document) implementado por
//! [`Certidao`](crate::Certidao).

use brado_core::Document;

use crate::Certidao;

/// Realiza validação de Certidão, máscarada ou não.
/// Retorna `true` se o argumento `doc` for uma Certidão válido, caso contrário,
/// retorna `false`.
///
/// ## Exemplos
///
/// Certidões válidos:
/// ```
/// let result = brado_certidao::validate("21924201552023106304243115818536"); // true
/// assert!(result);
///
/// let result = brado_certidao::validate("219242 01 55 2023 1 06304 243 1158185-36"); // true
/// assert!(result);
/// ```
///
/// Certidões inválidos:
/// ```
/// let result = brado_certidao::validate("21924201552023106304243115818535"); // false
/// assert!(!result);
///
/// let result = brado_certidao::validate("219242 01 55 2023 1 06304 243 1158185-35"); // false
/// assert!(!result);
/// ```
pub fn validate(doc: &str) -> bool {
    Certidao::validate(doc)
}

/// Verifica se o argumento `doc` pode ser uma Certidão sem símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Certidões válidos:
/// ```
/// let result = brado_certidao::is_bare("21924201552023106304243115818536"); // true
/// assert!(result);
///
/// let result = brado_certidao::is_bare("219242 01 55 2023 1 06304 243 1158185-36"); // false
/// assert!(!result);
/// ```
///
/// Certidões inválidos:
/// ```
/// let result = brado_certidao::is_bare("21924201552023106304243115818535"); // true
/// assert!(result);
/// ```
pub fn is_bare(doc: &str) -> bool {
    Certidao::is_bare(doc)
}

/// Verifica se o argumento `doc` pode ser uma Certidão com símbolos.
/// Se for, retorna `true`, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Certidões válidos:
/// ```
/// let result = brado_certidao::is_masked("219242 01 55 2023 1 06304 243 1158185-36"); // true
/// assert!(result);
///
/// let result = brado_certidao::is_masked("21924201552023106304243115818536"); // false
/// assert!(!result);
/// ```
///
/// Certidões inválidos:
/// ```
/// let result = brado_certidao::is_masked("219242 01 55 2023 1 06304 243 1158185-35"); // true
/// assert!(result);
/// ```
pub fn is_masked(doc: &str) -> bool {
    Certidao::is_masked(doc)
}

/// Aplica máscara de Certidão no argumento `doc` e retorna resultado.
/// O argumento deve ser uma string sem símbolos, caso contrário, deve lançar
/// erro.
///
/// ## Exemplos
///
/// Documento de 32 dígitos sem máscara:
/// ```
/// let result = match brado_certidao::mask("21924201552023106304243115818536") { // Ok("219242 01 55 2023 1 06304 243 1158185-36")
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// assert!(brado_certidao::is_masked(&result)); // true
/// ```
///
/// Documento de 32 dígitos com máscara:
/// ```should_panic
/// let result = match brado_certidao::mask("219242 01 55 2023 1 06304 243 1158185-36") { // It panics!
///     Ok(doc) => doc,
///     Err(e) => panic!("{}", e),
/// };
/// ```
pub fn mask(doc: &str) -> Result<String, &'static str> {
    Certidao::mask(doc)
}

/// Gera e retorna uma Certidão aleatório sem máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_certidao::generate(); // "21924201552023106304243115818536"
/// assert!(brado_certidao::is_bare(&result)); // true
/// ```
pub fn generate() -> String {
    Certidao::generate()
}

/// Gera e retorna uma Certidão aleatório com máscara.
///
/// ## Exemplo
/// ```
/// let result = brado_certidao::generate_masked(); // "219242 01 55 2023 1 06304 243 1158185-36"
/// assert!(brado_certidao::is_masked(&result)); // true
/// ```
pub fn generate_masked() -> String {
    Certidao::generate_masked()
}
