use quote::quote;

use crate::database::model::Model;

pub fn generate_db_entity(model: &Model) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ident = &model.ident;
    let table = &model.table;

    let partial_type = &model.partial_type_or_self()?;

    let crate_id = &model.conundrum_crate_import();
    if let Some(um) = &model.unit {
        let ty = um.ty.clone();
        return Ok(quote! {
        impl<'a> #crate_id::ecosystem::db::db_traits::db_entity::DBEntity<'a, #ty::IDType> for #ident {
            type PartialUpdateType = #partial_type;

            fn table() -> #crate_id::ecosystem::db::tables::DatabaseTable {
                #ty::table()
            }

            fn merge_keys() -> &'static [&'static str] {
                #ty::merge_keys()
            }

            fn primary_key() -> &'static str {
                #ty::primary_key()
            }

            fn primary_value(&self) -> #ty::IDType {
                #ty::primary_value()
            }

            fn set_primary_value(&mut self, value: #ty::IDType) {
                self.set_primary_value(value);
            }
        }
        });
    }

    let primary_ident = &model.primary_field()?.ident;
    let primary_key = model.primary_field()?.key.as_str();

    let id_type = &model.primary_field()?.ty;

    Ok(quote! {
        impl<'a> #crate_id::ecosystem::db::db_traits::db_entity::DBEntity<'a, #id_type> for #ident {
            type PartialUpdateType = #partial_type;

            fn table() -> #crate_id::ecosystem::db::tables::DatabaseTable {
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
