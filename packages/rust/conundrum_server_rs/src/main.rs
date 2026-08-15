use std::path::PathBuf;

use cdrm_server_lib::run_server::run_server;
use conundrum::ecosystem::environment_variables::cdrm_env_variable::CdrmEnvVariable;

#[tokio::main]
pub async fn main() {
    let env_level = CdrmEnvVariable::LogLevel.read().map(|x| x.to_lowercase()).unwrap_or("warn".to_string());
    let filters = format!("warn,conundrum_server_rs={},conundrum={},conundrum_db={},conundrum_fs={}",
                          &env_level, &env_level, &env_level, &env_level);
    pretty_env_logger::formatted_builder().parse_filters(filters.as_str()).init();
    let _ = run_server(Some(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../conundrum_frontend/src/core/codegen/bindings.ts"))).await;
}
