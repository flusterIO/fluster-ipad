use fluster_core_utilities::core_types::fluster_error::FlusterResult;

pub type DatabaseIndexSetupFunction = fn() -> FlusterResult<()>;
