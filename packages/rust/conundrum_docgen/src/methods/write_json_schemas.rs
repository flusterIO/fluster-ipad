use conundrum_cli::models::config::CliConfig;
use schemars::schema_for;

use crate::workspace_utils::get_workspace_root_duplicate::get_workspace_root;

pub fn write_json_schemas() {
    let schema = schema_for!(CliConfig);
    let root = get_workspace_root();
    let schemas_output_dir = std::path::Path::new(&root).join("packages")
                                                        .join("rust")
                                                        .join("conundrum_ts")
                                                        .join("src")
                                                        .join("code_gen")
                                                        .join("schemas");
    println!("outputdir : {:#?}", schemas_output_dir);
    let s = schema.as_value().to_string();
    if let Err(err) = std::fs::write(schemas_output_dir.join("cli_config.json"), s) {
        eprintln!("Error: {:#?}", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_json_schemas() {
        write_json_schemas();
    }
}
