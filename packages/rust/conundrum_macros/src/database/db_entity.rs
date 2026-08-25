use quote::quote;

use crate::database::model::Model;

pub fn generate_db_entity(model: &Model) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ident = &model.ident;
    let table = &model.table;

    let primary_ident = &model.primary_field()?.ident;
    let primary_key = model.primary_field()?.key.as_str();

    let id_type = &model.primary_field()?.ty;

    let partial_type = &model.partial_type_or_self()?;

    Ok(quote! {
        impl<'a> conundrum::ecosystem::db::db_traits::db_entity::DBEntity<'a, #id_type> for #ident {
            type PartialUpdateType = #partial_type;

            fn table() -> conundrum::ecosystem::db::tables::DatabaseTable {
                #table
            }

            fn merge_keys() -> &'static [&'static str] {
                &[#primary_key]
            }

            fn primary_key() -> &'static str {
                #primary_key
            }

            fn primary_value(&self) -> #id_type {
                self.#primary_ident.clone()
            }

            fn set_primary_value(&mut self, value: #id_type) {
                self.#primary_ident = value.clone();
            }
        }
    })
}
