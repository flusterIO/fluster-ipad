use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

pub mod database_entity;
pub mod db_default_crud;
pub mod db_entity;
pub mod db_partial;
pub mod model;
pub mod schema;
