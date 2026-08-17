use conundrum::lifted_models::primitives::db_id::DatabaseId;

pub struct GitCommit {
    pub repo_id: DatabaseId,
    pub git_hash: String,
}
