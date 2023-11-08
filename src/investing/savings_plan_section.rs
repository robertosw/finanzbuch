use serde::{Deserialize, Serialize};
use super::SavingsPlanInterval;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SavingsPlanSection {
    
    /// inclusive!
    pub start_month: u8,
    
    /// inclusive!
    pub start_year: u16,
    
    /// inclusive!
    pub end_month: u8,
    
    /// inclusive!
    pub end_year: u16,
    
    /// can be negative
    pub amount: f64,
    pub interval: SavingsPlanInterval,
}
