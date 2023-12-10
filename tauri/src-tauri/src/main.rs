// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate lazy_static;

mod investing;

use crate::investing::depot_entry_table::*;
use finanzbuch_lib::investing::inv_variant::InvestmentVariant;
use finanzbuch_lib::DataFile;
use finanzbuch_lib::DepotEntry;
use std::str::FromStr;
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
            add_depot_entry,
            add_depot_entrys_previous_year,
            get_html_add_depot_entry_form,
            get_depot_entry_list_html,
            get_depot_entry_table_html,
            set_depot_entry_table_cell,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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

    // Button to add one
    all_buttons.push_str(
        format!(
            r#"
            <button id="depotEntryBtnAdd" class="nav2" onclick="navBarBtnAddDepotEntry()">Add entry</button>
            "#,
        )
        .as_str(),
    );

    return all_buttons;
}

#[tauri::command]
fn add_depot_entry(name: String, variant: String) -> bool
{
    if name.is_empty() {
        return false;
    }

    let variant = match InvestmentVariant::from_str(variant.as_str()) {
        Ok(v) => v,
        Err(e) => {
            println!("Error converting String into InvestmentVariant: {e}");
            return false;
        }
    };
    let mut datafile = DATAFILE_GLOBAL.lock().expect("DATAFILE_GLOBAL Mutex was poisoned");
    datafile
        .investing
        .add_depot_entry(name.as_str(), DepotEntry::default(name.as_str(), variant));

    datafile.write();
    return true;
}

#[tauri::command]
fn get_html_add_depot_entry_form() -> String
{
    let mut options: String = String::new();

    for variant in InvestmentVariant::into_iter() {
        let variant_str = variant.to_string();
        options.push_str(format!(r#" <option value="{variant_str}">{variant_str}</option> "#,).as_str());
    }

    return format!(
        r#"
        <form id="depotEntryAddContainer" onsubmit="addDepotEntryFormSubmit(event)">
            <div class="depotEntryAddElement">
                <label>Name:</label>
                <input type="text" id="depotEntryAdd-Name">
            </div>
            <div class="depotEntryAddElement">
                <label>Variant:</label>
                <select name="depotEntryAdd-Selection" id="depotEntryAdd-Selection">
                    {options}
                </select>
            </div>
            <button type="submit" id="depotEntryAddDoneBtn">Done</button>
        </form>
        "#
    );
}
