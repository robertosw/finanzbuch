use std::env::args;
use std::path::Path;
use std::process::exit;

use finance_yaml::input_from_csv;
use finance_yaml::input_manual;
use finance_yaml::print_table;

// according to https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#extracting-logic-from-main
// the main function should be used for everything that has to be done before the program can really start
// the logic should be in lib.rs
//
// main should also be small and simple enough, that it can be "tested" by reading the code
// there shouldn't be the need to write tests for main, because there shouldn't be complicated logic here

enum CliOption {
    TableOutput,
    CsvInput,
    ManualInput,
    UnknownCommand,
    WrongUsage,
}

fn main() {
    let args: Vec<String> = args().collect();

    match parse_args(&args) {
        CliOption::TableOutput => table_output(&args[2]),
        CliOption::CsvInput => csv_input(&args[2], &args[3], &args[4]),
        CliOption::ManualInput => manual_input(&args),
        CliOption::UnknownCommand => print_cmd_usage(),
        CliOption::WrongUsage => print_cmd_usage(),
    }
}

///
fn parse_args(args: &Vec<String>) -> CliOption {
    match args.len() {
        1 | 2 => return CliOption::WrongUsage,
        _ => (),
    };

    match args[1].as_str() {
        "-o" => match args.len() - 2 {
            1 => return CliOption::TableOutput,
            _ => return CliOption::WrongUsage,
        },
        "-csv" => match args.len() - 2 {
            3 => return CliOption::CsvInput,
            _ => return CliOption::WrongUsage,
        },
        "-i" => match args.len() - 2 {
            4 => return CliOption::ManualInput,
            _ => return CliOption::WrongUsage,
        },
        _ => return CliOption::UnknownCommand,
    }
}

///
fn manual_input(args: &Vec<String>) {
    // filter for number
    let mut arg2 = args[2].clone().replace(",", ".");
    arg2.retain(|c| c == '.' || c.is_numeric());
    let income = match arg2.parse::<f64>() {
        Ok(income) => income,
        Err(e) => {
            println!("{:?} could not be parsed as a f64: {}", args[4], e);
            print_cmd_usage();
        }
    };

    // filter for number
    let mut arg3 = args[3].clone().replace(",", ".");
    arg3.retain(|c| c == '.' || c.is_numeric());
    let expenses = match arg3.parse::<f64>() {
        Ok(expenses) => expenses,
        Err(e) => {
            println!("{:?} could not be parsed as a f64: {}", args[4], e);
            print_cmd_usage();
        }
    };
    let year = match args[4].parse::<u16>() {
        Ok(year) => year,
        Err(e) => {
            println!("{:?} could not be parsed as a int: {}", args[4], e);
            print_cmd_usage();
        }
    };
    let month = match args[5].parse::<u8>() {
        Ok(month) => month,
        Err(e) => {
            println!("{:?} could not be parsed as a int: {}", args[5], e);
            print_cmd_usage();
        }
    };
    input_manual(income, expenses, month, year);
}

///
fn csv_input(arg2: &String, arg3: &String, arg4: &String) {
    let csv_file_path: &Path = {
        let path = Path::new(arg2.as_str());
        let ext = match path.extension() {
            Some(ext) => ext,
            None => {
                println!("{:?} does not point to a .csv file", arg2);
                print_cmd_usage();
            }
        };

        if path.is_file() && (ext == "csv") {
            path
        } else {
            println!("{:?} does not point to a .csv file", arg2);
            print_cmd_usage();
        }
    };
    let year = match arg3.parse::<u16>() {
        Ok(year) => year,
        Err(e) => {
            println!("{:?} could not be parsed as a int: {}", arg3, e);
            print_cmd_usage();
        }
    };
    let month = match arg4.parse::<u8>() {
        Ok(month) => month,
        Err(e) => {
            println!("{:?} could not be parsed as a int: {}", arg4, e);
            print_cmd_usage();
        }
    };
    input_from_csv(&csv_file_path, year, month);
}

///
fn table_output(arg2: &String) {
    match arg2.parse::<u16>() {
        Ok(year) => print_table(year),
        Err(e) => {
            println!("{:?} could not be parsed as a int: {}", arg2, e);
            print_cmd_usage();
        }
    }
}

///
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
