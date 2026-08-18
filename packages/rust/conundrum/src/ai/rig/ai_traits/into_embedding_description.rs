use rig::Embed;

pub trait IntoEmbeddingDescription: Embed {
    fn into_embedding_description(&self) -> String;
    fn human_readable_model_name() -> &'static str;
}

impl IntoEmbeddingDescription for String {
    fn into_embedding_description(&self) -> String {
        self.clone()
    }

    fn human_readable_model_name() -> &'static str {
        "String"
    }
}
