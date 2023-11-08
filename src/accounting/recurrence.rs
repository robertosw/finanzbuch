use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RecurringInOut {
    pub name: String,
    
    /// always positive
    pub quantity: f64,
    // TODO When calculating how much this would be per month, just use the standard year.
    // weeks per year normally: ((52*7 + 1) / 7) weeks (I dont care about leap years, its such a small difference)
    // Example: something that happens every 5 weeks:
    //  52,1428 / 5 = 10,4286
    //  (quantity * 10,4286) / 12 = per month
    pub recurrence: Recurrence,

    /// after how many Days/Weeks/Months/Years does this happen again
    pub interval: u16, // not u8, so that 365 days are possible

    /// how often does this happen per interval
    pub frequency: u16, // not u8, so that 365 days are possible
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Recurrence {
    Day,
    Week,
    Month,
    Year,
}
