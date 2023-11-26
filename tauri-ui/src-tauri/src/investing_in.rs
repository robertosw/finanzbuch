use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentMonthFields
{
    Amount,
    PricePerUnit,
    AdditionalTransactions,
}

#[tauri::command]
pub fn set_investing_month_field(field: InvestmentMonthFields)
{
    println!("field: {:?}", field);
}
