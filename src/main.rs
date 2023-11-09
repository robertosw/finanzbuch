// according to https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#extracting-logic-from-main
// the main function should be used for everything that has to be done before the program can really start
// the logic should be in lib.rs
//
// main should also be small and simple enough, that it can be "tested" by reading the code
// there shouldn't be the need to write tests for main, because there shouldn't be complicated logic here

use std::{process::exit, thread, time::Duration};

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use finance_yaml::*;

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
        clearscreen::clear().unwrap();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select an option")
            .default(0)
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
    println!("You selected option 1!");
    let _confirmation = Confirm::new().with_prompt("Do you want to continue?").interact().unwrap();
}

fn accounting_manual_input() {
    println!("Adding values for given year and month.");
    let year: u16 = Input::new().with_prompt("Year").interact_text().unwrap();
    let month: u8 = Input::new().with_prompt("Month").interact_text().unwrap();
    let income: f64 = SanitizeInput::monetary_string_to_f64(Input::new().with_prompt("Income").interact_text().unwrap()).unwrap();
    let expenses: f64 = SanitizeInput::monetary_string_to_f64(Input::new().with_prompt("Expenses").interact_text().unwrap()).unwrap();

    println!("Saving In: {income} Out: {expenses} to {year} {month}");
    accounting_input_manual(income, expenses, month, year);

    thread::sleep(Duration::from_secs(3));
}

fn accounting_table_output() {
    println!("You selected option 3!");
    // Add your code here
}

fn investing_new_depot_entry() {}
fn investing_set_comparisons() {}
fn investing_modify_savings_plan() {}


// TODO write own panic macro that does not output lines and compiler message (panic_release!)

// fn main() {
//     let args: Vec<String> = args().collect();

//     match parse_task(&args) {
//         CliTask::AccountingTableOutput => print_accounting_table(parse_args_for_table_output(&args)),
//         CliTask::AccountingInputMonthFromCsv => {
//             let (path, year_nr, month_nr) = parse_args_for_csv_input(&args);
//             accounting_input_month_from_csv(&path, year_nr, month_nr);
//         }
//         CliTask::ManualAccountingInput => {
//             let (income, expenses, month_nr, year_nr): (f64, f64, u8, u16) = parse_args_for_manual_input(&args);
//             accounting_input_manual(income, expenses, month_nr, year_nr);
//         }
//         CliTask::ManualInvestingInput => {
//             todo!()
//         }
//         CliTask::UnknownCommand => print_cmd_usage(),
//         CliTask::WrongUsage => print_cmd_usage(),
//     }
// }

// /// - Check which command arguments have been given and find out which task has to be done
// /// - Correct Task is only returned if the correct amount of cmd arguments for this task have been provided, content is not checked however
// fn parse_task(args: &Vec<String>) -> CliTask {
//     match args.len() {
//         1 | 2 => return CliTask::WrongUsage,
//         _ => (),
//     };

//     match args[1].as_str() {
//         "-o" => match args.len() - 2 {
//             1 => return CliTask::AccountingTableOutput,
//             _ => return CliTask::WrongUsage,
//         },
//         "--csv" | "-c" => match args.len() - 2 {
//             3 => return CliTask::AccountingInputMonthFromCsv,
//             _ => return CliTask::WrongUsage,
//         },
//         "-i" => match args.len() - 2 {
//             4 => return CliTask::ManualAccountingInput,
//             _ => return CliTask::WrongUsage,
//         },
//         _ => return CliTask::UnknownCommand,
//     }
// }

// /// - try to parse the command line arguments for this task
// /// - Returns `(income, expenses, month_nr, year_nr): (f64, f64, u8, u16)`
// fn parse_args_for_manual_input(args: &Vec<String>) -> (f64, f64, u8, u16) {
//

//     let year = match args[4].parse::<u16>() {
//         Ok(year) => year,
//         Err(e) => panic!("{:?} could not be parsed as a u16: {}", args[4], e),
//     };
//     let month = match args[5].parse::<u8>() {
//         Ok(month) => month,
//         Err(e) => panic!("{:?} could not be parsed as a u8: {}", args[5], e),
//     };

//     return (income, expenses, month, year);
// }

// /// - try to parse the command line arguments for this task
// /// - Returns `(path, year_nr, month_nr): (&Path, u16, u8)`
// fn parse_args_for_csv_input(args: &Vec<String>) -> (&Path, u16, u8) {
//     let csv_file_path: &Path = {
//         let path = Path::new(args[2].as_str());
//         let ext = match path.extension() {
//             Some(ext) => ext,
//             None => panic!("{:?} does not point to a .csv file", args[2]),
//         };

//         match path.is_file() && (ext == "csv") {
//             true => path,
//             false => panic!("{:?} does not point to a .csv file", args[2]),
//         }
//     };

//     let year = match args[3].parse::<u16>() {
//         Ok(year) => year,
//         Err(e) => panic!("{:?} could not be parsed as a u16: {}", args[3], e),
//     };
//     let month = match args[4].parse::<u8>() {
//         Ok(month) => month,
//         Err(e) => panic!("{:?} could not be parsed as a u8: {}", args[4], e),
//     };

//     return (csv_file_path, year, month);
// }

// /// - try to parse the command line arguments for this task
// /// - Returns `year: u16`
// fn parse_args_for_table_output(args: &Vec<String>) -> u16 {
//     match args[2].parse::<u16>() {
//         Ok(year) => return year,
//         Err(e) => panic!("{:?} could not be parsed as a u16: {}", args[2], e),
//     }
// }

// /// Explain the user how to use this command
// fn print_cmd_usage() -> ! {
//     let args: Vec<String> = args().collect();
//     let cmd = args.get(0).unwrap();

//     println!("Usage:");
//     println!("\t{} [ -c | -i | -o ]", cmd);
//     println!("");
//     println!("1. Provide new data to save for later use (overwrites existing data)");
//     println!("  1.1 Extract income and expenses from a csv file and define the year and month to which the data should be assigned");
//     println!("\t{} -csv  [file (string)]   [year (int)] [month (int)]", cmd);
//     println!("\t{} -csv  path/to/file.csv      2023           7", cmd);
//     println!("\t{} -csv 'path/to/file.csv'     2023           7", cmd);
//     println!("");
//     println!("  1.2 Define all input values manually");
//     println!("\t{} -i [income (int/float)] [expenses (int/float)] [year (int)] [month (int)]", cmd);
//     println!("\t{} -i       1111.11               2222.22             2023           7      ", cmd);
//     println!("");
//     println!("2. Output table with calculated values for one year");
//     println!("\t{} -o [year (int)]", cmd);
//     println!("\t{} -o     2023    ", cmd);
//     println!("");

//     exit(0);
// }
