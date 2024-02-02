//! # brado documentation
//!
//! brado is a brazilian docs validator.
//!
//! ## Example
//! ```
//! use brado;
//! use brado::Document;
//! let cpf_doc: Document = Document::new("639.292.470-11");
//! brado::cpf::validate(&cpf_doc, true, false);
//! ```
mod docs;
pub use crate::docs::cnh;
pub use crate::docs::cnpj;
pub use crate::docs::cpf;
pub use crate::docs::doc::Document;
