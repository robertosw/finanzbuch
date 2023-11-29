// these have to be public so that the tests in /tests can use this
pub mod accounting;
pub mod datafile;
pub mod investing;

pub use crate::accounting::accounting_month::AccountingMonth;
pub use crate::accounting::Accounting;
pub use crate::datafile::DataFile;
pub use crate::investing::depot_entry::DepotEntry;

// TODO check what has to be pub

use csv::ReaderBuilder;
use investing::inv_variant::InvestmentVariant;
use investing::inv_year::InvestmentYear;
use investing::Investing;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// use tinyrand::Rand;
// use tinyrand::RandRange;
// use tinyrand::Seeded;
// use tinyrand::StdRand;
// use tinyrand_std::ClockSeed;

const MASK_YEAR: u32 = 0b1111_1111_1111_1111_0000_0000_0000_0000;
const MASK_MONTH: u32 = 0b0000_0000_0000_0000_1111_0000_0000_0000;
const MASK_DAY: u32 = 0b0000_0000_0000_0000_0000_1111_1100_0000;
const MASK_WEEK: u32 = 0b0000_0000_0000_0000_0000_0000_0011_1111;

const DAYS_UNTIL_MONTH_START: [u16; 13] = [
    0,
    0,
    31,
    31 + 28,
    31 + 28 + 31,
    31 + 28 + 31 + 30,
    31 + 28 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30,
    31 + 28 + 31 + 30 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30,
];

/// - `16` bit Year
/// - `4` bit Month
/// - `6` bit Day
/// - `6` bit Week
///     - For simplicity, every year is treated as if the start of the year is also the first day of the first week
///     - Since `(366 days / 7 days) > 52 weeks`, the max value allowed is 53, to indicate that the date is in the 53th week
/// - Expects values to be starting at 1
///
/// The highest possible value is 31. December 65535 (Week 53)
///
/// <pre>
/// 0000 0000 0000 0000 0000 0000 0000 0000
/// |-----------------| |--| |-----||-----|
///        Year         Month  Day    Week
/// </pre>
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    /// January 1st 2000 (Week 1)
    pub fn default() -> Self { Self(0 | 2000 << 16 | 1 << 12 | 1 << 6 | 1) }

    /// This ***panics*** if
    /// - month > 12 or 0
    /// - day > 31 or 0
    pub fn new_risky(year: u16, month: u8, day: u8) -> Self
    {
        // ranges in rust are included..excluded
        if !(1..32).contains(&day) || !(1..13).contains(&month) {
            panic!("This datatype only allows 1-31 days, 1-12 months and 1-53 weeks. Input was day {day}, month {month}");
        }

        let week = Self::_calc_week(month, day);
        return Self(0 | (year as u32) << 16 | (month as u32) << 12 | (day as u32) << 6 | week);
    }

    /// This returns with Err if
    /// - month > 12 or 0
    /// - day > 31 or 0
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, ()>
    {
        // ranges in rust are included..excluded
        if !(1..32).contains(&day) || !(1..13).contains(&month) {
            return Err(());
        }
        let week = Self::_calc_week(month, day);
        return Ok(Self(0 | (year as u32) << 16 | (month as u32) << 12 | (day as u32) << 6 | week));
    }

    /// (Year, Month, Day)
    pub fn date(&self) -> (u16, u8, u8, u8) { (self.year(), self.month(), self.day(), self.week()) }
    pub fn year(&self) -> u16 { (self.0 >> 16) as u16 }
    pub fn month(&self) -> u8 { ((self.0 & MASK_MONTH) >> 12) as u8 }
    pub fn day(&self) -> u8 { ((self.0 & MASK_DAY) >> 6) as u8 }
    pub fn week(&self) -> u8 { (self.0 & MASK_WEEK) as u8 }

    // reset value and assign new
    pub fn set_year(&mut self, year: u16) { self.0 = (self.0 & !MASK_YEAR) | (year as u32) << 16; }

    /// Expects month to be `>= 1 && <= 12`, will return `Err` if thats not the case
    pub fn set_month(&mut self, month: u8) -> Result<(), ()>
    {
        if month > 12 || month == 0 {
            return Err(());
        }
        self.0 &= !MASK_MONTH; // reset value
        self.0 |= (month as u32) << 8;

        self._set_week();
        return Ok(());
    }

    /// Expects day to be `>= 1 && <= 31`, will return `Err` if thats not the case
    pub fn set_day(&mut self, day: u8) -> Result<(), ()>
    {
        if day > 31 || day == 0 {
            return Err(());
        }
        self.0 &= !MASK_DAY; // reset value
        self.0 |= day as u32;

        self._set_week();
        return Ok(());
    }

    /// assumes that day and month have been set before
    fn _set_week(&mut self)
    {
        self.0 &= !MASK_WEEK; // reset week value;
        self.0 |= Self::_calc_week(self.month(), self.day());
    }

    fn _calc_week(month: u8, day: u8) -> u32
    {
        let day_in_year = DAYS_UNTIL_MONTH_START[month as usize] + day as u16;
        return ((day_in_year as f32 / 7.0).ceil()) as u32;
    }
}

// Idea was to use this for static methods only, to be able to use helper functions everywhere
pub struct SanitizeInput;
impl SanitizeInput
{
    /// Round to two decimal places and return absolute value
    pub fn f64_to_monetary_f64(float: f64) -> f64 { (float.abs() * 100.0).round() / 100.0 }

    /// - Can parse xx.x and xx,x
    /// - Ignores everything thats not a digit or `.` `,`
    /// - Rounds to two decimal places
    /// - Returns absolute value
    ///
    /// - Error String contains descriptive message
    pub fn string_to_monetary_f64(string: &String) -> Result<f64, String>
    {
        let mut filtered = string.clone().replace(",", ".");
        filtered.retain(|c| c == '.' || c.is_ascii_digit());

        return match filtered.parse::<f64>() {
            Ok(expenses) => Ok(Self::f64_to_monetary_f64(expenses)),
            Err(e) => Err(format!("{:?} could not be parsed as a f64: {}", filtered, e)),
        };
    }
}

pub fn generate_depot_entry()
{
    let mut datafile = DataFile::read();
    const NAME: &str = "name 123";
    let depot_entry_key = Investing::name_to_key(NAME);

    datafile
        .investing
        .depot
        .insert(depot_entry_key, DepotEntry::default(NAME, InvestmentVariant::Stock));

    match datafile.investing.get_depot_entry_mut(&String::from(NAME)) {
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
