// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;

mod investing;

use crate::investing::depot_entry_table::*;
use finanzbuch_lib::DataFile;
use std::sync::Mutex;

lazy_static! {

    /// #### Entire data:
    /// ```Rust
    /// let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    /// ```
    ///
    /// #### Read access to one thing (clone() is required)
    /// ```Rust
    /// let depot = {
    ///     let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    ///     datafile.investing.depot.clone()
    /// };
    /// ```
    pub static ref DATAFILE_GLOBAL: Mutex<DataFile> = Mutex::new(DataFile::read());
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_depot_entrys_previous_year,
            get_depot_entry_list_html,
            get_depot_entry_table_html,
            set_depot_entry_table_cell,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn add_depot_entrys_previous_year(depot_entry_hash: String) -> bool
{
    let Ok(depot_entry_hash) = depot_entry_hash.parse::<u64>() else {
        return false;
    };

    return true;
}

// Only commands regarding the html thats in index.html go here (so mostly only things for the NavBar)

#[tauri::command]
fn get_depot_entry_list_html() -> String
{
    let mut all_buttons: String = String::new();
    let depot = {
        let datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
        datafile.investing.depot.clone()
    };

    for (hash, entry) in depot.iter() {
        let name = entry.name();
        let key_val = *hash;

        all_buttons.push_str(
            format!(
                r#"
                <button id="depotEntryBtn-{key_val}" name="{hash}" class="nav2" onclick="getDepotEntryTableHtml()">{name}</button>
                "#,
            )
            .as_str(),
        )
    }

    return all_buttons;
}
