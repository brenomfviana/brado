// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Núcleo compartilhado da biblioteca brado.
//!
//! `brado_core` reúne o trait [`Document`], que define a interface comum de
//! todos os documentos brasileiros, e as funções auxiliares genéricas do
//! módulo [`common`], utilizadas na validação e geração de documentos.
//!
//! # Guia Rápido
//!
//! Normalmente, o usuário interage com este núcleo por meio da fachada
//! `brado`. A implementação do trait [`Document`] para um documento qualquer
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
pub mod document;

pub use document::Document;
