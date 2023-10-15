// these have to be public so that the tests in /tests can use this
pub mod csv_reader;
pub mod structs;
pub use crate::structs::config::Config;
pub use crate::structs::Month;

use std::process::exit;
use tinyrand::Rand;
use tinyrand::RandRange;
use tinyrand::Seeded;
use tinyrand::StdRand;
use tinyrand_std::ClockSeed;

pub fn print_table(ymlfile: &mut Config, year_nr: u16) {
    let year = match ymlfile.years.get(&year_nr) {
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
    println!(
        " {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {}",
        "Sum", year.income_sum, year.expenses_sum, "", "", ""
    ); // TODO
    println!(" {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {}", "Avg", "", "", "", "", ""); // TODO
    println!(" {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {}", "Median", "", "", "", "", ""); // TODO
    println!("");
}

pub fn input_manual(ymlfile: &mut Config, income: f64, expenses: f64, month_nr: u8, year_nr: u16) {
    let calc_difference: f64 = income - expenses;
    let calc_percentage: f64 = expenses / income;
    println!("Difference: {}, Percentage: {}", calc_difference, calc_percentage);

    ymlfile.add_or_get_year(year_nr).insert_or_overwrite_month(Month {
        month_nr,
        income,
        expenses,
        difference: calc_difference,
        percentage: calc_percentage,
    });

    ymlfile.write();
}

/// return values
/// - income, expenses, month, year
pub fn _generate_random_input() -> (f64, f64, u8, u16) {
    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);
    let rand_month: u8 = rand.next_range(1 as usize..13 as usize) as u8;
    let rand_year: u16 = rand.next_range(2000 as usize..2024 as usize) as u16;
    let rand_income: f64 = rand.next_u16() as f64 / 11.11;
    let rand_expenses: f64 = rand.next_u16() as f64 / 11.11;
    return (rand_income, rand_expenses, rand_month, rand_year);
}
