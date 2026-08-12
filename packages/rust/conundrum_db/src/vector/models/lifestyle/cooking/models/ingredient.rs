use crate::vector::models::{lifestyle::cooking::models::cooking_method::CookingMethod, primitives::db_id::DatabaseId};

pub struct Ingredient {
    pub id: DatabaseId,
    pub name: String,
    pub cooked: Option<CookingMethod>,
}
