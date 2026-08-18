use rig::client::{AgentClientExt, ProviderClient};

use crate::{ai::models::agent::agent_description::AgentDescription, ecosystem::error_handling::ai_error::AIResult};

pub trait FromAgentDescription<AIClient>
    where AIClient: rig::client::CompletionClient + AgentClientExt {
    fn from_agent_description(client: AIClient, agent_desc: AgentDescription, task_base_temperature: f64) -> Self
        where Self: Sized;
}
