pub trait AIParamProvider<DatabaseIdType, ServerState> {
    fn get_agent_description(agent_id: DatabaseIdType, rout_context: ServerState);
}
