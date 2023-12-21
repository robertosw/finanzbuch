pub mod depot;
pub mod inv_months;
pub mod inv_variant;
pub mod inv_year;
pub mod savings_plan_section;

use serde::Deserialize;
use serde::Serialize;

use self::depot::Depot;
use self::inv_months::InvestmentMonth;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum SavingsPlanInterval
{
    Monthly,
    Annually,
}
impl std::fmt::Display for SavingsPlanInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self {
            SavingsPlanInterval::Monthly => write!(f, "Monthly"),
            SavingsPlanInterval::Annually => write!(f, "Annually"),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Investing
{
    /// User defined growth rates to compare to
    ///
    /// 5 = 5%
    ///
    /// These will be affected by all transactions that are done (planned and additional)
    pub comparisons: Vec<u8>,

    /// It NOT is guaranteed that all `DepotEntry`'s have the same years.
    /// This can be ensured by running `Depot.ensure_uniform_histories()`
    ///
    /// A year will always have all 12 months.
    pub depot: Depot,
}
impl Default for Investing
{
    fn default() -> Self
    {
        return Self {
            comparisons: vec![],
            depot: Depot::new(),
        };
    }
}
impl Investing
{
    pub fn add_comparison(&mut self, growth_rate: u8) { self.comparisons.push(growth_rate); }
}
