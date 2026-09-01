use crate::database::model::Model;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn derive_db_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let model = Model::try_from(input).expect("Failed to constuct Model struct from macro input.");
    match generate_db_entity(&model) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

pub fn generate_db_entity(model: &Model) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ident = &model.ident;
    let table = &model.table_required()?;

    let as_entity = &model.as_db_entity_path();
    let crate_id = &model.conundrum_crate_import();

    if let Some(um) = &model.unit {
        let ty = um.ty.clone();
        return Ok(quote! {
        impl #crate_id::ecosystem::db::db_traits::db_entity::DBEntity for #ident {
            fn table() -> #crate_id::ecosystem::db::tables::DatabaseTable {
                #table
            }
        }
        });
    }

    Ok(quote! {
        impl<'a> #crate_id::ecosystem::db::db_traits::db_entity::DBEntity for #ident {

            fn table() -> #crate_id::ecosystem::db::tables::DatabaseTable {
                #table
            }
        }
    })
}
