mod structs;

use std::env::args;
use std::f32::consts::E;
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
        3 => match args[1].as_str() {
            "-o" => match args[2].parse::<u16>() {
                Ok(year) => print_table(year),
                Err(e) => {
                    println!("{:?} could not be parsed as a int: {}", args[2], e);
                    print_cmd_usage(&args[0]);
                }
            },
            _ => print_cmd_usage(&args[0]),
        },
        5 => match args[1].as_str() {
            "-csv" => {
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
            _ => print_cmd_usage(&args[0]),
        },
        6 => match args[1].as_str() {
            "-i" => {
                let income = match args[2].parse::<f64>() {
                    Ok(income) => income,
                    Err(e) => {
                        println!("{:?} could not be parsed as a f64: {}", args[4], e);
                        print_cmd_usage(&args[0]);
                    }
                };
                let expenses = match args[3].parse::<f64>() {
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
        },
        _ => print_cmd_usage(&args[0]),
    }
}

fn print_table(year: u16) {
    todo!();
}

fn input_from_csv(path: &Path, year: u16, month: u8) {
    todo!();
}

fn print_cmd_usage(cmd: &String) -> ! {
    println!("Usage:");
    println!("\t{} [ -csv | -i | -o ]", cmd);
    println!("");
    println!("1. Provide new data to save for later use (overwrites existing data)");
    println!("1.1 Extract income and expenses from a csv file and define the year and month to which the data should be assigned");
    println!("\t{} -csv  [file (string)]   [year (int)] [month (int)]", cmd);
    println!("\t{} -csv  path/to/file.csv      2023           7", cmd);
    println!("\t{} -csv 'path/to/file.csv'     2023           7", cmd);
    println!("");
    println!("1.2 Define all input values manually");
    println!("\t{} -i [income (int/float)] [expenses (int/float)] [year (int)] [month (int)]", cmd);
    println!("\t{} -i       1111.11               2222.22             2023           7      ", cmd);
    println!("");
    println!("2. Output table with calculated values for one year");
    println!("\t{} -o [year (int)]", cmd);
    println!("\t{} -o     2023    ", cmd);
    println!("");

    exit(0);
}

fn input_manual(income: f64, expenses: f64, month_nr: u8, year_nr: u16) {
    // let (input_income, input_expenses, input_month_nr, input_year_nr): (f64, f64, u8, u16) = generate_random_input();
    // println!("in {}, out {}, month {}, year {}", input_income, input_expenses, input_month_nr, input_year_nr);

    let calc_difference: f64 = income - expenses;
    let calc_percentage: f64 = expenses / income;
    println!("diff {}, perc {}", calc_difference, calc_percentage);

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
fn generate_random_input() -> (f64, f64, u8, u16) {
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);
    let rand_month: u8 = rand.next_range(1 as usize..13 as usize) as u8;
    let rand_year: u16 = rand.next_range(2000 as usize..2024 as usize) as u16;
    let rand_income: f64 = rand.next_u16() as f64 / 11.11;
    let rand_expenses: f64 = rand.next_u16() as f64 / 11.11;
    return (rand_income, rand_expenses, rand_month, rand_year);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::Year;

    #[test]
    fn month_compare() {
        const MONTH: u8 = 1;
        const YEAR: u16 = 2000;

        let mut ymlfile = YamlFile {
            version: 1,
            goal: 0.0,
            years: vec![Year {
                year_nr: YEAR,
                income: 0.0,
                expenses: 0.0,
                months: Month::default_months(),
            }],
        };

        match ymlfile.years.iter().position(|y| y.year_nr == YEAR) {
            Some(index) => match ymlfile.years.get_mut(index) {
                Some(ymlyear) => {
                    let month = &mut ymlyear.months[MONTH as usize - 1];

                    // I just created this test because I wasn't sure that this comparison is done correctly
                    // other languages might have compared the datatype of both sides and would always say its the same
                    assert!(*month == Month::default(month.month_nr));
                    assert_ne!(*month, Month::default(month.month_nr + 1));
                }
                None => (),
            },
            None => (),
        }
    }
}
