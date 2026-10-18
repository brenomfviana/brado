// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Utilitários para validação de Carteira Nacional de Habilitação (CNH).
//!
//! `brado_cnh` é a crate do documento CNH da biblioteca brado. Normalmente,
//! o usuário interage com este módulo por meio da fachada `brado`
//! (`brado::cnh`); o exemplo a seguir utiliza a crate diretamente:
//!
//! ```
//! let result = brado_cnh::validate("84718735264");
//! assert!(result);
//! ```

mod api;
mod cnh;

pub use api::{generate, generate_masked, is_bare, is_masked, mask, validate};
pub use cnh::Cnh;

#[cfg(test)]
mod tests;
