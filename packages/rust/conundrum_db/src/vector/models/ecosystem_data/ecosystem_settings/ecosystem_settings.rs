use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::vector::models::{
    ecosystem_data::ecosystem_settings::setting_sub_types::vector_generation_methods::VectorGenerationMethods,
    primitives::db_id_single_instance::DBIDSingleInstance,
};

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct EcosystemSettings {
    pub id: DBIDSingleInstance,
    /// The number of days that logs should be preserved. Default is 7.
    pub save_log_duration: u16,
    /// A number clamped between 0 and 1 indicating the amount of preference the
    /// Conundrum ecosystem should give to local AI. A higher score will result
    /// in more tasks being relegated to Ollama, when Ollama may be capable
    /// of the job.
    ///
    /// While these tools have not yet been implemented as of writing this,
    /// these are some of the intended tools and their subjective 'local_ai'
    /// score. If this `local_ai_preference` is greater than the tool's
    /// `ai_score`, than local ai will be used for that task.
    ///
    /// ### Intended Scores
    ///
    /// - Vector Generation: 0.8
    /// - Flashcard Generation: 0.65
    /// - Note Summarization: 0.5
    /// - Welcome the user: 0.35 (be aware, local AI cannot query the vector DB
    ///   created by server scale AI)
    ///
    /// This breaks down however when using the primary chat, as agent routing
    /// does not integrate across the local/remote barrier. This will however
    /// save significant token cost for tasks like FlashCard generation where
    /// a specifically trained local model may be sufficient.
    pub local_ai_preference: f32,
    /// This struct describes a map of `VectorGenerationMethods that can reduce
    /// token consumption and/or improve vector retrieval by keeping things
    /// more up to date.
    pub vector_generation_methods: VectorGenerationMethods,
}
