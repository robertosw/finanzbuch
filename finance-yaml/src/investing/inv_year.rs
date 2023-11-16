use crate::investing::InvestmentMonth;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct InvestmentYear
{
    pub year_nr: u16,
    pub months: [InvestmentMonth; 12],
}
impl InvestmentYear
{
    pub fn default(year_nr: u16) -> Self
    {
        return Self {
            year_nr,
            months: InvestmentYear::default_months(),
        };
    }

    pub fn default_months() -> [InvestmentMonth; 12]
    {
        return std::array::from_fn(|i| InvestmentMonth::default(i as u8 + 1));
    }
}
