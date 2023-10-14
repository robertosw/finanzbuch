mod structs;

use std::env::args;
use std::path::Path;
use std::process::exit;

use crate::structs::Month;
use crate::structs::YamlFile;
use tinyrand::Rand;
use tinyrand::RandRange;
use tinyrand::Seeded;
use tinyrand::StdRand;
use tinyrand_std::ClockSeed;

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 | 2 => print_cmd_usage(&args[0]),
        _ => (),
    }

    match (args[1].as_str(), args.len() - 2) {
        // output as table
        ("-o", 1) => match args[2].parse::<u16>() {
            Ok(year) => print_table(year),
            Err(e) => {
                println!("{:?} could not be parsed as a int: {}", args[2], e);
                print_cmd_usage(&args[0]);
            }
        },

        // input with csv
        ("-csv", 3) => {
            let csv_file_path: &Path = {
                let path = Path::new(args[2].as_str());
                let ext = match path.extension() {
                    Some(ext) => ext,
                    None => {
                        println!("{:?} does not point to a .csv file", args[2]);
                        print_cmd_usage(&args[0]);
                    }
                };

                if path.is_file() && (ext == "csv") {
                    path
                } else {
                    println!("{:?} does not point to a .csv file", args[2]);
                    print_cmd_usage(&args[0]);
                }
            };
            let year = match args[3].parse::<u16>() {
                Ok(year) => year,
                Err(e) => {
                    println!("{:?} could not be parsed as a int: {}", args[3], e);
                    print_cmd_usage(&args[0]);
                }
            };
            let month = match args[4].parse::<u8>() {
                Ok(month) => month,
                Err(e) => {
                    println!("{:?} could not be parsed as a int: {}", args[4], e);
                    print_cmd_usage(&args[0]);
                }
            };

            input_from_csv(&csv_file_path, year, month);
        }

        //input manually
        ("-i", 4) => {
            let mut arg2 = args[2].clone().replace(",", ".");
            arg2.retain(|c| c == '.' || c.is_numeric());
            let income = match arg2.parse::<f64>() {
                Ok(income) => income,
                Err(e) => {
                    println!("{:?} could not be parsed as a f64: {}", args[4], e);
                    print_cmd_usage(&args[0]);
                }
            };

            let mut arg3 = args[3].clone().replace(",", ".");
            arg3.retain(|c| c == '.' || c.is_numeric());
            let expenses = match arg3.parse::<f64>() {
                Ok(expenses) => expenses,
                Err(e) => {
                    println!("{:?} could not be parsed as a f64: {}", args[4], e);
                    print_cmd_usage(&args[0]);
                }
            };
            let year = match args[4].parse::<u16>() {
                Ok(year) => year,
                Err(e) => {
                    println!("{:?} could not be parsed as a int: {}", args[4], e);
                    print_cmd_usage(&args[0]);
                }
            };
            let month = match args[5].parse::<u8>() {
                Ok(month) => month,
                Err(e) => {
                    println!("{:?} could not be parsed as a int: {}", args[5], e);
                    print_cmd_usage(&args[0]);
                }
            };
            input_manual(income, expenses, month, year);
        }
        _ => print_cmd_usage(&args[0]),
    }
}

