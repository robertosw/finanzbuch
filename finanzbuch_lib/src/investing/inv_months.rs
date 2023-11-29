use serde::Deserialize;
use serde::Serialize;

use crate::SanitizeInput;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct InvestmentMonth
{
    // transactions done because of the savings plan are not copied here
    month_nr: u8,
    amount: f64,

    /// what was the price per share at the time of adding this data?
    price_per_unit: f64,

    /// (eg. dividends), these are not excluded from amount and price
    additional_transactions: f64,
}
impl InvestmentMonth
{
    /// Will panic if month_nr not 1-12
    pub fn default(month_nr: u8) -> Self
    {
        if month_nr > 12 || month_nr == 0 {
            panic!("month_nr out of bounds");
        }
        Self {
            month_nr,
            amount: 0.0,
            price_per_unit: 0.0,
            additional_transactions: 0.0,
        }
    }

    /// Will panic if month_nr not 1-12
    pub fn new(month_nr: u8, amount: f64, price_per_unit: f64, additional_transactions: f64) -> Self
    {
        if month_nr > 12 || month_nr == 0 {
            panic!("month_nr out of bounds");
        }
        Self {
            month_nr,
            amount: SanitizeInput::f64_to_monetary_f64_abs(amount),
            price_per_unit: SanitizeInput::f64_to_monetary_f64_abs(price_per_unit),
            additional_transactions: SanitizeInput::f64_to_monetary_f64_abs(additional_transactions),
        }
    }

    // ---------- Getters ----------
    pub fn month_nr(&self) -> u8 { self.month_nr }
    pub fn amount(&self) -> f64 { self.amount }
    pub fn price_per_unit(&self) -> f64 { self.price_per_unit }
    pub fn additional_transactions(&self) -> f64 { self.additional_transactions }

    // ---------- Setters ----------
    pub fn set_amount(&mut self, amount: f64) { self.amount = SanitizeInput::f64_to_monetary_f64_abs(amount); }
    pub fn set_price_per_unit(&mut self, price_per_unit: f64) { self.price_per_unit = SanitizeInput::f64_to_monetary_f64_abs(price_per_unit); }
    pub fn set_additional_transactions(&mut self, additional_transactions: f64)
    {
        self.additional_transactions = SanitizeInput::f64_to_monetary_f64_abs(additional_transactions);
    }
}
