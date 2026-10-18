// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Utilitários para validação de Cadastro de Pessoa Física (CPF).
//!
//! `brado_cpf` é a crate do documento CPF da biblioteca brado. Normalmente,
//! o usuário interage com este módulo por meio da fachada `brado`
//! (`brado::cpf`); o exemplo a seguir utiliza a crate diretamente:
//!
//! ```
//! let result = brado_cpf::validate("639.292.470-11");
//! assert!(result);
//! ```

mod api;
mod cpf;

pub use api::{generate, generate_masked, is_bare, is_masked, mask, validate};
pub use cpf::Cpf;

#[cfg(test)]
mod tests;
