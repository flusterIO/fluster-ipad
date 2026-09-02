use convert_case::Pattern;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type, parse_macro_input};

use crate::database::model::{model::Model, utils::MacroUtils};

pub fn derive_db_partial(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let model = Model::try_from(input).expect("Failed to constuct Model struct from macro input.");

    match gen_db_partial(&model) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

pub fn gen_db_partial(input: &Model) -> syn::Result<proc_macro2::TokenStream> {
    let partial_name = input.partial_name();
    let include_generics = match input.opts.include_partial_generics {
        true => {
            quote! {
                true
            }
        }
        false => {
            quote! {
                false
            }
        }
    };
    let db_attr = match input.in_source_crate {
        true => {
            if let Some(um) = input.unit.clone() {
                let unit_type = um.ty.clone();
                quote! {
                    #[db(source_crate = true, unit = #unit_type, include_generics = #include_generics)]
                }
            } else {
                quote! {
                    #[db(source_crate = true, include_generics = #include_generics)]
                }
            }
        }
        false => {
            if let Some(um) = input.unit.clone() {
                let unit_type = um.ty.clone();
                quote! {
                    #[db(unit = #unit_type, include_generics = #include_generics)]
                }
            } else {
                quote! {
                    #[db(include_generics = #include_generics)]
                }
            }
        }
    };

    let partial_generics = match (input.opts.include_generics || input.opts.include_partial_generics) {
        true => {
            let generics = input.generics.clone();
            quote! {
               #generics
            }
        }
        false => {
            quote! {}
        }
    };
    let partial_where_clause = match (input.opts.include_generics || input.opts.include_partial_generics) {
        true => {
            if let Some(g) = input.where_clause.clone() {
                let g = input.where_clause.clone();
                quote! {
                    #g
                }
            } else {
                quote! {}
            }
        }
        false => {
            quote! {}
        }
    };

    let partial_dummy = &input.partial_dummy();
    if let Some(um) = &input.unit {
        let partial_nested_type = um.ty.clone();
        let crate_id = input.conundrum_crate_import();
        let self_def = match input.opts.partial_self {
            true => {
                quote! {
                #partial_nested_type
                }
            }
            false => {
                quote! {
                <#partial_nested_type as #crate_id::ecosystem::db::db_traits::db_entity::DBSchema>::PartialUpdateType
                }
            }
        };
        Ok(quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type #partial_dummy, conundrum_macros::DBSchema)]
        #db_attr
        pub struct #partial_name #partial_generics(#self_def);
        })
    } else {
        let fields = input.fields.clone();

        let mut partial_fields = Vec::new();
        let primary_field = input.primary_field().ok();

        for field in fields {
            let field_ident = field.ident.clone();

            let partial_options = field.options.partial.clone();

            if partial_options.skip || field.options.skip {
                continue;
            }

            let field_type = &field.ty;
            let is_primary_field = primary_field.is_some_and(|x| *x == field.clone());
            let fm = field.partial_field_macros_tokens(is_primary_field);
            let serde_macro = field.serde_macro();

            if partial_options.required || is_primary_field {
                partial_fields.push(quote! {
                                        #fm
                                        #serde_macro
                                        pub #field_ident: #field_type
                                    });
            } else {
                partial_fields.push(quote! {
                                        #fm
                                        #serde_macro
                                        pub #field_ident: Option<#field_type>
                                    });
            }
        }

        Ok(quote! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type #partial_dummy, conundrum_macros::DBSchema)]
            #db_attr
            pub struct #partial_name #partial_generics #partial_where_clause {
                #(#partial_fields),*
            }
        })
    }
}
