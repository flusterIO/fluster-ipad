use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericArgument, PathArguments, Type, parse_macro_input};

use crate::database::model::Model;

pub fn gen_db_schema(input: &Model) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let crate_id = &input.conundrum_crate_import();

    let mut generated_fields = Vec::new();

    for field in input.fields.clone() {
        let field_name = field.ident.clone();

        if field.is_skipped() {
            continue;
        }

        let analyzed = analyze_type(&field.ty);

        let options = field.options;

        let nullable = options.nullable.unwrap_or(analyzed.nullable);

        let name = options.rename.unwrap_or_else(|| field_name.to_string());

        let field_type = analyzed.ty;

        generated_fields.push(quote! {
                  std::sync::Arc::new(
                      <#field_type as #crate_id::ecosystem::db::db_traits::db_field::DatabaseField>::field_definition(
                          #name,
                          #nullable,
                      )
                  )
              });
    }

    Ok(quote! {
        impl<'a> #crate_id::ecosystem::db::db_traits::db_entity::DBSchema<'a> for #name {
            fn arrow_fields()
                -> #crate_id::ecosystem::error_handling::db_error::DatabaseResult<
                    Vec<std::sync::Arc<arrow_schema::Field>>
                >
            {
                Ok(vec![
                    #(#generated_fields),*
                ])
            }
        }
    })
}

pub fn derive_db_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let model = Model::try_from(input).expect("Failed to constuct Model struct from macro input.");

    match gen_db_schema(&model) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}


struct FieldOptions {
    rename: Option<String>,
    nullable: Option<bool>,
    skip: bool,
}

fn parse_field_options(field: &syn::Field) -> syn::Result<FieldOptions> {
    let mut options = FieldOptions { rename: None,
                                     nullable: None,
                                     skip: false };

    for attr in &field.attrs {
        if !attr.path().is_ident("db") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    options.skip = true;
                    return Ok(());
                }

                if meta.path.is_ident("nullable") {
                    options.nullable = Some(true);
                    return Ok(());
                }

                if meta.path.is_ident("rename") {
                    let value: syn::LitStr = meta.value()?.parse()?;
                    options.rename = Some(value.value());
                    return Ok(());
                }

                if meta.path.is_ident("nullable") {
                    let value: syn::LitBool = meta.value()?.parse()?;
                    options.nullable = Some(value.value);
                    return Ok(());
                }

                if meta.path.is_ident("partial") {
                    return Ok(());
                }

                Err(meta.error("unknown #[db(...)] option"))
            })?;
    }

    Ok(options)
}

struct AnalyzedType {
    ty: Type,
    nullable: bool,
}

fn analyze_type(ty: &Type) -> AnalyzedType {
    if let Some(inner) = unwrap_option(ty) {
        AnalyzedType { ty: inner.clone(),
                       nullable: true }
    } else {
        AnalyzedType { ty: ty.clone(),
                       nullable: false }
    }
}

fn unwrap_option(ty: &Type) -> Option<&Type> {
    let Type::Path(type_path) = ty else {
        return None;
    };

    let segment = type_path.path.segments.last()?;

    if segment.ident != "Option" {
        return None;
    }

    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };

    let GenericArgument::Type(inner) = arguments.args.first()? else {
        return None;
    };

    Some(inner)
}
