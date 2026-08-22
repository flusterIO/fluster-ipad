use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Type, parse_macro_input};

pub fn derive_db_partial(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match generate_db_partial(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

struct PartialOptions {
    include: bool,
    required: bool,
}

fn parse_partial_options(field: &syn::Field) -> syn::Result<PartialOptions> {
    let mut options = PartialOptions { include: true,
                                       required: false };

    for attr in &field.attrs {
        if !attr.path().is_ident("db") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("partial") {
                    // #[db(partial(required))]
                    if meta.input.peek(syn::token::Paren) {
                        meta.parse_nested_meta(|nested| {
                                if nested.path.is_ident("required") {
                                    options.required = true;
                                    return Ok(());
                                }
                                if nested.path.is_ident("skip") {
                                    options.required = false;
                                    return Ok(());
                                }

                                Err(nested.error("unknown partial option"))
                            })?;
                    }

                    return Ok(());
                }

                Ok(())
            })?;
    }

    Ok(options)
}

pub fn generate_db_partial(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;

    let partial_name = syn::Ident::new(&format!("{struct_name}Partial"), struct_name.span());

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => &fields.named,

            _ => {
                return Err(syn::Error::new_spanned(struct_name, "DBPartial requires named fields"));
            }
        },

        _ => {
            return Err(syn::Error::new_spanned(struct_name, "DBPartial can only be derived for structs"));
        }
    };

    let mut partial_fields = Vec::new();

    for field in fields {
        let field_ident = field.ident.as_ref().unwrap();

        let options = parse_partial_options(field)?;

        if !options.include {
            continue;
        }

        let field_type = &field.ty;

        if options.required {
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
