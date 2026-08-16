use conundrum_db::vector::models::primitives::db_id::DatabaseId;

use crate::server_state::ServerState;

pub trait AIParamProvider {
    fn get_agent_description(agent_id: DatabaseId, rout_context: ServerState);
}
