use crate::database::model::Model;
use quote::quote;
use syn::{DeriveInput, Ident, Path, PathSegment, TypePath, parse_macro_input, punctuated::Punctuated};

use proc_macro::{Span, TokenStream};

pub fn derive_db_default_crud(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let model = Model::try_from(input).expect("Failed to constuct Model struct from macro input.");
    match gen_default_crud(&model) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

pub fn gen_default_crud(input: &Model) -> Result<proc_macro2::TokenStream, syn::Error> {
    let crate_id = input.conundrum_crate_import();
    let partial_type = input.partial_type_or_partial_name()?;
    let self_ident = input.ident.clone();
    let id_type = input.primary_field_type_with_nested_type_fallback();
    Ok(quote! {
        impl<'a> #crate_id::ecosystem::db::db_traits::entity_crud::EntityCRUD<'a, #partial_type> for #self_ident {
            type IDType = #id_type;
            async fn get_by_predicate(predicate: Option<String>,
                                      pagination: Option<#crate_id::ecosystem::db::parameters::general::pagination::PaginationParams>,
                                      sort: Option<Vec<#crate_id::ecosystem::db::parameters::general::sort_query::SortQuery>>,
                                      db: #crate_id::ecosystem::db::db::ArcMutexDB)
                                      -> #crate_id::ecosystem::error_handling::db_error::DatabaseResult<Vec<Self>>
                where Self: Sized {
                #crate_id::get_by_predicate!(#self_ident, db, predicate, pagination, sort)
            }
        }
    })
}
