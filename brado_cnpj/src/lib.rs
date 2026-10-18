// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Utilitários para validação de Cadastro Nacional de Pessoa Jurídica (CNPJ).
//!
//! `brado_cnpj` é a crate do documento CNPJ da biblioteca brado. Normalmente,
//! o usuário interage com este módulo por meio da fachada `brado`
//! (`brado::cnpj`); o exemplo a seguir utiliza a crate diretamente:
//!
//! ```
//! let result = brado_cnpj::validate("05200851000100");
//! assert!(result);
//! ```

mod api;
mod cnpj;

pub use api::{generate, generate_masked, is_bare, is_masked, mask, validate};
pub use cnpj::Cnpj;

#[cfg(test)]
mod tests;
