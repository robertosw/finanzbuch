pub mod accounting_year;

use crate::accounting::accounting_year::AccountingYear;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Accounting {
    pub goal: f64,
    pub history: HashMap<u16, AccountingYear>,
}
impl Accounting {
    pub fn default() -> Self {
        return Self {
            goal: 1.0,
            history: HashMap::new(),
        };
    }

    /// - if the year does not already exist, adds it to `DataFile.years` with default values
    /// - changes nothing if the year exists
    /// - returns the year as a mutable reference (`&mut Year`)`
    ///   - this allows function chaining: `DataFile.add_or_get_year().function_on_year()`
    pub fn add_or_get_year(&mut self, year_nr: u16) -> &mut AccountingYear {
        if self.history.contains_key(&year_nr) == false {
            self.history.insert(year_nr, AccountingYear::default(year_nr));
        }

        match self.history.get_mut(&year_nr) {
            Some(y) => return y,
            None => panic!("The year {year_nr} was just created but could not be retrieved from HashMap"),
        };
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AccountingMonth {
    pub month_nr: u8,
    pub income: f64,
    pub expenses: f64,
    pub difference: f64,
    pub percentage: f64,
}
impl AccountingMonth {
    pub fn default(month: u8) -> Self {
        return Self {
            month_nr: month,
            income: 0.0,
            expenses: 0.0,
            difference: 0.0,
            percentage: 0.0,
        };
    }

    pub fn default_months() -> [Self; 12] {
        return [
            Self::default(1),
            Self::default(2),
            Self::default(3),
            Self::default(4),
            Self::default(5),
            Self::default(6),
            Self::default(7),
            Self::default(8),
            Self::default(9),
            Self::default(10),
            Self::default(11),
            Self::default(12),
        ];
    }
}
