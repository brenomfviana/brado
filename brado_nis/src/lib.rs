// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Utilitários para validação de NIS/NIT/PIS/PASEP.
//!
//! NIS: Número de Identificação Social;
//! NIT: Número de Inscrição do Trabalhador;
//! PIS: Programa de Integração Social;
//! PASEP: Programa de Formação do Patrimônio do Servidor Público.
//!
//! `brado_nis` é a crate do documento NIS/NIT/PIS/PASEP da biblioteca brado.
//! Normalmente, o usuário interage com este módulo por meio da fachada `brado`
//! (`brado::nis`); o exemplo a seguir utiliza a crate diretamente:
//!
//! ```
//! let result = brado_nis::validate("408.65658.04-7");
//! assert!(result);
//! ```

mod api;
mod nis;

pub use api::{generate, generate_masked, is_bare, is_masked, mask, validate};
pub use nis::Nis;

#[cfg(test)]
mod tests;
