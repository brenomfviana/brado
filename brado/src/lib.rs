// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

#[cfg(feature = "derive")]
pub use brado_derive::*;

#[cfg(feature = "certidao")]
pub use brado_certidao as certidao;

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

pub use brado_core::common;

pub mod docs;

#[cfg(test)]
mod tests;
