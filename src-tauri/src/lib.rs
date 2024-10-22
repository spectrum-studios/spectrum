#[tauri::command]
fn get_title() -> String {
    format!("Spectrum")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_title])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
