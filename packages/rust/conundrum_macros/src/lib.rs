// mod how_the_fuck_do_we_get_exported;
mod database;
use convert_case::{self, Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

// use crate::how_the_fuck_do_we_get_exported::db::database_entity::database_entity_macro_logic;

// #[proc_macro_derive(DatabaseEntity, attributes(join_tables))]
// pub fn database_entity_macro(input: TokenStream) -> TokenStream {
//     database_entity_macro_logic(input)
// }
//
//
#[proc_macro_derive(DatabaseEntity, attributes(db))]
pub fn derive_complete_db_entity(input: TokenStream) -> TokenStream {
    database::database_entity::derive_db_partial(input)
}

#[proc_macro_derive(DBSchema, attributes(db))]
pub fn derive_database_schema(input: TokenStream) -> TokenStream {
    database::schema::derive_db_schema(input)
}

#[proc_macro_derive(DBEntity, attributes(db))]
pub fn derive_database_entity(input: TokenStream) -> TokenStream {
    database::db_entity::derive_db_entity(input)
}

#[proc_macro_derive(DBPartial, attributes(db))]
pub fn derive_database_partial(input: TokenStream) -> TokenStream {
    database::db_partial::derive_db_partial(input)
}

#[proc_macro_derive(ConundrumPropertyMap, attributes(cdrm_property))]
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
