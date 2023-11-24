use crate::investing::InvestmentMonth;
use serde::Deserialize;
use serde::Serialize;
use tinyrand::Rand;
use tinyrand::Seeded;
use tinyrand::StdRand;
use tinyrand_std::ClockSeed;

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

    pub fn default_months() -> [InvestmentMonth; 12] { return std::array::from_fn(|i| InvestmentMonth::default(i as u8 + 1)); }

    pub fn randomly_filled_months() -> [InvestmentMonth; 12]
    {
        let seed = ClockSeed::default().next_u64();
        let mut rand = StdRand::seed(seed);

        return std::array::from_fn(|i| {
            return InvestmentMonth {
                month_nr: i as u8 + 1,
                amount: rand.next_u16() as f64 / 111.11,
                price_per_unit: rand.next_u16() as f64 / 11.11,
                additional_transactions: rand.next_u16() as f64 / 1111.11,
            };
        });
    }
}
