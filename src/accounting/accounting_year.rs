use crate::accounting::accounting_month::AccountingMonth;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AccountingYear {
    pub year_nr: u16,
    pub months: [AccountingMonth; 12],  // TODO make this private with get and set methods, so every access is controlled
}
impl AccountingYear {
    pub fn default(year_nr: u16) -> Self {
        return Self {
            year_nr,
            months: AccountingMonth::default_months(),
        };
    }

    /// - If the month (specified by `new_month.month_nr`) contains only default values, these will be overwritten without a note.
    /// - If the month contains values other than defaults, these will also be overwritten without confirmation, but the old values will be printed into the terminal
    pub fn insert_or_overwrite_month(&mut self, new_month: AccountingMonth) {
        let month_nr = new_month.month_nr;
        let month: &mut AccountingMonth = &mut self.months[month_nr as usize - 1];

        if *month != AccountingMonth::default(month.month_nr) {
            // ("{:0>2?}")
            //       2 - width
            //      > -- where to align actual value, > means {fill}{value}, < means {value}{fill}
            //     0 --- with what to fill
            println!("{:0>2?}.{:4?} will be overwritten!", month.month_nr, self.year_nr);
            println!("Old content: {:?}", *month);
        }
        
        *month = new_month;
    }

    pub fn get_sum_income(&self) -> f64 {
        todo!()
    }

    pub fn get_sum_expenses(&self) -> f64 {
        todo!()
    }

    pub fn get_difference(&self) -> f64 {
        todo!()
    }

    pub fn get_percentage(&self) -> f64 {
        todo!()
    }
}
