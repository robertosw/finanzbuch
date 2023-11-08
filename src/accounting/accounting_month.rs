use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AccountingMonth {
    pub month_nr: u8,

    /// always positive
    pub income: f64,

    /// always positive
    pub expenses: f64,
    pub note: String,
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
