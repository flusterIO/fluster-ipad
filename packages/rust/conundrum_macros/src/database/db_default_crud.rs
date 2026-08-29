use crate::database::model::Model;
use quote::quote;

pub fn gen_default_crud(input: &Model) -> Result<proc_macro2::TokenStream, syn::Error> {
    let id = input.primary_field()?;
    let crate_id = input.conundrum_crate_import();
    let id_type = id.ty.clone();
    let partial_type = input.partial_type_or_self()?;
    let self_ident = input.ident.clone();
    Ok(quote! {
        impl<'a> #crate_id::ecosystem::db::db_traits::entity_crud::EntityCRUD<'a, #id_type, #partial_type> for #self_ident {
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
