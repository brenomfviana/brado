// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Utilitários para validação de Título Eleitoral.
//!
//! `brado_eleitoral` é a crate do documento Título Eleitoral da biblioteca
//! brado. Normalmente, o usuário interage com este módulo por meio da
//! fachada `brado` (`brado::eleitoral`); o exemplo a seguir utiliza a crate
//! diretamente:
//!
//! ```
//! let result = brado_eleitoral::validate("773537801651");
//! assert!(result);
//! ```

mod api;
mod eleitoral;

pub use api::{generate, generate_masked, is_bare, is_masked, mask, validate};
pub use eleitoral::Eleitoral;

#[cfg(test)]
mod tests;
