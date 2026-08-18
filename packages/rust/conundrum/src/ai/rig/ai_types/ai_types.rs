use rig::providers::ollama;
use rig::providers::openai;

pub type LocalCompletionModel = ollama::CompletionModel;

pub type LocalStreamingResponse = ollama::StreamingCompletionResponse;
pub type LocalMultiTurnStream = rig::agent::MultiTurnStreamItem<ollama::CompletionModel>;
pub type LocalMultiTurnStreamItem =
    rig::agent::MultiTurnStreamItem<rig::providers::ollama::StreamingCompletionResponse>;

pub type RemoteCompletionModel = openai::responses_api::ResponsesCompletionModel;

pub type RemoteCompletionClient = openai::Client;
