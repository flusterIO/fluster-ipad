use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

use crate::database::{
    db_default_crud::gen_default_crud, db_entity::generate_db_entity, db_partial::gen_db_partial, model::Model,
    schema::gen_db_schema,
};

pub fn derive_db_partial(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let model = Model::try_from(input).expect("Failed to constuct Model struct from macro input.");
    match generate(&model) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

pub fn generate(model: &Model) -> Result<proc_macro2::TokenStream, syn::Error> {
    let db_entity = generate_db_entity(model)?;
    let db_schema = gen_db_schema(model)?;
    let crud = gen_default_crud(model)?;
    let partial = gen_db_partial(model)?;

    Ok(quote! {
        #db_entity
        #db_schema
        #crud
        #partial
    })
}
