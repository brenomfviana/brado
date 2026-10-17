//! Traço que define a interface comum de todos os documentos brasileiros.

/// Interface comum a todos os documentos brasileiros.
///
/// O traço [`Document`] captura a API uniforme dos documentos: funções
/// livres de validação, checagem de máscara, mascaramento e geração. Cada
/// crate de documento expõe um marcador que o implementa.
///
/// ## Exemplos
///
/// ```
/// use brado_core::Document;
///
/// fn info<D: Document>() -> (&'static str, usize) {
///     (D::NAME, D::SIZE)
/// }
/// ```
pub trait Document {
    /// Nome do documento (ex.: `"CPF"`).
    const NAME: &'static str;

    /// Tamanho do documento sem máscara (ex.: `11` para o CPF).
    const SIZE: usize;

    /// Verifica se um documento `doc` é válido, mascarado ou não.
    /// Retorna `true` se o argumento `doc` for um documento válido, caso
    /// contrário, retorna `false`.
    fn validate(doc: &str) -> bool;

    /// Verifica se um documento `doc` não possui símbolos.
    /// Retorna `true` se o argumento `doc` não possuir símbolos, caso
    /// contrário, retorna `false`.
    fn is_bare(doc: &str) -> bool;

    /// Verifica se um documento `doc` está mascarado de acordo com o
    /// documento correspondente. Retorna `true` se o argumento `doc` estiver
    /// mascarado, caso contrário, retorna `false`.
    fn is_masked(doc: &str) -> bool;

    /// Mascara um documento `doc`, apenas se não possuir símbolos e tiver o
    /// número de caracteres do documento sem símbolos. Retorna uma string
    /// (`Result<String, &'static str>`) correspondente ao documento mascarado
    /// ou um erro.
    fn mask(doc: &str) -> Result<String, &'static str>;

    /// Gera um novo documento sem símbolos (`String`).
    fn generate() -> String;

    /// Gera um novo documento mascarado (`String`).
    fn generate_masked() -> String;
}
