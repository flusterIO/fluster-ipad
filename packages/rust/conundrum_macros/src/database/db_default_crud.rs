use crate::database::model::Model;
use quote::quote;
use syn::{Ident, Path, PathSegment, TypePath, punctuated::Punctuated};

use proc_macro2::Span;

pub fn gen_default_crud(input: &Model) -> Result<proc_macro2::TokenStream, syn::Error> {
    let crate_id = input.conundrum_crate_import();
    let id_type = input.primary_field()
                       .map(|x| {
                           let ty = x.ty.clone();
                           quote! {
                             #ty
                           }
                       })
                       .unwrap_or_else(|_| {
                           let nested_type =
                               input.unit
                                    .as_ref()
                                    .cloned()
                                    .expect("You must provide a 'unit' value if the struct does not have an id field.")
                                    .ty;
                           quote! {
                               <#nested_type as #crate_id::ecosystem::db::db_traits::db_entity::DBEntity>::IDType
                           }
                       });
    let partial_type = input.partial_type_or_self()?;
    let self_ident = input.ident.clone();
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
