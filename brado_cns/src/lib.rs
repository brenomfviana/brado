// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Utilitários para validação de Cartão Nacional de Saúde (CNS).
//!
//! `brado_cns` é a crate do documento CNS da biblioteca brado. Normalmente,
//! o usuário interage com este módulo por meio da fachada `brado`
//! (`brado::cns`); o exemplo a seguir utiliza a crate diretamente:
//!
//! ```
//! let result = brado_cns::validate("144082627260004");
//! assert!(result);
//! ```

mod api;
mod cns;

pub use api::{generate, generate_masked, is_bare, is_masked, mask, validate};
pub use cns::Cns;

#[cfg(test)]
mod tests;
