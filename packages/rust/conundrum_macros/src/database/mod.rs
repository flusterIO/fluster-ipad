use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

// mod attributes;
// mod field;
// mod model;
// mod partial;
pub mod db_partial;
pub mod schema;

// pub fn derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     match model::parse(&input) {
//         Ok(model) => generate(model).into(),

//         Err(error) => error.to_compile_error().into(),
//     }
// }

// fn generate(model: model::Model) -> proc_macro2::TokenStream {
//     let schema = schema::generate(&model);
//     let partial = partial::generate(&model);

//     quote::quote! {
//         #schema
//         #partial
//     }
// }
