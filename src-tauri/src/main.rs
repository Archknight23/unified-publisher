// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tauri::command]
fn greet(name: &str) -> String{
  format!("Sup {}! You've been selected for testing. Graitude is implied.", name)
}
fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet])
      .run(tauri::generate_context!())
      .expect("Oh no, shit's broken, gurl");
  
}
