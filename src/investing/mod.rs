use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum InvestmentVariant {
    Stock,
    Fund,
    Etf,
    Bond,
    Option,
    Commoditiy,
    Crypto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SavingsPlanInterval {
    Weekly,
    Monthly,
    Annually,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Investing {
    pub comparisons: Vec<u8>,

    /// key is the name
    pub depot: HashMap<String, Investment>,
}
impl Investing {
    pub fn default() -> Self {
        return Self {
            comparisons: vec![],
            depot: HashMap::new(),
        };
    }

    pub fn add_depot_element(&mut self, name: String, investment: Investment) {
        self.depot.insert(name, investment);
    }

    pub fn add_comparison(&mut self, growth_rate: u8) {
        self.comparisons.push(growth_rate);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Investment {
    pub variant: InvestmentVariant,
    pub savings_plan: Vec<SavingsPlanSection>, // TODO this has to be sorted and checked for overlaps

    /// <Year, Months>
    pub history: HashMap<u16, [InvestmentMonth; 12]>,
}
impl Investment {
    pub fn default(variant: InvestmentVariant) -> Self {
        return Self {
            variant,
            savings_plan: vec![],
            history: HashMap::new(),
        };
    }

    pub fn default_months() -> [InvestmentMonth; 12] {
        return std::array::from_fn(|i| InvestmentMonth::default(i as u8 + 1));
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SavingsPlanSection {
    pub start_month: u8,
    pub start_year: u16,
    pub end_month: u8,
    pub end_year: u16,
    pub amount: f64,
    pub interval: SavingsPlanInterval,
}

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
