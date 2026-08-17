use conundrum::lifted_models::primitives::db_id::DatabaseId;

use crate::vector::models::lifestyle::cooking::models::cooking_method::CookingMethod;

pub struct Ingredient {
    pub id: DatabaseId,
    pub name: String,
    pub cooked: Option<CookingMethod>,
}
