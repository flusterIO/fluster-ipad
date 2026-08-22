use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericArgument, PathArguments, Type, parse_macro_input};

pub fn derive_db_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match generate_db_schema(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn generate_db_schema(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => &fields.named,

            syn::Fields::Unnamed(_) => {
                return Err(syn::Error::new_spanned(name, "DBSchema does not support tuple structs"));
            }

            syn::Fields::Unit => {
                return Err(syn::Error::new_spanned(name, "DBSchema does not support unit structs"));
            }
        },

        _ => {
            return Err(syn::Error::new_spanned(name, "DBSchema can only be derived for structs"));
        }
    };

    let mut generated_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();

        let options = parse_field_options(field)?;

        if options.skip {
            continue;
        }

        let analyzed = analyze_type(&field.ty);

        let nullable = options.nullable.unwrap_or(analyzed.nullable);

        let name = options.rename.unwrap_or_else(|| field_name.to_string());

        let field_type = analyzed.ty;

        generated_fields.push(quote! {
                                  std::sync::Arc::new(
                                      <#field_type as DBSchemaField>::field_definition(
                                          #name,
                                          #nullable,
                                      )
                                  )
                              });
    }

    Ok(quote! {
        impl<'a> DBSchema<'a> for #name {
            fn arrow_fields()
                -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<
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
