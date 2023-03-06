// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest;
use tauri::SystemTray;
use tauri::{CustomMenuItem, Manager, SystemTrayMenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn generate() -> String {
    let request = reqwest::get("https://labs.bible.org/api/?passage=random&type=json").await;

    let text = match request {
        Err(_why) => return String::from("Failed to fetch bible verse"),
        Ok(res) => res.text().await,
    };

    match text {
        Err(_why) => return String::from("Error while reading response."),
        Ok(text) => {
            return text;
        }
    }
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    println!("Closing splashscreen.");
    // Close splashscreen
    // if let Some(splashscreen) = window.get_window("splashscreen") {
    window.get_window("splashscreen").unwrap().close().unwrap();
    // }
    // Show main window
    window.get_window("bible-gui").unwrap().show().unwrap();
}

fn main() {
    let generate_item = CustomMenuItem::new("generate".to_string(), "Generate");
    let tray_menu = SystemTrayMenu::new().add_item(generate_item);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![close_splashscreen, generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
