use crate::vector::models::primitives::db_id::DatabaseId;

pub struct UserBlog {
    pub note_ids: Vec<DatabaseId>,
    pub output_path: Option<String>,
}
