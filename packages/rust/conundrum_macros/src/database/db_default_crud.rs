use crate::database::model::Model;
use quote::quote;

pub fn gen_default_crud(input: &Model) -> Result<proc_macro2::TokenStream, syn::Error> {
    let id = input.primary_field()?;
    let id_type = id.ty.clone();
    let partial_type = input.partial_type_or_self()?;
    let self_ident = input.ident.clone();
    Ok(quote! {
        impl<'a> conundrum::ecosystem::db::db_traits::entity_crud::EntityCRUD<'a, #id_type, #partial_type> for #self_ident {
            async fn get_by_predicate(predicate: Option<String>,
                                      pagination: Option<conundrum::ecosystem::db::parameters::general::pagination::PaginationParams>,
                                      sort: Option<Vec<conundrum::ecosystem::db::parameters::general::sort_query::SortQuery>>,
                                      db: &conundrum::ecosystem::db::db::ArcMutexDB)
                                      -> conundrum::ecosystem::error_handling::db_error::DatabaseResult<Vec<Self>>
                where Self: Sized {
                conundrum::get_by_predicate!(#self_ident, db, predicate, pagination, sort)
            }
        }
    })
}
