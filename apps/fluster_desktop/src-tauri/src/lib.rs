mod core;
use core::commands;
use fluster_core_utilities::{
    core_types::fluster_error::FlusterError,
    tauri::features::{
        notifications::data::toast_config::ToastConfig,
        syncing::data::sync_file_system_options::SyncFilesystemDirectoryOptions,
    },
};
use tauri_plugin_prevent_default::Flags;
use tauri_specta::{Builder, collect_commands, collect_events};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let cmds = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            // -- File System --
            commands::path_exists,
            commands::read_utf8_file,
            commands::read_file_to_bytes,
            // -- Database --
            commands::get_database_path,
        ])
        .events(collect_events![ToastConfig])
        .typ::<FlusterError>()
        .typ::<SyncFilesystemDirectoryOptions>();
    let prevent = tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::all().difference(Flags::FIND | Flags::RELOAD))
        .build();

    #[cfg(debug_assertions)] // So we don't export types on release builds.
    cmds.export(
        specta_typescript::Typescript::default()
            .bigint(specta_typescript::BigIntExportBehavior::String),
        "../../../../packages/desktop_bindings/src/index.ts",
    )
    .expect("Exports bindings to typescript.");
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent)
        .invoke_handler(cmds.invoke_handler())
        .setup(move |app| {
            cmds.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
