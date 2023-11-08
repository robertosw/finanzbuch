use super::{inv_months::InvestmentMonth, inv_year::InvestmentYear, InvestmentVariant, SavingsPlanInterval};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
pub struct DepotElement {
    pub variant: InvestmentVariant,
    pub savings_plan: Vec<SavingsPlanSection>, // TODO this has to be sorted and checked for overlaps

    /// Key is YearNr
    pub history: HashMap<u16, InvestmentYear>,
}
impl DepotElement {
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
