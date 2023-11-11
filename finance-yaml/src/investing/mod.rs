pub mod depot_element;
pub mod inv_months;
pub mod inv_variant;
pub mod inv_year;
pub mod savings_plan_section;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::{depot_element::DepotElement, inv_months::InvestmentMonth};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SavingsPlanInterval {
    Monthly,
    Annually,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Investing {
    /// User defined growth rates to compare to
    ///
    /// 5 = 5%
    ///
    /// These will be affected by all transactions that are done (planned and additional)
    pub comparisons: Vec<u8>,

    /// key is the name
    pub depot: HashMap<String, DepotElement>,
}
impl Investing {
    pub fn default() -> Self {
        return Self {
            comparisons: vec![],
            depot: HashMap::new(),
        };
    }

    pub fn add_depot_element(&mut self, name: String, depot_element: DepotElement) {
        self.depot.insert(name, depot_element);
    }

    pub fn add_comparison(&mut self, growth_rate: u8) {
        self.comparisons.push(growth_rate);
    }
}
