// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod investing_in;
mod investing_out;

use crate::investing_in::*;
use crate::investing_out::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_investing_month_field, get_investing_table_html])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
