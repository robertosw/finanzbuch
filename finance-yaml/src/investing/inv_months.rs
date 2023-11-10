use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct InvestmentMonth {
    // transactions done because of the savings plan are not copied here
    pub month_nr: u8,
    pub amount: f64,

    /// what was the price per share at the time of adding this data?
    pub price_per_unit: f64,

    /// (eg. dividends), these are not excluded from amount and price
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
