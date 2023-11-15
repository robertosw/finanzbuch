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

/// Returns all of the csv cells like this: `Lines<Cells>`
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
