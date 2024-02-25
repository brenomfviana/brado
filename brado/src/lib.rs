//! # brado documentation
//!
//! brado is a brazilian docs validator.
//!
//! ## Example
//! ```
//! use brado::cpf;
//! cpf::validate("639.292.470-11");
//! ```
pub mod cnh;
pub mod cnpj;
pub mod common;
pub mod cpf;
pub mod docs;
