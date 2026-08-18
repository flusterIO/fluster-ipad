use crate::ecosystem::db::db_traits::{
    local_default::DefaultLocalVectorGeneration, remote_default::DefaultRemoteVectorGeneration,
};

pub struct SchemaParameters {
    pub gen_vectors_locally: bool,
}

impl DefaultLocalVectorGeneration for SchemaParameters {
    fn default_local() -> Self {
        SchemaParameters { gen_vectors_locally: true }
    }
}

impl DefaultRemoteVectorGeneration for SchemaParameters {
    fn default_remote() -> Self {
        SchemaParameters { gen_vectors_locally: false }
    }
}
