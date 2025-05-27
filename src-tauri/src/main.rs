#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, Runtime, AppHandle, Window};
use std::path::PathBuf;
use std::fs::read_dir;
use tauri_plugin_dialog::{DialogExt, FilePath};

#[tauri::command]
fn test_fn() -> String {
  "Hello from Rust".to_string()
}
#[tauri::command]
async fn get_wfmdir<R: Runtime>(app: AppHandle<R>) -> Vec<PathBuf> {
    let folder_dialog = app.dialog().file();
    let dialog_result = folder_dialog.blocking_pick_folder();
    
    match dialog_result {
        Some(result_pathbuf) => {
            let path = PathBuf::from(result_pathbuf.to_string()); // Convert FilePath to PathBuf
            match read_dir(path) { // Handle the Result first
                Ok(entries) => { // Now entries is the iterator
                    entries
                        .filter_map(|entry| {
                            let entry = entry.ok()?;
                            let path = entry.path();
                            if path.extension().map_or(false, |ext| ext == "wfm") {
                                Some(path)
                            } else {
                                None
                            }
                        })
                        .collect()
                },
                Err(_) => Vec::new() // Return empty vec if read_dir fails
            }
        },
        None => Vec::new()
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