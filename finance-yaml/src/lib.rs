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

// use tinyrand::Rand;
// use tinyrand::RandRange;
// use tinyrand::Seeded;
// use tinyrand::StdRand;
// use tinyrand_std::ClockSeed;

// 12 (for Months) would fit into 4 bits, giving 20 bits to the year, but thats not really practical, because year would have to be returned as u32, not u16
/// ```
/// 0000 0000 0000 0000 0000 0000 0000 0000
/// |------Year-------| |-Month-| |--Day--|
/// ```
/// Expects to be used with "normal" values: January = 1, December = 12, First day in month = 1
pub struct FastDate(u32);
impl PartialEq for FastDate
{
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}
impl Eq for FastDate {}
impl PartialOrd for FastDate
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { self.0.partial_cmp(&other.0) }
}
impl Ord for FastDate
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.0.cmp(&other.0) }
}
impl FastDate
{
    /// This returns with Err if
    /// - month > 12 or 0
    /// - day > 31 or 0
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, ()>
    {
        if month > 12 || day > 31 || month == 0 || day == 0 {
            return Err(());
        }
        return Ok(Self(0 | (year as u32) << 16 | (month as u32) << 8 | (day as u32)));
    }

    /// (Year, Month, Day)
    pub fn date(&self) -> (u16, u8, u8) { (self.year(), self.month(), self.day()) }
    pub fn year(&self) -> u16 { (self.0 >> 16) as u16 }
    pub fn month(&self) -> u8 { (self.0 >> 8) as u8 }
    pub fn day(&self) -> u8 { self.0 as u8 }
    pub fn raw(&self) -> u32 { self.0 }

    // reset value and assign new
    pub fn set_year(&mut self, year: u16) { self.0 = (self.0 & 0b0000_0000_0000_0000_1111_1111_1111_1111) | (year as u32) << 16; }

    /// Expects month to be `>= 1 && <= 12`, will return `Err` if thats not the case
    pub fn set_month(&mut self, month: u8) -> Result<(), ()>
    {
        if month > 12 || month == 0 {
            return Err(());
        }
        self.0 &= 0b1111_1111_1111_1111_0000_0000_1111_1111; // reset value
        self.0 = self.0 | (month as u32) << 8;
        return Ok(());
    }

    /// Expects day to be `>= 1 && <= 31`, will return `Err` if thats not the case
    pub fn set_day(&mut self, day: u8) -> Result<(), ()>
    {
        if day > 31 || day == 0 {
            return Err(());
        }
        self.0 &= 0b1111_1111_1111_1111_1111_1111_0000_0000; // reset value
        self.0 = self.0 | (day as u32);
        return Ok(());
    }
}

pub struct SanitizeInput;
impl SanitizeInput
{
    /// Round to two decimal places and return absolute value
    pub fn monetary_f64_to_f64(float: f64) -> f64 { (float.abs() * 100.0).round() / 100.0 }

    /// - Can parse xx.x and xx,x
    /// - Ignores everything thats not a digit or `.` `,`
    /// - Rounds to two decimal places
    /// - Returns absolute value
    ///
    /// - Error String contains descriptive message
    pub fn monetary_string_to_f64(string: &String) -> Result<f64, String>
    {
        let mut filtered = string.clone().replace(",", ".");
        filtered.retain(|c| c == '.' || c.is_ascii_digit());

        return match filtered.parse::<f64>() {
            Ok(expenses) => Ok(Self::monetary_f64_to_f64(expenses)),
            Err(e) => Err(format!("{:?} could not be parsed as a f64: {}", filtered, e)),
        };
    }
}

pub fn generate_depot_entry()
{
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
