// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod investing;

use crate::investing::depot_entry_table::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_depot_entry_table_cell, get_depot_entry_table_html])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
