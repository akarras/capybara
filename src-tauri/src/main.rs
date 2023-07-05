// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::Client;
use tauri::State;
use tauri_plugin_log::LogTarget;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_http(client: State<'_, Client>, url: String) -> Result<String, String> {
    Ok(client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn post_http(client: State<'_, Client>, url: String, body: String) -> Result<String, String> {
    Ok(client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?)
}

fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .manage(Client::new())
        .invoke_handler(tauri::generate_handler![greet, get_http, post_http])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
