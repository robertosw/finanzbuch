// according to https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#extracting-logic-from-main
// the main function should be used for everything that has to be done before the program can really start
// the logic should be in lib.rs
//
// main should also be small and simple enough, that it can be "tested" by reading the code
// there shouldn't be the need to write tests for main, because there shouldn't be complicated logic here

use std::{path::PathBuf, process::exit, str::FromStr};

use dialoguer::{theme::ColorfulTheme, *};
use finance_yaml::{
    csv_reader::accounting_input_month_from_csv,
    investing::{inv_variant::InvestmentVariant, SavingsPlanInterval},
    *,
};

fn main() {
    let selections = &[
        "Exit",
        "Accounting: Import values for one month from csv file", // 1
        "Accounting: Manually input values for one month",       // 2
        "Accounting: Output one year in table view",             // 3
        "Investing: Create new entry in depot",                  // 4
        "Investing: Set values for comparisons",                 // 5
        "Investing: Add or modify savings plan",                 // 6
    ];

    loop {
        // clearscreen::clear().unwrap();
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
            5 => investing_set_comparisons(),
            6 => investing_modify_savings_plan(),
            _ => unreachable!(),
        }
    }
}

fn accounting_csv_import() {
    println!("Adding values into given year and month.");
    let path: PathBuf = loop {
        let path_str: String = Input::new().with_prompt("Path to csv").interact_text().unwrap();

        let path = PathBuf::from(&path_str);
        let ext = match &path.extension() {
            Some(ext) => *ext,
            None => {
                println!("{:?} does not have a extension.", &path_str);
                continue;
            }
        };

        let ext_str = match ext.to_str() {
            Some(str) => str,
            None => {
                println!("The extension of {:?} could not be parsed.", &path_str);
                continue;
            }
        };

        match path.is_file() && (ext_str == "csv") {
            true => break path,
            false => {
                println!("{:?} does not point to a .csv file", path);
                continue;
            }
        };
    };

    let year: u16 = Input::new().with_prompt("Year").interact_text().unwrap();
    let month: u8 = Input::new().with_prompt("Month").interact_text().unwrap();
    // TODO note

    accounting_input_month_from_csv(&path, year, month);
}

fn accounting_manual_input() {
    println!("Adding values into given year and month.");
    let year: u16 = Input::new().with_prompt("Year").interact_text().unwrap();
    let month: u8 = Input::new().with_prompt("Month").interact_text().unwrap();
    let income: f64 = SanitizeInput::monetary_string_to_f64(&Input::new().with_prompt("Income").interact_text().unwrap()).unwrap();
    let expenses: f64 = SanitizeInput::monetary_string_to_f64(&Input::new().with_prompt("Expenses").interact_text().unwrap()).unwrap();
    // TODO note

    println!("Saving In: {income} Out: {expenses} to {year} {month}");
    accounting_input_manual(income, expenses, month, year);

    // thread::sleep(Duration::from_secs(3));
}

fn accounting_table_output() {
    println!("Choose a year to display.");
    let year: u16 = Input::new().with_prompt("Year").interact_text().unwrap();
    print_accounting_table(year);
}

fn investing_new_depot_entry() {
    println!("Please specify a name for this depot entry.");
    let name: String = Input::new().allow_empty(false).with_prompt("Name").interact_text().unwrap();

    let variants: Vec<&str> = vec!["Stock", "Fund", "Etf", "Bond", "Option", "Commoditiy", "Crypto"];
    let selection: usize = Select::new().with_prompt("Select a type").items(&variants).interact().unwrap();
    investing_new_depot_element(name, DepotElement::default(InvestmentVariant::from_str(variants[selection]).unwrap()));
}
fn investing_set_comparisons() {
    todo!(); // TODO
}
fn investing_modify_savings_plan() {
    println!(
        "This adds one savings plan into one depot entry. \
        If you changed the amount that is bought per week / month / year, \
        it is better to create an additional savings plan in the same depot entry for that new amount."
    );

    // TODO check for depot first

    let confirmation = Confirm::new()
        .with_prompt("Do you want to create a new savings plan?")
        .default(true)
        .show_default(true)
        .interact()
        .unwrap();

    if !confirmation {
        return;
    }

    println!(
        "Every savings plan is defined by a start date and an end date (month and year). Both are inclusive. \
        If you want to create a savings plan for the entire year of 2023, the start is 2023-1 and the end is 2023-12."
    );
    println!("This program assumes that each year only has 52 weeks. In reality this is closer to 52.3.");

    let start_year: u16 = Input::new().with_prompt("Start year").interact_text().unwrap();
    let start_month: u8 = Input::new().with_prompt("Start month").interact_text().unwrap();
    let end_year: u16 = Input::new().with_prompt("End year").interact_text().unwrap();
    let end_month: u8 = Input::new().with_prompt("End month").interact_text().unwrap();

    let variants: Vec<&str> = vec!["Weekly", "Monthly", "Annually"];
    let selection: usize = Select::new().with_prompt("Select your interval").items(&variants).interact().unwrap();

    let amount: f64 = Input::new().with_prompt("Amount per interval").interact_text().unwrap();

    // TODO do something with this
}
