use serde::Serialize;

pub struct EmbeddingParams<T>
    where T: Serialize + Clone {
    pub ndims: Option<usize>,
    /// If true, will use ollama for gerating embeddings.
    pub use_local: Option<bool>,
    pub items: Vec<T>,
}
