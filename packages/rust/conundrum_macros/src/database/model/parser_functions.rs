use quote::quote;
use syn::{GenericArgument, PathArguments, Type};

use crate::database::model::partial_options::PartialOptions;

pub fn is_option(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    segment.ident == "Option"
}

pub fn extract_inner_type(ty: &Type) -> Type {
    let Type::Path(type_path) = ty else {
        return ty.clone();
    };

    let Some(segment) = type_path.path.segments.last() else {
        return ty.clone();
    };

    if segment.ident != "Option" {
        return ty.clone();
    }

    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return ty.clone();
    };

    let Some(GenericArgument::Type(inner)) = arguments.args.first() else {
        return ty.clone();
    };

    inner.clone()
}

pub fn parse_partial(options: &mut PartialOptions, meta: syn::meta::ParseNestedMeta<'_>) -> Result<(), syn::Error> {
    meta.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                options.skip = true;
                return Ok(());
            }

            if meta.path.is_ident("skip_arrow") {
                options.skip_arrow = true;
                return Ok(());
            }

            if meta.path.is_ident("required") {
                options.required = true;
                return Ok(());
            }

            if meta.path.is_ident("patch") {
                options.patch = true;
                return Ok(());
            }

            Err(meta.error(format!("unknown partial option: {}",
                                   meta.path
                                       .get_ident()
                                       .map(|q| {
                                           quote! {
                                               #q
                                           }
                                       })
                                       .unwrap_or_else(|| quote! {})).as_str()))
        })
}
