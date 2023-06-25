// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use capybara_lemmy_client::post::{GetPosts, PostResponse};
use tauri_plugin_log::LogTarget;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_http(url: String) -> Option<String> {
    Some(reqwest::get(url).await.ok()?.text().await.ok()?)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().targets([LogTarget::Stdout, LogTarget::Webview]).build())
        .invoke_handler(tauri::generate_handler![greet, get_http])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
