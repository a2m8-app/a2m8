#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use typeshare::typeshare;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[typeshare]
pub struct A2M8Config {}

// export type Script = {
//   id: string;
//   name: `${string}.lua`;
//   description: string;
//   startup: boolean;
//   favorite: boolean;
//   content: string;
//   error?: string;
//   status: scriptStatus;
// };
use uuid::Uuid;

#[typeshare]
pub struct A2M8Script {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub startup: bool,
    pub favorite: bool,
    pub content: String,
    pub error: Option<String>,
    pub status: ScriptStatus,
}
#[typeshare]
enum ScriptStatus {
    Running,
    Stopped,
    Ended,
    Error,
}
