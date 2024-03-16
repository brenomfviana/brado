// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Utilitários para validação de documentos brasileiros
//!
//! Brado fornece funções para identificação, validação e geração de documentos
//! brasileiros. O nome desta biblioteca, Brado, é um acronimo de BRAzilian
//! DOcs validator (validador de DOcumentos BRAsileiros).
//!  
//! # Guia Rápido
//!
//! Para utilizar a biblioteca, basta importá-la no arquivo (`use brado`),
//! escolher o módulo correspondente ao documento e chamar a função desejada.
//! Por exemplo, para validar um CPF, basta utilizar o código a seguir:
//!
//! ```
//! use brado;
//! let result = brado::cpf::validate("639.292.470-11");
//! assert!(result);
//! ```
pub mod cnh;
pub mod cnpj;
pub mod common;
pub mod cpf;
pub mod docs;