fn print_table(year_nr: u16) {
    let ymlfile = YamlFile::read();
    let year = match ymlfile.years.iter().find(|y| y.year_nr == year_nr) {
        Some(year) => year,
        None => {
            println!("There is no data for the year {year_nr}.");
            exit(0);
        }
    };

    // target:
    //    Month  |   Income   |  Expenses  | Difference | Percentage | Goal met?
    //    ------- | ---------- | ---------- | ---------- | ---------- | ---------
    //    2023 01 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 02 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 03 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 04 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 05 |     378.76 |    3445.18 |   -3066.43 |      910 % | false
    //    2023 06 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 07 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 08 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 09 |   12345.00 |  123456.00 | -111111.00 |     1000 % | false
    //    2023 10 |   12345.00 |    1234.00 |   11111.00 |       10 % | true
    //    2023 11 |       0.00 |       0.00 |       0.00 |        0 % | -
    //    2023 12 |    1111.11 |    2222.22 |   -1111.11 |      200 % | false
    //    ------- | ---------- | ---------- | ---------- | ---------- | ---------
    //       2023 |   26179.87 |  130357.40 |          - |          % | -

    // table for months
    println!("");
    println!(
        " {:^7} | {:^10} | {:^10} | {:^10} | {:^10} | {}",
        "Month", "Income", "Expenses", "Difference", "Percentage", "Goal met?"
    );
    println!(" {:-^7} | {:-^10} | {:-^10} | {:-^10} | {:-^10} | {:-^9}", "", "", "", "", "", ""); // divider
    for month in &year.months {
        let goal_met: &str = match (month.percentage * 100.0) as u64 {
            0 => "-", // dont show true/false if there is no value
            _ => match month.percentage <= ymlfile.goal {
                true => "true",
                false => "false",
            },
        };

        println!(
            " {:4} {:>2} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {}",
            year.year_nr,
            month.month_nr,
            month.income,
            month.expenses,
            month.difference,
            month.percentage * 100.0,
            goal_met
        );
    }
    println!("");

    // table for different statics for year
    println!(
        " {:>7} | {:^10} | {:^10} | {:^10} | {:^10} | {}",
        year_nr, "Income", "Expenses", "Difference", "Percentage", "Goal met?"
    );
    println!(" {:-^7} | {:-^10} | {:-^10} | {:-^10} | {:-^10} | {:-^9}", "", "", "", "", "", ""); // divider
    println!(" {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {}", "Sum", "", "", "", "", ""); // TODO
    println!(" {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {}", "Avg", "", "", "", "", ""); // TODO
    println!(" {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {}", "Median", "", "", "", "", ""); // TODO
    println!("");
}

fn print_cmd_usage(cmd: &String) -> ! {
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

fn input_from_csv(_path: &Path, _year: u16, _month: u8) {
    todo!();
}

fn input_manual(income: f64, expenses: f64, month_nr: u8, year_nr: u16) {
    // let (input_income, input_expenses, input_month_nr, input_year_nr): (f64, f64, u8, u16) = generate_random_input();
    // println!("in {}, out {}, month {}, year {}", input_income, input_expenses, input_month_nr, input_year_nr);

    let calc_difference: f64 = income - expenses;
    let calc_percentage: f64 = expenses / income;
    println!("Difference: {}, Percentage: {}", calc_difference, calc_percentage);

    // read file and sort ascending
    let mut ymlfile = YamlFile::read();

    ymlfile.add_or_get_year(year_nr).insert_or_overwrite_month(Month {
        month_nr,
        income,
        expenses,
        difference: calc_difference,
        percentage: calc_percentage,
    });

    // beim einfügen in ein Jahr und Monat überprüfen ob in dem Monat schon Werte waren
    // Wenn nicht, zur Jahres summe einfach die Monatswerte aufaddieren
    // Wenn Monat überschrieben, dann erst Differenz zu vorherigen Werten berechnen, überschreiben und im Jahr aufaddieren
    ymlfile.write();
}

/// return values
/// - income, expenses, month, year
fn _generate_random_input() -> (f64, f64, u8, u16) {
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);
    let rand_month: u8 = rand.next_range(1 as usize..13 as usize) as u8;
    let rand_year: u16 = rand.next_range(2000 as usize..2024 as usize) as u16;
    let rand_income: f64 = rand.next_u16() as f64 / 11.11;
    let rand_expenses: f64 = rand.next_u16() as f64 / 11.11;
    return (rand_income, rand_expenses, rand_month, rand_year);
}

#[cfg(test)]
mod integration_tests {
    mod tests_main;
    mod tests_structs;
}
