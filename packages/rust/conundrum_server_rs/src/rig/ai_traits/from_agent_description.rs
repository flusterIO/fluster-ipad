use conundrum_db::vector::models::ai::agent::agent_description::AgentDescription;

use crate::errors::server_error::ServerResult;

use crate::server_state::ServerState;

pub trait FromAgentDescription {
    fn from_agent_description(agent_desc: AgentDescription) -> ServerResult<Self>
        where Self: Sized;
}
