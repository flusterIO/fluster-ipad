//! # Conundrum Database Macros
//!
//! **Warning:** I have no idea what I'm doing and I don't have access to server
//! scale AI... but I'm super over writing those models by hand so we're doing
//! this regardless.
use convert_case::{self, Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(ConundrumDBModel, attributes(cdrm_property))]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    #[allow(clippy::collapsible_if)]
    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let field_actions =
                fields.named.iter().map(|f| {
                                       let field_name = &f.ident.as_ref().cloned().unwrap();

                                       let is_option = match f.ty.clone() {
                                           syn::Type::Path(p) => {
                                               println!("P: {:#?}", p);
                                               p.path.is_ident("Option")
                                           },
                                           _ => false
                                       };

                                       match f.attrs.iter().find(|a| a.path().is_ident("cdrm_property")) {
                                           Some(attr) => {
                                               let func_name = attr.parse_args::<syn::Expr>().unwrap();
                                               quote! {
                                                   if let Some(res) = (#func_name)(&self.#field_name) {
                                                       properties.push(res);
                                                   }
                                               }
                                           }
                                           None => {
                                               quote! {
                                                    Arc::new(DatabaseId::field_definition(stringify!(#field_name), false));
                                               }
                                           }
                                       }
                                   });

            let expanded = quote! {
                        impl<'a> DBSchema<'a> for AutoTaggable {
                            fn arrow_fields(
                                )
                                -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<std::sync::Arc<arrow_schema::Field>>>
                            {
                                let r = vec![
            #(#field_actions)*
                                ];
                                Ok(r)
                            }
                        }
                                    };
            return expanded.into();
        }
    };
    quote!().into()
}

#[proc_macro_derive(DatabaseModel, attributes(cdrm_property))]
pub fn database_model_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    #[allow(clippy::collapsible_if)]
    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let field_actions =
                fields.named.iter().map(|f| {
                                       let field_name = &f.ident.as_ref().cloned().unwrap();

                                       let camel_case_field_name = field_name.to_string().to_case(Case::Camel);

                                       match f.attrs.iter().find(|a| a.path().is_ident("cdrm_property")) {
                                           Some(attr) => {
                                               let func_name = attr.parse_args::<syn::Expr>().unwrap();
                                               quote! {
                                                   if let Some(res) = (#func_name)(&self.#field_name) {
                                                       properties.push(res);
                                                   }
                                               }
                                           }
                                           None => {
                                               quote! {
                                                   if let Some(res) = &self.#field_name.as_ref().cloned().map(|n| n.as_cdrm_property(#camel_case_field_name)).flatten() {
                                                       properties.push(res.clone());
                                                   }
                                               }
                                           }
                                       }
                                   });

            let expanded = quote! {
                impl #name {
                    pub fn to_cdrm_property_stream(&self) -> String {
                        let mut properties: Vec<String> = Vec::new();
                        #(#field_actions)*
                        properties.join(" ")
                    }
                }
            };
            return expanded.into();
        }
    };
    quote!().into()
}
