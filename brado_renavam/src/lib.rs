// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Utilitários para validação de Registro Nacional de Veículos Automotores
//! (RENAVAM).
//!
//! `brado_renavam` é a crate do documento RENAVAM da biblioteca brado.
//! Normalmente, o usuário interage com este módulo por meio da fachada `brado`
//! (`brado::renavam`); o exemplo a seguir utiliza a crate diretamente:
//!
//! ```
//! let result = brado_renavam::validate("79072338363");
//! assert!(result);
//! ```

mod api;
mod renavam;

pub use api::{generate, generate_masked, is_bare, is_masked, mask, validate};
pub use renavam::Renavam;

#[cfg(test)]
mod tests;
