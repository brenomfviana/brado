//! Utilitários para validação de documentos.

use crate::cnh;
use crate::cnpj;
use crate::cpf;

/// Verifica se um documento `doc` é um CPF, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um CPF válido,
/// caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CPFs válidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_cpf("63929247011"); // true
/// assert!(result);
///
/// let result = docs::is_cpf("639.292.470-11"); // true
/// assert!(result);
/// ```
///
/// CPFs inválidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_cpf("63929247010"); // false
/// assert!(!result);
///
/// let result = docs::is_cpf("639.292.470-10"); // false
/// assert!(!result);
/// ```
pub fn is_cpf(doc: &str) -> bool {
    cpf::validate(doc)
}

/// Verifica se um documento `doc` é um CNPJ, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um CNPJ válido,
/// caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNPJs válidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_cnpj("05200851000100"); // true
/// assert!(result);
///
/// let result = docs::is_cnpj("05.200.851/0001-00"); // true
/// assert!(result);
/// ```
///
/// CNPJs inválidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_cnpj("05200851000101"); // false
/// assert!(!result);
///
/// let result = docs::is_cnpj("05.200.851/0001-01"); // false
/// assert!(!result);
/// ```
pub fn is_cnpj(doc: &str) -> bool {
    cnpj::validate(doc)
}

/// Verifica se um documento `doc` é uma CNH, máscarado ou não.
/// Retorna `true` se o argumento `doc` for uma CNH válido,
/// caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNHs válidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_cnh("84718735264"); // true
/// assert!(result);
///
/// let result = docs::is_cnh("847 187 352 64"); // true
/// assert!(result);
/// ```
///
/// CNHs inválidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_cnh("84718735265"); // false
/// assert!(!result);
///
/// let result = docs::is_cnh("847 187 352 65"); // false
/// assert!(!result);
/// ```
pub fn is_cnh(doc: &str) -> bool {
    cnh::validate(doc)
}
