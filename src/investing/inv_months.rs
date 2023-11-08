use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct InvestmentMonth {
    pub month_nr: u8,
    pub amount: f64,
    pub price_per_unit: f64,

    /// transactions done additionally to the transactions of the savings plan, dividends would go here
    pub additional_transactions: f64,
}
impl InvestmentMonth {
    pub fn default(month_nr: u8) -> Self {
        return Self {
            month_nr,
            amount: 0.0,
            price_per_unit: 0.0,
            additional_transactions: 0.0,
        };
    }
}
