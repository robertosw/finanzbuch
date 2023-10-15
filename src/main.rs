use std::env::args;
use std::path::Path;
use std::process::exit;

use finance_yaml::csv_reader::input_month_from_csv;
use finance_yaml::input_manual;
use finance_yaml::print_table;
use finance_yaml::YamlFile;

// according to https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#extracting-logic-from-main
// the main function should be used for everything that has to be done before the program can really start
// the logic should be in lib.rs
//
// main should also be small and simple enough, that it can be "tested" by reading the code
// there shouldn't be the need to write tests for main, because there shouldn't be complicated logic here

enum CliTask {
    TableOutput,
    InputMonthFromCsv,
    ManualInput,
    UnknownCommand,
    WrongUsage,
}

// TODO write own panic macro that does not output lines and compiler message (panic_release!)

fn main() {
    let mut ymlfile = YamlFile::new();
    ymlfile.init();

    let args: Vec<String> = args().collect();

    match parse_task(&args) {
        CliTask::TableOutput => print_table(&mut ymlfile, parse_args_for_table_output(&args)),
        CliTask::InputMonthFromCsv => {
            let (path, year_nr, month_nr) = parse_args_for_csv_input(&args);
            input_month_from_csv(&mut ymlfile, &path, year_nr, month_nr);
        }
        CliTask::ManualInput => {
            let (income, expenses, month_nr, year_nr): (f64, f64, u8, u16) = parse_args_for_manual_input(&args);
            input_manual(&mut ymlfile, income, expenses, month_nr, year_nr);
        }
        CliTask::UnknownCommand => print_cmd_usage(),
        CliTask::WrongUsage => print_cmd_usage(),
    }
}

/// - Check which command arguments have been given and find out which task has to be done
/// - Correct Task is only returned if the correct amount of cmd arguments for this task have been provided, content is not checked however
fn parse_task(args: &Vec<String>) -> CliTask {
    match args.len() {
        1 | 2 => return CliTask::WrongUsage,
        _ => (),
    };

    match args[1].as_str() {
        "-o" => match args.len() - 2 {
            1 => return CliTask::TableOutput,
            _ => return CliTask::WrongUsage,
        },
        "-csv" => match args.len() - 2 {
            3 => return CliTask::InputMonthFromCsv,
            _ => return CliTask::WrongUsage,
        },
        "-i" => match args.len() - 2 {
            4 => return CliTask::ManualInput,
            _ => return CliTask::WrongUsage,
        },
        _ => return CliTask::UnknownCommand,
    }
}

/// - try to parse the command line arguments for this task
/// - Returns `(income, expenses, month_nr, year_nr): (f64, f64, u8, u16)`
fn parse_args_for_manual_input(args: &Vec<String>) -> (f64, f64, u8, u16) {
    // filter for number
    let mut arg2 = args[2].clone().replace(",", ".");
    arg2.retain(|c| c == '.' || c.is_numeric());
    let income = match arg2.parse::<f64>() {
        Ok(income) => income,
        Err(e) => panic!("{:?} could not be parsed as a f64: {}", arg2, e),
    };

    // filter for number
    let mut arg3 = args[3].clone().replace(",", ".");
    arg3.retain(|c| c == '.' || c.is_numeric());
    let expenses = match arg3.parse::<f64>() {
        Ok(expenses) => expenses,
        Err(e) => panic!("{:?} could not be parsed as a f64: {}", arg3, e),
    };

    let year = match args[4].parse::<u16>() {
        Ok(year) => year,
        Err(e) => panic!("{:?} could not be parsed as a u16: {}", args[4], e),
    };
    let month = match args[5].parse::<u8>() {
        Ok(month) => month,
        Err(e) => panic!("{:?} could not be parsed as a u8: {}", args[5], e),
    };

    return (income, expenses, month, year);
}

/// - try to parse the command line arguments for this task
/// - Returns `(path, year_nr, month_nr): (&Path, u16, u8)`
fn parse_args_for_csv_input(args: &Vec<String>) -> (&Path, u16, u8) {
    let csv_file_path: &Path = {
        let path = Path::new(args[2].as_str());
        let ext = match path.extension() {
            Some(ext) => ext,
            None => panic!("{:?} does not point to a .csv file", args[2]),
        };

        match path.is_file() && (ext == "csv") {
            true => path,
            false => panic!("{:?} does not point to a .csv file", args[2]),
        }
    };

    let year = match args[3].parse::<u16>() {
        Ok(year) => year,
        Err(e) => panic!("{:?} could not be parsed as a u16: {}", args[3], e),
    };
    let month = match args[4].parse::<u8>() {
        Ok(month) => month,
        Err(e) => panic!("{:?} could not be parsed as a u8: {}", args[4], e),
    };

    return (csv_file_path, year, month);
}

/// - try to parse the command line arguments for this task
/// - Returns `year: u16`
fn parse_args_for_table_output(args: &Vec<String>) -> u16 {
    match args[2].parse::<u16>() {
        Ok(year) => return year,
        Err(e) => panic!("{:?} could not be parsed as a u16: {}", args[2], e),
    }
}

/// Explain the user how to use this command
fn print_cmd_usage() -> ! {
    let args: Vec<String> = args().collect();
    let cmd = args.get(0).unwrap();

    println!("Usage:");
    println!("\t{} [ -csv | -i | -o ]", cmd);
    println!("");
    println!("1. Provide new data to save for later use (overwrites existing data)");
    println!("  1.1 Extract income and expenses from a csv file and define the year and month to which the data should be assigned");
    println!("\t{} -csv  [file (string)]   [year (int)] [month (int)]", cmd);
    println!("\t{} -csv  path/to/file.csv      2023           7", cmd);
    println!("\t{} -csv 'path/to/file.csv'     2023           7", cmd);
    println!("");
    println!("  1.2 Define all input values manually");
    println!("\t{} -i [income (int/float)] [expenses (int/float)] [year (int)] [month (int)]", cmd);
    println!("\t{} -i       1111.11               2222.22             2023           7      ", cmd);
    println!("");
    println!("2. Output table with calculated values for one year");
    println!("\t{} -o [year (int)]", cmd);
    println!("\t{} -o     2023    ", cmd);
    println!("");

    exit(0);
}
