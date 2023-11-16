mod accounting;
mod investing;

use crate::accounting::*;
use crate::investing::*;
use dialoguer::theme::ColorfulTheme;
use dialoguer::*;
use std::process::exit;

// NOTE: Since dialoguer will sometimes remove lines from the terminal that were visible before (eg. while selecting something)
// It is more reliable to use a \n at the start of each println!() to create some space

// TODO improve texts
// TODO move stuff from lib.rs to main.rs thats not actually lib.rs code

fn main()
{
    println!(
        "You can cancel at every moment using Ctrl+C, because data is only written at the moment one dialogue is finished.\n\
        Options with ! are not yet implemented."
    );

    // TODO what about removing data??

    let selections = &[
        "Exit",
        "Accounting - In:   Import values for one month from csv file",        // 1
        "Accounting - In:   Manually input values for one month",              // 2
        "Accounting - Out:  Output a table and graph for one year",            // 3
        "Investing - In:    Create new entry in depot",                        // 4
        "Investing - In:  ! Set values for comparisons",                       // 5
        "Investing - In:    Add new savings plan to one depot entry",          // 6
        "Investing - In:  ! Modify one savings plan of one depot entry",       // 7
        "Investing - In:  ! Input values of one depot element",                // 8
        "Investing - Out: ! Output depot overview of one year",                // 9
        "Investing - Out: ! Output depot overview of a specific timeframe",    // 10
        "Investing - Out:   Output all saving plans of one depot entry",       // 11
        "Investing - Out: ! Output one year of one depot element",             // 12
        "Investing - Out: ! Output a specific timeframe of one depot element", // 13
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

            1 => csv_import(),
            2 => manual_input(),
            3 => table_graph_output(),

            4 => new_depot_entry(),
            5 => todo!(),
            6 => add_savings_plan(),
            7 => todo!(),
            8 => todo!(),
            9 => todo!(),
            10 => todo!(),
            11 => output_savings_plan(),
            12 => todo!(),
            13 => todo!(),
            _ => unreachable!(),
        }
    }
}
