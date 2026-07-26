use serde::{Deserialize, Serialize};

use crate::vector::models::auth::user::user_mood::UserMood;

#[derive(Serialize, Deserialize, Clone)]
pub struct UserAIMetaData {
    pub preferred_name: Option<String>,
    pub profession: Option<String>,
    pub mood: Option<UserMood>,
}
