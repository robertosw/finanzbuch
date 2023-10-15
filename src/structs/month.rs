use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Month {
    pub month_nr: u8,
    pub income: f64,
    pub expenses: f64,
    pub difference: f64,
    pub percentage: f64,
}
impl Month {
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
