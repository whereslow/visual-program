#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! drag successful", name)
}