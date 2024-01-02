// these have to be public so that the tests in /tests can use this
pub mod accounting;
pub mod datafile;
pub mod fast_date;
pub mod investing;

pub use crate::accounting::accounting_month::AccountingMonth;
pub use crate::accounting::Accounting;
pub use crate::datafile::DataFile;
pub use crate::investing::depot::DepotEntry;

// TODO check what has to be pub

use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::Read;
use std::num::ParseFloatError;
use std::path::PathBuf;
use std::time::SystemTime;

// use tinyrand::Rand;
// use tinyrand::RandRange;
// use tinyrand::Seeded;
// use tinyrand::StdRand;
// use tinyrand_std::ClockSeed;

pub struct CurrentDate;
impl CurrentDate
{
    pub fn datetime() -> DateTime<Utc> { return SystemTime::now().into(); }
    pub fn current_year() -> u16 { return Self::datetime().year() as u16; }
    pub fn current_month() -> u8 { return Self::datetime().month() as u8; }
}

// Idea was to use this for static methods only, to be able to use helper functions everywhere
pub struct SanitizeInput;
impl SanitizeInput
{
    #[inline(always)]
    /// Round to two decimal places and return absolute value
    pub fn f64_to_monetary_f64_abs(float: f64) -> f64 { (float.abs() * 100.0).round() / 100.0 }

    #[inline(always)]
    /// Rounds to two decimal places, sign not changed
    pub fn f64_to_monetary_f64(float: f64) -> f64 { (float * 100.0).round() / 100.0 }

    /// - Can parse xx.x and xx,x
    /// - Ignores everything thats not a digit or `.` `,` `+` `-`
    /// - Does not round
    /// - Only returns absolute value if stated by `return_abs_value: true`
    /// - Empty Strings result in value 0.0
    pub fn string_to_f64(string: &str, return_abs_value: bool) -> Result<f64, ParseFloatError>
    {
        if string.is_empty() {
            return Ok(0.0);
        }
        let mut filtered = string.replace(",", ".");
        filtered.retain(|c| c == '.' || c == '-' || c == '+' || c.is_ascii_digit());

        return match filtered.parse::<f64>() {
            Ok(expenses) => match return_abs_value {
                true => Ok(expenses.abs()),
                false => Ok(expenses),
            },
            Err(e) => Err(e),
        };
    }
}

/// Returns all of the csv cells like this: `Lines<Cells>`
pub fn get_csv_contents_with_header(path: &PathBuf) -> Vec<Vec<String>>
{
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

// ================================================== Private ================================================== //

fn _read_csv_to_string(path: &PathBuf) -> String
{
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
