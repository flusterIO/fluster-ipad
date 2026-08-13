// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// struct ServerPort(u16);

// #[tauri::command]
// fn get_server_port(state: tauri::State<ServerPort>) -> u16 {
//     state.0
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().setup(|app| {
        tokio::async_runtime::spawn(async move {
            conundrum_server_rs::run_server::run_server().await;
        })
    })
                             .plugin(tauri_plugin_opener::init())
                             .invoke_handler(tauri::generate_handler![greet])
                             .run(tauri::generate_context!())
                             .expect("error while running tauri application");
}
