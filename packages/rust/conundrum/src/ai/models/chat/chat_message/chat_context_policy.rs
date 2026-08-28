pub struct ChatContextPolicy {
    pub recent_messages: usize,

    pub max_memory_results: usize,

    pub max_document_results: usize,

    pub max_context_tokens: usize,
}

impl Default for ChatContextPolicy {
    fn default() -> Self {
        Self { recent_messages: 20,
               max_memory_results: 8,
               max_document_results: 6,
               max_context_tokens: 16_000 }
    }
}
