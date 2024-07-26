use tauri::{Builder, generate_context, generate_handler, WindowBuilder, WindowUrl};

fn main() {
  Builder::default()
    .invoke_handler(generate_handler![open_about_window])
    .run(generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn open_about_window(app: tauri::AppHandle) {
  WindowBuilder::new(
    &app,
    "about",
    WindowUrl::App("index.html#/about".into())  // 使用相对路径
  )
  .title("About Window")
  .build()
  .unwrap();
}
