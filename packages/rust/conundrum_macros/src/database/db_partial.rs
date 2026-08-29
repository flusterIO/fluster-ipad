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

    let fields = input.fields.clone();

    let mut partial_fields = Vec::new();
    let primary_field = input.primary_field()?;

    for field in fields {
        let field_ident = field.ident.clone();

        let options = field.options.partial.clone();

        if options.skip {
            continue;
        }

        let field_type = &field.ty;

        if options.required || field == primary_field.clone() {
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
        #[derive(Debug, Clone)]
        pub struct #partial_name {
            #(#partial_fields),*
        }
    })
}
