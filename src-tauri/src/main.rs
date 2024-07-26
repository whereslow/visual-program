#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{generate_context, generate_handler, Builder, WindowBuilder, WindowUrl};

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![open_about_window])
        .run(generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn open_about_window(app: tauri::AppHandle, path: &str) -> Result<bool, bool> {
    let flag = WindowBuilder::new(
        &app,
        path,
        WindowUrl::App(("/".to_owned() + path).into()),
    )
    .inner_size(800., 600.)
    .position(500.0, 250.0)
    .title(path)
    .build();
    match flag {
        Ok(_) => Ok(true),
        Err(_) => Ok(false)
    }
}
