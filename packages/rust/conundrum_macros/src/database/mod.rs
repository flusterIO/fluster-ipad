use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

// mod attributes;
// mod field;
pub mod database_entity;
mod db_default_crud;
mod db_entity;
mod model;
// mod partial;
pub mod db_model;
pub mod db_partial;
pub mod schema;

// pub fn derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

// match model::parse(&input) {
//     Ok(model) => generate(model).into(),

//     Err(error) => error.to_compile_error().into(),
// }
// }

// pub fn generate(model: model::Model) -> proc_macro2::TokenStream {
//     let schema = schema::generate(&model);
//     let partial = partial::generate(&model);

//     quote::quote! {
//         #schema
//         #partial
//     }
// }
