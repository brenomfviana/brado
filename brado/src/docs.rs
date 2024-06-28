//! Utilitários para validação de documentos.

use crate::certidao;
use crate::cnh;
use crate::cnpj;
use crate::cns;
use crate::cpf;
use crate::eleitoral;
use crate::nis;
use crate::renavam;

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
/// Retorna `true` se o argumento `doc` for uma CNH válida,
/// caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNHs válidas:
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
/// CNHs inválidas:
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

/// Verifica se um documento `doc` é um CNS, máscarado ou não.
/// Retorna `true` se o argumento `doc` for um CNS válido,
/// caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// CNSs válidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_cns("144082627260004"); // true
/// assert!(result);
///
/// let result = docs::is_cns("144 0826 2726 0004"); // true
/// assert!(result);
/// ```
///
/// CNSs inválidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_cns("144082627260005"); // false
/// assert!(!result);
///
/// let result = docs::is_cns("144 0826 2726 0005"); // false
/// assert!(!result);
/// ```
pub fn is_cns(doc: &str) -> bool {
    cns::validate(doc)
}

/// Verifica se um documento `doc` é um Número NIS/NIT/PIS/PASEP, máscarado ou
/// não. Retorna `true` se o argumento `doc` for um Número NIS/NIT/PIS/PASEP
/// válido, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Números NIS/NIT/PIS/PASEP válidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_nis("40865658047"); // true
/// assert!(result);
///
/// let result = docs::is_nis("408.65658.04-7"); // true
/// assert!(result);
/// ```
///
/// Números NIS/NIT/PIS/PASEP inválidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_nis("40865658046"); // false
/// assert!(!result);
///
/// let result = docs::is_nis("408.65658.04-6"); // false
/// assert!(!result);
/// ```
pub fn is_nis(doc: &str) -> bool {
    nis::validate(doc)
}

/// Verifica se um documento `doc` é um Título Eleitoral, máscarado ou
/// não. Retorna `true` se o argumento `doc` for um Título Eleitoral
/// válido, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Títulos Eleitorais válidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_eleitoral("773537801651"); // true
/// assert!(result);
///
/// let result = docs::is_eleitoral("7735 3780 1651"); // true
/// assert!(result);
/// ```
///
/// Títulos Eleitorais inválidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_eleitoral("773537801650"); // false
/// assert!(!result);
///
/// let result = docs::is_eleitoral("7735 3780 1650"); // false
/// assert!(!result);
/// ```
pub fn is_eleitoral(doc: &str) -> bool {
    eleitoral::validate(doc)
}

/// Verifica se um documento `doc` é um RENAVAM, máscarado ou
/// não. Retorna `true` se o argumento `doc` for um RENAVAM
/// válido, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// RENAVAMs válidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_renavam("79072338363"); // true
/// assert!(result);
///
/// let result = docs::is_renavam("7907233836-3"); // true
/// assert!(result);
/// ```
///
/// RENAVAMs inválidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_renavam("79072338362"); // false
/// assert!(!result);
///
/// let result = docs::is_renavam("7907233836-2"); // false
/// assert!(!result);
/// ```
pub fn is_renavam(doc: &str) -> bool {
    renavam::validate(doc)
}

/// Verifica se um documento `doc` é uma Certidão, máscarado ou
/// não. Retorna `true` se o argumento `doc` for uma Certidão
/// válido, caso contrário, retorna `false`.
///
/// ## Exemplos
///
/// Certidões válidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_certidao("21924201552023106304243115818536"); // true
/// assert!(result);
///
/// let result = docs::is_certidao("219242 01 55 2023 1 06304 243 1158185-36"); // true
/// assert!(result);
/// ```
///
/// Certidões inválidos:
/// ```
/// use brado::docs;
///
/// let result = docs::is_certidao("21924201552023106304243115818535"); // false
/// assert!(!result);
///
/// let result = docs::is_certidao("219242 01 55 2023 1 06304 243 1158185-35"); // false
/// assert!(!result);
/// ```
pub fn is_certidao(doc: &str) -> bool {
    certidao::validate(doc)
}
