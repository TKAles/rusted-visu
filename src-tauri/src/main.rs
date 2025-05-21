#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, Runtime, AppHandle, Window};
use std::path::PathBuf;
use tauri_plugin_dialog::{DialogExt, FilePath};

#[tauri::command]
fn test_fn() -> String {
  "Hello from Rust".to_string()
}

#[tauri::command]
async fn get_wfmdir<R: Runtime>(app: AppHandle<R>) -> String {
    let folder_dialog = app.dialog().file();
    let dialog_result = folder_dialog.blocking_pick_folder();
    match dialog_result {
      Some(path) => path.to_string(),
      None => "".to_string()
    }
}    

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![
      get_wfmdir
    ])
    .run(tauri::generate_context!())
    .expect("Error while running tauri application!!");
}