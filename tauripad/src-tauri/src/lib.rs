// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::fs;

use rfd::FileDialog;
use serde::Serialize;
#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct FileInfo {
    file_path : String,
    file_contents : String
}

#[tauri::command]
fn open_file() -> Option<FileInfo>{

    let files = FileDialog::new()
        .add_filter("text", &["txt", "rs"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file()?;

    
            Some(FileInfo{
                file_path : files.to_str()?.to_owned(),
                file_contents : fs::read_to_string(files.as_path()).ok()?
            })
        
}

use std::fs::OpenOptions;
use std::io::Write;

#[tauri::command]
fn modify_file(file_path:String, file_contents:String){

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to open file");

    if let Err(e) = file.write_all(file_contents.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![open_file,modify_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

