// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Utilitários para validação de documentos brasileiros
//!
//! Brado fornece funções para identificação, validação e geração de documentos
//! brasileiros. O nome desta biblioteca (Brado) é um acronimo de BRAzilian
//! DOcs validator (validador de DOcumentos BRAsileiros).
//!
//! # Guia Rápido
//!
//! Para utilizar a biblioteca, basta importá-la no arquivo (`use brado`),
//! escolher o módulo correspondente ao documento e chamar a função desejada.
//! Por exemplo, para validar um CPF, basta utilizar o código a seguir:
//!
//! ```
//! # #[cfg(feature = "cpf")]
//! # {
//! use brado;
//! let result = brado::cpf::validate("639.292.470-11");
//! assert!(result);
//! # }
//! ```

pub mod docs;

#[cfg(feature = "derive")]
pub use brado_derive::*;

#[cfg(feature = "cnh")]
pub use brado_cnh as cnh;

#[cfg(feature = "cnpj")]
pub use brado_cnpj as cnpj;

#[cfg(feature = "cns")]
pub use brado_cns as cns;

#[cfg(feature = "cpf")]
pub use brado_cpf as cpf;

#[cfg(feature = "eleitoral")]
pub use brado_eleitoral as eleitoral;

#[cfg(feature = "nis")]
pub use brado_nis as nis;

#[cfg(feature = "renavam")]
pub use brado_renavam as renavam;

#[cfg(feature = "certidao")]
pub use brado_certidao as certidao;

pub use brado_core::common;

#[cfg(test)]
mod tests;
