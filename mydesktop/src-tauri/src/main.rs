// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use center_api;

mod simple_note;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn init(path: String) -> Result<(), simple_note::Error> {
    simple_note::init(simple_note::Config {
        path,
    })
}

use center_api::db::sqlite::{RawRows, Field};

#[tauri::command]
fn test_insert() {
    let mut rows = RawRows::new(vec![
        "a".to_string(),
        "b".to_string(),
    ]);
    rows.add_row(vec![Field::Integer(1), Field::Integer(2)]);
    let conn = sqlite::open("mydesktop.sqlite3").unwrap();

    rows.insert(conn, "test").unwrap();
}

#[tauri::command]
fn add_note() -> Result<String, simple_note::Error> {
    simple_note::add_note()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test_insert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
