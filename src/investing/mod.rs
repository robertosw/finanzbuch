use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InvestmentVariant {
    Stock,
    Fund,
    Etf,
    Bond,
    Option,
    Commoditiy,
    Crypto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Investing {
    pub comparisons: Vec<u8>,

    /// key is a hash of the name (String)
    pub depot: HashMap<u64, Investment>,
}
impl Investing {
    pub fn default() -> Self {
        return Self {
            comparisons: vec![],
            depot: HashMap::new(),
        };
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Investment {
    pub name: String,
    pub variant: InvestmentVariant,
    pub savings_plan: Vec<SavingsPlanInterval>,
    pub history: HashMap<u16, [InvestmentMonth; 12]>,
}

// TODO this probably has to be sorted somehow
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SavingsPlan {
    pub start_month: u8,
    pub start_year: u16,
    pub end_month: u8,
    pub end_year: u16,
    pub amount: f64,
    pub interval: SavingsPlanInterval,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SavingsPlanInterval {
    Weekly,
    Monthly,
    Annually,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InvestmentMonth {
    pub month_nr: u8,
    pub amount: f64,
    pub price_per_unit: f64,
    pub quantity_sold: f64,
}
