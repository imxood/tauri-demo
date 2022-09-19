#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{WindowBuilder, WindowUrl};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window_builder = WindowBuilder::new(app, "main", WindowUrl::default())
                .title("Tauri API Validation")
                .min_inner_size(1024., 650.)
                .decorations(false)
                .transparent(true);

            let window = window_builder.build().unwrap();

            #[cfg(debug_assertions)]
            window.open_devtools();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
