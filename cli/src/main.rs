mod accounting;
mod investing;

use crate::accounting::*;
use crate::investing::*;
use dialoguer::{theme::ColorfulTheme, *};
use std::process::exit;

// NOTE: Since dialoguer will sometimes remove lines from the terminal that were visible before (eg. while selecting something)
// It is more reliable to use a \n at the start of each println!() to create some space

// TODO improve texts
// TODO move stuff from lib.rs to main.rs thats not actually lib.rs code

fn main() {
    println!("You can cancel at every moment using Ctrl+C.\nData is only written at the moment one dialogue is finished.");

    let selections = &[
        "Exit",
        "Accounting: Import values for one month from csv file", // 1
        "Accounting: Manually input values for one month",       // 2
        "Accounting: Output a table for one year",               // 3
        "Investing: Create new entry in depot",                  // 4
        "Investing: Set values for comparisons",                 // 5
        "Investing: Add or modify savings plan",                 // 6
        "Investing: Output overview of the last 12 months",      // 7
        "Investing: Output overview of a specific timeframe",    // 8
    ];

    loop {
        println!();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select an option")
            .default(1)
            .items(selections)
            .interact()
            .unwrap();

        match selection {
            0 => exit(0),
            1 => accounting_csv_import(),
            2 => accounting_manual_input(),
            3 => accounting_table_output(),
            4 => cli_investing_new_depot_entry(),
            5 => cli_investing_set_comparisons(),
            6 => cli_investing_modify_savings_plan(),
            7 => investing_output_last_12_months(),
            8 => investing_output_specific_timeframe(),
            _ => unreachable!(),
        }
    }
}
