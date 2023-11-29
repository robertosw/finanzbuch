pub mod depot_entry;
pub mod inv_months;
pub mod inv_variant;
pub mod inv_year;
pub mod savings_plan_section;

use ahash::AHasher;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::hash::Hasher;

use self::depot_entry::DepotEntry;
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

    // This key has to be something that can be used in an `id=""` in html
    /// Key is the name of the `DepotEntry`
    pub depot: HashMap<u64, DepotEntry>,
}
impl Investing
{
    pub fn name_to_key(name: &str) -> u64
    {
        let mut hasher = AHasher::default();
        hasher.write(name.as_bytes());
        return hasher.finish();
    }

    pub fn get_depot_entry(&self, name: &str) -> Option<&DepotEntry> { self.depot.get(&Self::name_to_key(name)) }
    pub fn get_depot_entry_mut(&mut self, name: &str) -> Option<&mut DepotEntry> { self.depot.get_mut(&Self::name_to_key(name)) }

    pub fn default() -> Self
    {
        return Self {
            comparisons: vec![],
            depot: HashMap::new(),
        };
    }

    pub fn add_depot_entry(&mut self, name: &str, depot_entry: DepotEntry) { self.depot.insert(Self::name_to_key(name), depot_entry); }

    pub fn add_comparison(&mut self, growth_rate: u8) { self.comparisons.push(growth_rate); }
}
