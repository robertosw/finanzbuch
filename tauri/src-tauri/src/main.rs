// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;

mod investing;

use crate::investing::depot_entry_table::*;
use finanzbuch_lib::DataFile;
use std::sync::Mutex;

lazy_static! {
    pub static ref DATAFILE_GLOBAL: Mutex<DataFile> = Mutex::new(DataFile::read());
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_depot_entry_table_cell, get_depot_entry_table_html])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
