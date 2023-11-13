// these have to be public so that the tests in /tests can use this
pub mod accounting;
pub mod datafile;
pub mod investing;

pub use crate::accounting::accounting_month::AccountingMonth;
pub use crate::accounting::Accounting;
pub use crate::datafile::DataFile;
pub use crate::investing::depot_element::DepotElement;

// TODO check what has to be pub

use csv::ReaderBuilder;
use investing::inv_variant::InvestmentVariant;
use investing::inv_year::InvestmentYear;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

// use tinyrand::Rand;
// use tinyrand::RandRange;
// use tinyrand::Seeded;
// use tinyrand::StdRand;
// use tinyrand_std::ClockSeed;

pub struct SanitizeInput;
impl SanitizeInput {
    /// Round to two decimal places and return absolute value
    pub fn monetary_f64_to_f64(float: f64) -> f64 {
        (float.abs() * 100.0).round() / 100.0
    }

    /// - Can parse xx.x and xx,x
    /// - Ignores everything thats not a digit or `.` `,`
    /// - Rounds to two decimal places
    /// - Returns absolute value
    ///
    /// - Error String contains descriptive message
    pub fn monetary_string_to_f64(string: &String) -> Result<f64, String> {
        let mut filtered = string.clone().replace(",", ".");
        filtered.retain(|c| c == '.' || c.is_ascii_digit());

        return match filtered.parse::<f64>() {
            Ok(expenses) => Ok(Self::monetary_f64_to_f64(expenses)),
            Err(e) => Err(format!("{:?} could not be parsed as a f64: {}", filtered, e)),
        };
    }
}

pub fn print_accounting_table(year_nr: u16) {
    let datafile = DataFile::read();
    let year = match datafile.accounting.history.get(&year_nr) {
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
    println!("The goal is to spend less than {} % of monthly income", datafile.accounting.goal * 100.0);
    println!("");
    println!(
        " {:^7} | {:^10} | {:^10} | {:^10} | {:^10} | {}",
        "Month", "Income", "Expenses", "Difference", "Percentage", "Goal met?"
    );
    println!(" {:-^7} | {:-^10} | {:-^10} | {:-^10} | {:-^10} | {:-^9}", "", "", "", "", "", ""); // divider
    for month in &year.months {
        let goal_met: &str = match (month.percentage_1() * 100.0) as u64 {
            0 => "-", // dont show true/false if there is no value
            _ => match month.percentage_1() <= datafile.accounting.goal {
                true => "true",
                false => "false",
            },
        };

        println!(
            " {:4} {:>2} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {}",
            year.year_nr,
            month.month_nr(),
            month.income(),
            month.expenses(),
            month.difference(),
            month.percentage_100(),
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

    // TODO do AVG and Median

    // Sum
    let year_diff: f64 = year.get_sum_income() - year.get_sum_expenses();
    let year_perc: f64 = (year.get_sum_expenses() / year.get_sum_income()) * 100.0;

    let months_with_goal_hit = year
        .months
        .iter()
        .filter(|&m| (m.percentage_1() <= datafile.accounting.goal) && m.percentage_1() != 0.0)
        .count() as f32;
    let months_with_data = year.months.iter().filter(|&m| *m != AccountingMonth::default(m.month_nr())).count() as f32;
    let goals_over_months = format!("{} / {}", months_with_goal_hit, months_with_data);

    println!(
        " {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {:^9}",
        "Sum",
        year.get_sum_income(),
        year.get_sum_expenses(),
        year_diff,
        year_perc,
        goals_over_months,
    );

    // AVG
    let goals_in_year_perc = format!("{:3.0} %", (months_with_goal_hit / months_with_data) * 100.0);
    println!(
        " {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {:^9}",
        "Avg", "", "", "", "", goals_in_year_perc
    );

    // Median
    println!(" {:>7} | {:>10.2} | {:>10.2} | {:>10.2} | {:>8.0} % | {:^9}", "Median", "", "", "", "", "-");
    println!("");
}

pub fn accounting_input_manual(income: f64, expenses: f64, month_nr: u8, year_nr: u16) {
    let mut datafile = DataFile::read();

    let calc_difference: f64 = income - expenses;
    let calc_percentage: f64 = expenses / income;
    println!("Difference: {}, Percentage: {}", calc_difference, calc_percentage);

    datafile
        .accounting
        .add_or_get_year(year_nr)
        .insert_or_overwrite_month(AccountingMonth::new(month_nr, income, expenses, String::new()));

    datafile.write();
}

pub fn generate_depot_entry() {
    let mut datafile = DataFile::read();

    datafile
        .investing
        .depot
        .insert(String::from("name 123"), DepotElement::default(InvestmentVariant::Stock));

    match datafile.investing.depot.get_mut("name 123") {
        Some(investment) => investment.history.insert(2023, InvestmentYear::default(2023)),
        None => panic!("Just added value was not found!"),
    };
}

pub fn get_csv_contents_with_header(path: &PathBuf) -> Vec<Vec<String>> {
    // open file for reading
    let mut file: File = match File::options().read(true).truncate(false).open(path) {
        Ok(file) => file,
        Err(_) => panic!("Could not open {:?}", path),
    };

    // read file content
    let content_string: String = {
        let mut temp: String = String::new();
        file.read_to_string(&mut temp).expect(format!("Error reading file {:?}", path).as_str());
        temp
    };

    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(content_string.as_bytes());

    // get headers
    let header: Vec<String> = match reader.headers() {
        Ok(val) => val.iter().map(|val| val.to_string()).collect(),
        Err(e) => panic!("Could not get CSV Header: {e}"),
    };

    // Get values
    let mut content_vec: Vec<Vec<String>> = Vec::new();
    for record in reader.records() {
        let line: Vec<String> = record.expect("Could not transform csv record").iter().map(|s| s.to_string()).collect();
        content_vec.push(line);
    }

    content_vec.insert(0, header);

    return content_vec;
}

pub fn investing_new_depot_element(name: String, depot_element: DepotElement) {
    let mut datafile = DataFile::read();
    datafile.investing.add_depot_element(name, depot_element);
    datafile.write();
}

pub fn is_depot_empty() -> bool {
    let datafile = DataFile::read();
    return datafile.investing.depot.is_empty();
}

// ================================================== Private ================================================== //

fn _read_csv_to_string(path: &PathBuf) -> String {
    // open file for reading
    let mut file: File = match File::options().read(true).truncate(false).open(path) {
        Ok(file) => file,
        Err(_) => panic!("Could not open {:?}", path),
    };

    // read file content
    let content: String = {
        let mut temp: String = String::new();
        match file.read_to_string(&mut temp) {
            Ok(v) => v,
            Err(_) => panic!("Error reading file {:?}", path),
        };
        temp
    };

    return content;
}

// /// return values
// /// - income, expenses, month, year
// pub fn _generate_random_input() -> (f64, f64, u8, u16) {
//     let seed = ClockSeed::default().next_u64();
//     let mut rand = StdRand::seed(seed);
//     let rand_month: u8 = rand.next_range(1 as usize..13 as usize) as u8;
//     let rand_year: u16 = rand.next_range(2000 as usize..2024 as usize) as u16;
//     let rand_income: f64 = rand.next_u16() as f64 / 11.11;
//     let rand_expenses: f64 = rand.next_u16() as f64 / 11.11;
//     return (rand_income, rand_expenses, rand_month, rand_year);
// }
