pub mod accounting_month;
pub mod accounting_year;
pub mod recurrence;

use crate::accounting::accounting_year::AccountingYear;

use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

use self::recurrence::RecurringInOut;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Accounting
{
    /// Represents the maximum percentage a user wants to spend of their income (per month/year)
    pub goal: f64,
    pub history: BTreeMap<u16, AccountingYear>,
    // TODO try if this can be transformed to BTreeMap<u16, [AccountingMonth; 12]>
    // Check if the BTreeMap Key can be used instead of Year.year_nr
    pub recurring_income: Vec<RecurringInOut>,
    pub recurring_expenses: Vec<RecurringInOut>,
}
impl Accounting
{
    pub fn default() -> Self
    {
        return Self {
            goal: 1.0,
            history: BTreeMap::new(),
            recurring_income: vec![],
            recurring_expenses: vec![],
        };
    }

    /// - if the year does not already exist, adds it to `DataFile.years` with default values
    /// - changes nothing if the year exists
    /// - returns the year as a mutable reference (`&mut Year`)`
    ///   - this allows function chaining: `DataFile.add_or_get_year().function_on_year()`
    pub fn add_or_get_year(&mut self, year_nr: u16) -> &mut AccountingYear
    {
        if self.history.contains_key(&year_nr) == false {
            self.history.insert(year_nr, AccountingYear::default(year_nr));
        }

        match self.history.get_mut(&year_nr) {
            Some(y) => return y,
            None => panic!("The year {year_nr} was just created but could not be retrieved from HashMap"),
        };
    }
}
