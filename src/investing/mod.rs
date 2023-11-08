pub mod inv_months;
pub mod inv_year;
pub mod depot_element;
pub mod savings_plan_section;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use self::{inv_months::InvestmentMonth, depot_element::DepotElement};

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

    pub fn add_depot_element(&mut self, name: String, investment: DepotElement) {
        self.depot.insert(name, investment);
    }

    pub fn add_comparison(&mut self, growth_rate: u8) {
        self.comparisons.push(growth_rate);
    }
}

