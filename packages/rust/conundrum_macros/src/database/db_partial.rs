use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type, parse_macro_input};

use crate::database::model::Model;

pub fn derive_db_partial(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let model = Model::try_from(input).expect("Failed to constuct Model struct from macro input.");

    match gen_db_partial(&model) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

pub fn gen_db_partial(input: &Model) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;

    let partial_name = syn::Ident::new(&format!("{struct_name}Partial"), struct_name.span());
    let table_name = input.table.clone();
    let db_attr = match input.in_source_crate {
        true => {
            if let Some(um) = input.unit.clone() {
                let unit_type = um.ty.clone();
                quote! {
                    #[db(table = #table_name, source_crate = true, unit = #unit_type)]
                }
            } else {
                quote! {
                    #[db(table = #table_name, source_crate = true)]
                }
            }
        }
        false => {
            if let Some(um) = input.unit.clone() {
                let unit_type = um.ty.clone();
                quote! {
                    #[db(table = #table_name, unit = #unit_type)]
                }
            } else {
                quote! {
                    #[db(table = #table_name)]
                }
            }
        }
    };
    if let Some(um) = &input.unit {
        let partial_nested_type = um.ty.clone();
        return Ok(quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, fake::Dummy, conundrum_macros::DBSchema)]
        #db_attr
        pub struct #partial_name(#partial_nested_type);
        });
    }

    let fields = input.fields.clone();

    let mut partial_fields = Vec::new();
    let primary_field = input.primary_field().ok();

    for field in fields {
        let field_ident = field.ident.clone();

        let options = field.options.partial.clone();

        if options.skip {
            continue;
        }

        let field_type = &field.ty;

        if options.required || primary_field.is_some_and(|x| *x == field) {
            partial_fields.push(quote! {
                                    pub #field_ident: #field_type
                                });
        } else {
            partial_fields.push(quote! {
                                    pub #field_ident: Option<#field_type>
                                });
        }
    }

    Ok(quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type, fake::Dummy, conundrum_macros::DBSchema)]
        #db_attr
        pub struct #partial_name {
            #(#partial_fields),*
        }
    })
}
