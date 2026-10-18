// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Utilitários para validação de Certidões de Nascimento, Casamento e Óbito.
//!
//! `brado_certidao` é a crate do documento Certidão da biblioteca brado.
//! Normalmente, o usuário interage com este módulo por meio da fachada `brado`
//! (`brado::certidao`); o exemplo a seguir utiliza a crate diretamente:
//!
//! ```
//! let result = brado_certidao::validate("21924201552023106304243115818536");
//! assert!(result);
//! ```

mod api;
mod certidao;

pub use api::{generate, generate_masked, is_bare, is_masked, mask, validate};
pub use certidao::Certidao;

#[cfg(test)]
mod tests;
