use serde::{Deserialize, Serialize};

use crate::vector::models::auth::user::user_ai_meta_data::UserAIMetaData;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub meta: UserAIMetaData,
    pub password: String,
}
