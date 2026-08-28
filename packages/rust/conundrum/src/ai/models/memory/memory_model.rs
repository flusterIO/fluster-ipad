use crate::lifted_models::primitives::date_time::DateTime;
use crate::lifted_models::primitives::db_id::DatabaseId;

pub struct Memory {
    pub id: DatabaseId,

    pub user_id: DatabaseId,

    pub convo_id: Option<DatabaseId>,

    pub kind: MemoryKind,

    pub text: String,

    /// A number betweeen 0 and 1 indicating the importane of this memory.
    pub importance: f32,

    pub created_at: DateTime,

    pub last_accessed_at: Option<DateTime>,

    pub vector: Option<Vec<f32>>,
}

pub enum MemoryKind {
    Fact,
    Preference,
    Goal,
    Project,
    Context,
    Summary,
}
