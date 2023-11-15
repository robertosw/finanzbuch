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
        "Accounting - In: Import values for one month from csv file",                  // 1
        "Accounting - In: Manually input values for one month",                        // 2
        "Accounting - Out: Output a table and graph for one year",                     // 3
        "Investing - In: Create new entry in depot",                                   // 4
        "Investing - In: Set values for comparisons (not yet)",                        // 5
        "Investing - In: Add or modify savings plan (not yet)",                        // 6
        "Investing - In: Input values of one depot element (not yet)",                 // 7
        "Investing - Out: Output overview of one year (not yet)",                      // 8
        "Investing - Out: Output overview of a specific timeframe (not yet)",          // 9
        "Investing - Out: Output one year of one depot element (not yet)",             // 10
        "Investing - Out: Output a specific timeframe of one depot element (not yet)", // 11
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

            4 => investing_new_depot_entry(),
            5 => todo!(),
            6 => cli_investing_modify_savings_plan(),
            7 => todo!(),
            8 => todo!(),
            9 => todo!(),
            10 => todo!(),
            11 => todo!(),
            _ => unreachable!(),
        }
    }
}
