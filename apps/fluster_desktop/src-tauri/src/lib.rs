mod core;
use core::commands;
use fluster_core_utilities::{
    core_types::fluster_error::FlusterError,
    tauri::features::{
        embedded_docs::data::embedded_docs_id::InternalEmbeddedDocsId,
        health::data::desktop_health_report::DesktopHealthReport,
        notifications::data::toast_config::ToastConfig, search::pagination::PaginationProps,
        syncing::data::sync_file_system_options::SyncFilesystemDirectoryOptions,
    },
};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_plugin_prevent_default::Flags;
use tauri_specta::{Builder, collect_commands, collect_events};

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct Test {
    pub val: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let cmds = Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            // -- File System --
            commands::path_exists,
            commands::read_utf8_file,
            commands::read_file_to_bytes,
            commands::fs_file_extension_glob,
            // -- Database --
            commands::get_database_path,
            commands::initialize_database,
            // -- Health --
            commands::get_desktop_health_report
        ])
        .events(collect_events![ToastConfig])
        .typ::<FlusterError>()
        .typ::<Test>()
        .typ::<DesktopHealthReport>()
        .typ::<PaginationProps>()
        .typ::<InternalEmbeddedDocsId>()
        .typ::<SyncFilesystemDirectoryOptions>();
    let prevent = tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::all().difference(Flags::FIND | Flags::RELOAD))
        .build();

    #[cfg(debug_assertions)] // So we don't export types on release builds.
    let s = cmds
        .export_str(
            specta_typescript::Typescript::default()
                .bigint(specta_typescript::BigIntExportBehavior::String),
        )
        .unwrap();
    println!("Result: {}", s);

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
