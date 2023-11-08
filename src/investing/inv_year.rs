use crate::investing::DepotElement;
use crate::investing::InvestmentMonth;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct InvestmentYear {
    pub year_nr: u16,
    pub months: [InvestmentMonth; 12],
}
impl InvestmentYear {
    pub fn default(year_nr: u16) -> Self {
        return Self {
            year_nr,
            months: DepotElement::default_months(),
        };
    }
}
