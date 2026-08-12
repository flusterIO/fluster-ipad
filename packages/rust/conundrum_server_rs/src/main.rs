use std::path::PathBuf;

use cdrm_server_lib::run_server::run_server;
use conundrum::ecosystem::environment_variables::cdrm_env_variable::CdrmEnvVariable;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init_custom_env(CdrmEnvVariable::LogLevel.to_string().as_str());
    run_server(Some(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../conundrum_frontend/src/core/codegen/bindings.ts"))).await;
}
