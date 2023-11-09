use crate::SanitizeInput;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AccountingMonth {
    month_nr: u8,
    income: f64,
    expenses: f64,
    note: String,
}
impl AccountingMonth {
    pub fn default(month: u8) -> Self {
        return Self {
            month_nr: month,
            income: 0.0,
            expenses: 0.0,
            note: String::new(),
        };
    }

    pub fn new(month_nr: u8, income: f64, expenses: f64, note: String) -> AccountingMonth {
        AccountingMonth {
            month_nr,
            income: SanitizeInput::monetary_f64_to_f64(income),
            expenses: SanitizeInput::monetary_f64_to_f64(expenses),
            note,
        }
    }

    pub fn default_months() -> [AccountingMonth; 12] {
        return std::array::from_fn(|i| Self::default(i as u8 + 1));
    }

    // Getter
    pub fn month_nr(&self) -> u8 {
        self.month_nr
    }
    pub fn income(&self) -> f64 {
        self.income
    }
    pub fn expenses(&self) -> f64 {
        self.expenses
    }
    pub fn note(&self) -> &str {
        self.note.as_ref()
    }
    // note doesnt need any content checking, because yaml can store any String
    pub fn note_mut(&mut self) -> &mut String {
        &mut self.note
    }

    // Setters
    // month_nr cannot be changed after the month was created

    /// Absolute value, rounded to two decimal places will be stored
    pub fn set_income(&mut self, income: f64) {
        self.income = SanitizeInput::monetary_f64_to_f64(income);
    }

    /// Absolute value, rounded to two decimal places will be stored
    pub fn set_expenses(&mut self, expenses: f64) {
        self.expenses = SanitizeInput::monetary_f64_to_f64(expenses);
    }
    pub fn set_note(&mut self, note: String) {
        self.note = note;
    }

    // Others
    pub fn difference(&self) -> f64 {
        self.income - self.expenses
    }

    /// 1.0 = 100%
    pub fn percentage_1(&self) -> f64 {
        self.expenses / self.income
    }

    // 100 = 100%
    pub fn percentage_100(&self) -> u16 {
        (self.percentage_1() * 100.0) as u16
    }
}
