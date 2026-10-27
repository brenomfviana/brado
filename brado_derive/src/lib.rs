// Copyright 2024 Developers of the Brado project.
//
// Licensed under the MIT license <LICENSE or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to
// those terms.

//! Macros procedurais da biblioteca brado.
//!
//! Disponibiliza o derive `#[derive(Brado)]` para validação de structs cujos
//! campos são documentos brasileiros.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, LitStr};

#[proc_macro_derive(Brado, attributes(brado))]
pub fn derive_brado(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields_named) => &fields_named.named,
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "brado_derive apenas suporta structs com campos nomeados",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input, "brado_derive apenas suporta structs")
                .to_compile_error()
                .into();
        }
    };

    let mut validations = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();

        let mut doc_type: Option<String> = None;
        for attr in &field.attrs {
            if !attr.path().is_ident("brado") {
                continue;
            }

            let result = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("document") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    doc_type = Some(s.value());
                }
                Ok(())
            });

            if let Err(err) = result {
                return err.to_compile_error().into();
            }
        }

        if let Some(doc_type) = doc_type {
            let validate_fn = match doc_type.as_str() {
                "cpf" => quote! { brado::cpf::validate },
                "cnpj" => quote! { brado::cnpj::validate },
                "cnh" => quote! { brado::cnh::validate },
                "cns" => quote! { brado::cns::validate },
                "nis" => quote! { brado::nis::validate },
                "eleitoral" => quote! { brado::eleitoral::validate },
                "renavam" => quote! { brado::renavam::validate },
                "certidao" => quote! { brado::certidao::validate },
                other => {
                    return syn::Error::new_spanned(
                        &input,
                        format!("documento não suportado: {}", other),
                    )
                    .to_compile_error()
                    .into();
                }
            };

            validations.push(quote! {
                if !#validate_fn(&self.#field_name) {
                    return false;
                }
            });
        }
    }

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            /// Valida todos os campos marcados com `#[brado(document = "...")]`.
            pub fn validate(&self) -> bool {
                #(#validations)*
                true
            }
        }
    };

    TokenStream::from(expanded)
}
