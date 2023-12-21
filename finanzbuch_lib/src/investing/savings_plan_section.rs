use crate::fast_date::FastDate;

use super::SavingsPlanInterval;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SavingsPlanSection
{
    /// inclusive!
    pub start: FastDate,

    /// inclusive!
    pub end: FastDate,

    /// can be negative
    pub amount: f64,
    pub interval: SavingsPlanInterval,
}
