// according to https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#extracting-logic-from-main
// the main function should be used for everything that has to be done before the program can really start
// the logic should be in lib.rs
//
// main should also be small and simple enough, that it can be "tested" by reading the code
// there shouldn't be the need to write tests for main, because there shouldn't be complicated logic here
use std::str::FromStr;
use std::{path::PathBuf, process::exit};

use dialoguer::{theme::ColorfulTheme, *};
use finance_yaml::{investing::inv_variant::InvestmentVariant, *};

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
            2 => cli_accounting_manual_input(),
            3 => cli_accounting_table_output(),
            4 => cli_investing_new_depot_entry(),
            5 => cli_investing_set_comparisons(),
            6 => cli_investing_modify_savings_plan(),
            _ => unreachable!(),
        }
    }
}

fn accounting_csv_import() {
    println!("\nThis dialogue allows you to import values from a csv file and insert them into a selected month.\n");
    let csv_path: PathBuf = loop {
        let path_str: String = Input::new().with_prompt("Path to csv file").interact_text().unwrap();

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

    let csv: Vec<Vec<String>> = get_csv_contents_with_header(&csv_path);
    let csv_headers: &Vec<String> = match csv.get(0) {
        Some(v) => v,
        None => {
            println!("CSV file is empty");
            return;
        }
    };
    let csv_first_line: &Vec<String> = match csv.get(1) {
        Some(v) => v,
        None => {
            println!("CSV has no content");
            return;
        }
    };

    let selection_items: Vec<String> = csv_first_line
        .iter()
        .map(|val| String::from(csv_headers.get(0).unwrap().clone() + "; " + val.as_str()))
        .collect();

    let selected_col = Select::new()
        .with_prompt(
            "\nPlease choose the column which contains the monetary values you want to import.\n\
            Each line below represents one column of the csv file in this format: 'Header; Content of first line'",
        )
        .items(&selection_items)
        .interact()
        .unwrap();

    println!("\nIn which year and month do you want to import this data?");
    let year_nr: u16 = Input::new().with_prompt("Year").interact_text().unwrap();
    let month_nr: u8 = Input::new().with_prompt("Month").interact_text().unwrap();
    // TODO note

    // User input done, save data
    let mut datafile = DataFile::read();

    let mut monetary_csv_values: Vec<f64> = Vec::new();
    for entry in csv {
        let value_f64: f64 = SanitizeInput::monetary_string_to_f64(&entry[selected_col]).unwrap();
        monetary_csv_values.push(value_f64);
    }

    // Sum up income and expenses
    let sum_positives: f64 = monetary_csv_values.iter().filter(|&v| v > &0.0).sum();
    let sum_negatives: f64 = monetary_csv_values.iter().filter(|&v| v < &0.0).sum();

    let acc_year = datafile.accounting.add_or_get_year(year_nr);
    acc_year.months[month_nr as usize - 1].set_income(sum_positives);
    acc_year.months[month_nr as usize - 1].set_expenses(sum_negatives);

    datafile.write(DataFile::home_path());
    println!(" --- Importing csv data done ---");
}

fn cli_accounting_manual_input() {
    println!("Adding values into given year and month.");
    let year: u16 = Input::new().with_prompt("Year").interact_text().unwrap();
    let month: u8 = Input::new().with_prompt("Month").interact_text().unwrap();
    let income: f64 = SanitizeInput::monetary_string_to_f64(&Input::new().with_prompt("Income").interact_text().unwrap()).unwrap();
    let expenses: f64 = SanitizeInput::monetary_string_to_f64(&Input::new().with_prompt("Expenses").interact_text().unwrap()).unwrap();
    // TODO note

    println!("Saving In: {income} Out: {expenses} to {year} {month}");
    accounting_input_manual(income, expenses, month, year);
}

fn cli_accounting_table_output() {
    println!("Choose a year to display.");
    let year: u16 = Input::new().with_prompt("Year").interact_text().unwrap();
    print_accounting_table(year);
}

fn cli_investing_new_depot_entry() {
    println!("Please specify a name for this depot entry.");
    let name: String = Input::new().allow_empty(false).with_prompt("Name").interact_text().unwrap();

    let variants: Vec<&str> = vec!["Stock", "Fund", "Etf", "Bond", "Option", "Commoditiy", "Crypto"];
    let selection: usize = Select::new().with_prompt("Select a type").items(&variants).interact().unwrap();
    investing_new_depot_element(name, DepotElement::default(InvestmentVariant::from_str(variants[selection]).unwrap()));

    println!(" --- Creating new depot entry done ---");
}
fn cli_investing_set_comparisons() {
    todo!(); // TODO
    println!(" --- Modifying comparisons done ---");
}
fn cli_investing_modify_savings_plan() {
    println!(
        "\n\
        This dialogue option allows you to create a new savings plan or edit an existing one.\n\n\
        - Both the start and end dates are included.\n\
        - The end date of one savings plan can be left blank.\n\
        - A new savings plan must be created each time the monthly payment is changed. \n\
        Example: From the beginning of January 2023 until the end of June 2023 you deposited €10 per month, \n\
        but from the beginning of July you deposited €20. To do this, you need to create a savings plan with a\n\
        start date of 2023-1 and an end date of 2023-6 (10€), and another with a start date of 2023-7 and any end date (20€).\n"
    );

    if is_depot_empty() {
        println!("Your depot is entry. Please create a depot entry first.");
        cli_investing_new_depot_entry();
    }

    let variants: Vec<&str> = vec!["Create", "Modify"];
    let _selection: usize = Select::new()
        .with_prompt("Do you want to create a new savings plan or modify an existing one?")
        .default(0)
        .items(&variants)
        .interact()
        .unwrap();

    if _selection == 0 {
        let _start_year: u16 = Input::new().with_prompt("Start year").interact_text().unwrap();
        let _start_month: u8 = Input::new().with_prompt("Start month").interact_text().unwrap();
        let _end_year: u16 = Input::new().with_prompt("End year").interact_text().unwrap();
        let _end_month: u8 = Input::new().with_prompt("End month").interact_text().unwrap();

        let variants: Vec<&str> = vec!["Monthly", "Annually"];
        let _selection: usize = Select::new().with_prompt("Select your interval").items(&variants).interact().unwrap();

        let _amount: f64 = Input::new().with_prompt("Amount per interval").interact_text().unwrap();

        // TODO do something with this
        todo!();
    } else {
        todo!();
    }

    println!(" --- Modifying savings plan done ---");
}
