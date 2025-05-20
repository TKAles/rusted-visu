// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dirs::desktop_dir;
use std::path::PathBuf;
use tauri::{generate_handler, Window};
use tauri_plugin_dialog::DialogExt;

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(generate_handler![select_wfm_folder])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
pub async fn select_wfm_folder(window: Window) ->
  Result<Option<String>, String> {
    let desktop_dir = desktop_dir().unwrap_or_else(|| PathBuf::from("."));
    let selected_folder = window
          .dialog()
          .file()
          .set_directory(desktop_dir)
          .pick_folder()
          .await;

    match selected_folder {
      Some(folder_path) => Ok(Some(folder_path.to_string_lossy().to_string())),
      None => Ok(None),

    }
}