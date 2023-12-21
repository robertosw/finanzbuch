use serde::Deserialize;
use serde::Serialize;

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

    /// Returns the maximum value that is possible:
    /// 31. December 65535 (Week 53)
    pub fn new_max() -> Self { return Self::new_risky(u16::MAX, 12, 31); }

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
