// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Núcleo compartilhado da biblioteca brado.
//!
//! `brado_core` reúne o traço [`Document`], que define a interface comum de
//! todos os documentos brasileiros, e as funções auxiliares genéricas do
//! módulo [`common`], utilizadas na validação e geração de documentos.
//!
//! # Guia Rápido
//!
//! Normalmente, o usuário interage com este núcleo por meio da fachada
//! `brado`. A implementação do traço [`Document`] para um documento qualquer
//! segue o exemplo abaixo:
//!
//! ```
//! use brado_core::Document;
//!
//! struct Registro;
//!
//! impl Document for Registro {
//!     const NAME: &'static str = "Registro";
//!     const SIZE: usize = 3;
//!
//!     fn validate(doc: &str) -> bool {
//!         doc.chars().count() == Self::SIZE
//!     }
//!
//!     fn is_bare(doc: &str) -> bool {
//!         doc.chars().all(|c| c.is_ascii_digit())
//!     }
//!
//!     fn is_masked(doc: &str) -> bool {
//!         doc.chars().any(|c| !c.is_ascii_digit())
//!     }
//!
//!     fn mask(doc: &str) -> Result<String, &'static str> {
//!         if doc.chars().count() == Self::SIZE {
//!             Ok(doc.to_string())
//!         } else {
//!             Err("The given string cannot be masked as Registro!")
//!         }
//!     }
//!
//!     fn generate() -> String {
//!         String::from("000")
//!     }
//!
//!     fn generate_masked() -> String {
//!         Self::generate()
//!     }
//! }
//!
//! assert!(Registro::validate("000"));
//! assert_eq!(Registro::NAME, "Registro");
//! ```
pub mod common;

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

    /// Verifica se um documento `doc` é válido, máscarado ou não.
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
