use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AccountingMonth {
    month_nr: u8,

    /// always positive
    income: f64, // TODO Sanitize Input (only positive, 2 decimal points)

    /// always positive
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
            income,
            expenses,
            note,
        }
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
    pub fn note_mut(&mut self) -> &mut String {
        &mut self.note
    }

    // Setters
    pub fn set_month_nr(&mut self, month_nr: u8) {
        self.month_nr = month_nr;
    }
    pub fn set_income(&mut self, income: f64) {
        self.income = income;
    }
    pub fn set_expenses(&mut self, expenses: f64) {
        self.expenses = expenses;
    }
    pub fn set_note(&mut self, note: String) {
        self.note = note;
    }

    // Others
    pub fn get_difference(&self) -> f64 {
        self.income - self.expenses
    }

    /// 1.0 = 100%
    pub fn get_percentage_1(&self) -> f64 {
        self.expenses / self.income
    }

    // 100 = 100%
    pub fn get_percentage_100(&self) -> u16 {
        (self.get_percentage_1() * 100.0) as u16
    }
}
