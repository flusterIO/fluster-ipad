mod core;
mod features;
use features::search::methods::fs_file_extension_glob::fs_file_extension_glob;
use fluster_core_utilities::core_types::fluster_error::FlusterError;
use fluster_core_utilities::tauri::features::notifications::toast_config::ToastConfig;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_plugin_prevent_default::Flags;
use tauri_specta::{Builder, collect_commands};

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Test {
    pub val: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let cmds = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            // -- Settings --
            // delete_setting_state,
            // save_setting_state,
            // get_setting_state,
            fs_file_extension_glob
        ])
        // .events(collect_events![])
        .typ::<ToastConfig>()
        .typ::<FlusterError>();
    let prevent = tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::all().difference(Flags::FIND | Flags::RELOAD))
        .build();

    #[cfg(debug_assertions)] // So we don't export types on release builds.
    cmds.export(
        specta_typescript::Typescript::default()
            .bigint(specta_typescript::BigIntExportBehavior::String),
        "/Users/bigsexy/Desktop/swift/Fluster/packages/desktop_bindings/src/bindings.ts",
    )
    .expect("Exports bindings to typescript.");
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(prevent)
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(cmds.invoke_handler())
        .setup(move |app| {
            cmds.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
