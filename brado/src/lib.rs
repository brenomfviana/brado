//! # brado documentation
//!
//! brado is a brazilian docs validator.
//!
//! ## Example
//! ```
//! use brado;
//! let cpf_doc = String::from("639.292.470-11");
//! brado::cpf::validate(&cpf_doc);
//! ```
pub mod cnh;
pub mod cnpj;
pub mod common;
pub mod cpf;
