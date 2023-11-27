// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;

mod investing;

use crate::investing::depot_entry_table::*;
use finanzbuch_lib::DataFile;
use std::sync::Mutex;

lazy_static! {
    /// `let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");`
    pub static ref DATAFILE_GLOBAL: Mutex<DataFile> = Mutex::new(DataFile::read());
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_depot_entry_table_cell,
            get_depot_entry_table_html,
            get_depot_entry_list_html,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Only commands regarding the html thats in index.html go here (so mostly only things for the NavBar)

#[tauri::command]
fn get_depot_entry_list_html() -> String
{
    let mut all_buttons: String = String::new();
    let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");

    for (key, entry) in datafile.investing.depot.iter() {
        let name = entry.name();
        let key_val = *key;

        all_buttons.push_str(
            format!(
                r#"
                <button id="depotEntryBtn-{key_val}" class="nav2" onclick="getDepotEntryHtml()">{name}</button>
                "#,
            )
            .as_str(),
        )
    }

    return all_buttons;
}
