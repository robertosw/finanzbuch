use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InvestmentVariant {
    Share, Fund, Etf
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Investment {
    pub variant: InvestmentVariant,
    pub history: HashMap<u16, InvestmentYear>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InvestmentYear {
    pub sum: f64,
    pub months: [InvestmentMonth; 12],
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InvestmentMonth {
    pub month_nr: u8,
    pub amount: f64,
    pub price_per_unit: f64,
    pub quantity_sold: f64,
}
