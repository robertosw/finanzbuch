use super::{inv_year::InvestmentYear, savings_plan_section::SavingsPlanSection, InvestmentVariant};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}
